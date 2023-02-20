use libc::{c_char};

const STRING : &str= "hello markrenChina";


#[no_mangle]
pub unsafe extern fn get_string_with_malloc() -> *mut c_char{
    let ptr: *mut c_char = libc::malloc(STRING.as_bytes().len()).cast();
    copy_string(ptr);
    ptr
}

#[no_mangle]
pub unsafe extern fn copy_string(ptr: *mut c_char) {
    let bytes = STRING.as_bytes();
    let len = bytes.len();
    std::ptr::copy(STRING.as_bytes().as_ptr().cast(), ptr, len);
    std::ptr::write(ptr.offset(len as isize) as *mut u8, 0u8);
}