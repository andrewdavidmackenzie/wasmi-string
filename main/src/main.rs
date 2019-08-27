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
    Allocate memory for a new null-terminated array of bytes inside the wasm module.
*/
fn alloc_byte_array(instance: &ModuleRef, memory: &MemoryRef, bytes: &[u8]) -> u32 {
    let result = instance
        .invoke_export("alloc", &[RuntimeValue::I32((bytes.len() + 1) as i32)],
                       &mut NopExternals);

    match result.unwrap().unwrap() {
        RuntimeValue::I32(pointer) => {
            let len = bytes.len();
            for i in 0..len {
                memory.set_value((pointer + i as i32) as u32, bytes[i]).unwrap();
            }
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
    Get the null terminated array of bytes from wasm module memory at `offset`

    Since the wasm module can only return one result (offset) we have to go through the array of bytes
    until we find the null termination and hence calculate the length
*/
fn get_bytes(memory: &MemoryRef, mut offset: u32) -> Vec<u8> {
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
    bytes
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

    let input_data = "What is the meaning of life?\0";

    // Allocate a string for the input data inside wasm module
    let input_data_wasm_ptr = alloc_byte_array(&instance, &memory, input_data.as_bytes());

    // Run the `run` function on the input_data and get a result back
    let result = instance
        .invoke_export("run", &[RuntimeValue::I32(input_data_wasm_ptr as i32)], &mut NopExternals);

    match result {
        Ok(e) => {
            match e.unwrap() {
                RuntimeValue::I32(result_offset) => {
                    let result = get_bytes(&memory, result_offset as u32);
                    let result_str = String::from_utf8(result).unwrap();
                    dealloc(&instance, result_offset as u32);
                    println!("Result is `{}`", result_str);
                }
                _ => println!("Not implemented yet")
            }
        }
        Err(e) => match e {
            _ => println!("Not implemented yet")
        }
    }
}
