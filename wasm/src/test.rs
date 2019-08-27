use std::mem;
use std::os::raw::c_void;
use std::ptr::copy;

/*
    Allocate a chunk of memory of `size` bytes in wasm module
*/
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern fn run(data_ptr: *mut c_void, input_data_length: u32) -> u32 {
    let question_vec: Vec<u8> = unsafe {
        Vec::from_raw_parts(data_ptr as *mut u8, input_data_length as usize,
                            input_data_length as usize)
    };

    let question: &str = &String::from_utf8(question_vec).unwrap();

    let answer = match question {
        "What is the meaning of life?" => "42",
        _ => "I don't know"
    };

    unsafe {
        copy(answer.as_ptr(), data_ptr as *mut u8, answer.len());
    }
    answer.len() as u32
}
