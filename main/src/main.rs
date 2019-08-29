extern crate wasmi;

use std::{env, str};
use std::fs::File;
use std::io::prelude::*;

use wasmi::{
    ImportsBuilder, MemoryRef, Module,
    ModuleInstance, ModuleRef,
    NopExternals, RuntimeValue,
};

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
    Allocate memory for a new null-terminated array of bytes inside the wasm module and copy
    the array of bytes into it
*/
fn send_byte_array(instance: &ModuleRef, memory: &MemoryRef, bytes: &[u8]) -> u32 {
    let result = instance
        .invoke_export("alloc", &[RuntimeValue::I32((bytes.len()) as i32)],
                       &mut NopExternals);

    match result.unwrap().unwrap() {
        RuntimeValue::I32(pointer) => {
            memory.set(pointer as u32, bytes).unwrap();
            pointer as u32
        }
        _ => 0 as u32
    }
}

fn ask(module_ref: &ModuleRef, memory: &MemoryRef, question: &str) {
    let input_data_length = question.len();
    println!("Question: '{}'", question);

    // Allocate a string for the input data inside wasm module
    let wasm_data_ptr = send_byte_array(&module_ref, &memory, question.as_bytes());

    // Run the `run` function on the input_data and get a result back
    let result = module_ref
        .invoke_export("run", &[RuntimeValue::I32(wasm_data_ptr as i32),
            RuntimeValue::I32(input_data_length as i32)],
                       &mut NopExternals);

    match result {
        Ok(value) => {
            match value.unwrap() {
                RuntimeValue::I32(result_length) => {
                    let result = memory.get(wasm_data_ptr, result_length as usize).unwrap();
                    let result_str = String::from_utf8(result).unwrap();
                    println!("Answer : '{}'", result_str);
                }
                _ => println!("Not implemented yet")
            }
        }
        Err(e) => println!("{:?}", e)
    }
}

fn main() {
    let path = env::current_dir().unwrap();
    let module = load_from_file(format!("{}/wasm/target/wasm32-unknown-unknown/debug/test.wasm",
                                        path.display()).as_str());

    let module_ref = ModuleInstance::new(&module, &ImportsBuilder::default())
        .unwrap()
        .assert_no_start();

    let memory_ref = module_ref.export_by_name("memory")
        .expect("`memory` export not found")
        .as_memory()
        .expect("export name `memory` is not of memory type")
        .to_owned();

    ask(&module_ref, &memory_ref, "What is the meaning of life?");
    ask(&module_ref, &memory_ref, "What is life?");
}
