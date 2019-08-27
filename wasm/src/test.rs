use std::mem;
use std::os::raw::{c_char, c_void};
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

/*
    Deallocate a chunk of memory in wasm module
*/
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern fn digest(data: *mut c_char) -> *mut c_char {
    let result = "42\0";
    unsafe {
        copy(result.as_ptr(), data as *mut u8, result.len());
    }
    data
}
