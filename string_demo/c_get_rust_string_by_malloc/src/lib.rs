use libc::{c_char, c_void};
use std::ffi::CString;
use std::usize;


const STRING : &str= "hello markrenChina";

type Allocator = unsafe extern fn(usize) -> *mut c_void;

#[no_mangle]
pub unsafe extern fn get_string_with_allocator(allocator: Allocator) -> *mut c_char {
    let ptr: *mut c_char = allocator(STRING.as_bytes().len()).cast();
    copy_string(ptr);
    ptr
}

///这里展示与缓冲区方式不一样的函数copy api
#[no_mangle]
pub unsafe extern fn copy_string(ptr: *mut c_char) {
    let bytes = STRING.as_bytes();
    let len = bytes.len();
    std::ptr::copy(STRING.as_bytes().as_ptr().cast(), ptr, len);
    std::ptr::write(ptr.offset(len as isize) as *mut u8, 0u8);
}

#[no_mangle]
pub unsafe extern fn free_string(ptr: *const c_char) {
    let _ = CString::from_raw(ptr as *mut _);
}