extern crate wasmi;

use std::{env, str};
use std::fs::File;
use std::io::prelude::*;

use wasmi::{
    ImportsBuilder, MemoryRef, Module,
    ModuleInstance, ModuleRef,
    NopExternals, RuntimeValue,
};

use wasm_env::WasmEnv;

mod wasm_env;

/*
    Load a wasm module from a .wasm file using WASMI
*/
fn load_from_file(filename: &str) -> Module {
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    Module::from_buffer(buf).unwrap()
}

/*
    Allocate memory for a new String inside the wasm module.
*/
fn new_string(instance: &ModuleRef, memory: &MemoryRef, s: &str) -> u32 {
    let result = instance
        .invoke_export("alloc", &[RuntimeValue::I32((s.len() + 1) as i32)],
                       &mut NopExternals);

    match result.unwrap().unwrap() {
        RuntimeValue::I32(pointer) => {
            let bytes = s.as_bytes();
            let len = bytes.len();
            for i in 0..len {
                memory.set_value((pointer + i as i32) as u32, bytes[i]).unwrap();
            }
            memory.set_value((pointer + len as i32) as u32, 0u8).unwrap(); // null terminate
            pointer as u32
        }
        _ => 0 as u32
    }
}

/*
    Dealloc a block of memory allocated inside wasm module
*/
fn dealloc(instance: &ModuleRef, offset: u32) {
    instance.invoke_export("dealloc",
                       &[RuntimeValue::I32(offset as i32)],
                       &mut NopExternals).unwrap();
}

/*
    Get the null terminated string from wasm module memory at `offset` and free that memory
    on the wasm side by calling `dealloc`

    Sicne the wasm module can only return one result (offset) we have to go through the string
    there until we find the null termination and hence calculate the length
*/
fn get_string(memory: &MemoryRef, mut offset: u32) -> String {
    let mut bytes: Vec<u8> = vec![];
    loop {
        let mut buf = [0u8; 1];
        match memory.get_into(offset, &mut buf) {
            Ok(_) => {
                if buf[0] != 0 {
                    bytes.push(buf[0]);
                    offset = offset + 1
                } else {
                    break;
                }
            }
            Err(_) => {}
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn main() {
    let path = env::current_dir().unwrap();
    let module = load_from_file(format!("{}/wasm/target/wasm32-unknown-unknown/debug/test.wasm",
                                        path.display()).as_str());
    let env = WasmEnv::new();

    let instance = ModuleInstance::new(
        &module, &ImportsBuilder::new().with_resolver("env", &env))
        .expect("Failed to instantiate module")
        .run_start(&mut NopExternals)
        .expect("Failed to run start function in module");

    let memory = instance.export_by_name("memory")
        .expect("`memory` export not found")
        .as_memory()
        .expect("export name `memory` is not of memory type")
        .to_owned();

    let s2 = "Löwe 老虎 Léopard";

    let p = new_string(&instance, &memory, s2);
    let result = instance
        .invoke_export("run", &[RuntimeValue::I32(p as i32)], &mut NopExternals);

    match result {
        Ok(e) => {
            match e.unwrap() {
                RuntimeValue::I32(result_offset) => {
                    let result = get_string(&memory, result_offset as u32);
                    dealloc(&instance, result_offset as u32);
                    println!("Result is `{}`", result);
                }
                _ => println!("Not implemented yet")
            }
        }
        Err(e) => match e {
            _ => println!("Not implemented yet")
        }
    }
}
