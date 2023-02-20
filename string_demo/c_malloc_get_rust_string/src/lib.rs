use libc::{c_char, c_int };
use std::{slice, ptr ,usize};

const HELLO: &str = "hello worls";

#[no_mangle]
pub unsafe extern "C" fn get_string_api(buffer: *mut c_char, length: *mut usize) -> c_int {
    if buffer.is_null() {
        if length.is_null(){
            return -1;
        }else {
            *length = HELLO.as_bytes().len() + 1;
            return 0;
        }
    }

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, *length);

    if HELLO.len() >= buffer.len() {
        return -1;
    }

    ptr::copy_nonoverlapping(
            HELLO.as_ptr(),
            buffer.as_mut_ptr(),
            HELLO.len(),
            );

    buffer[HELLO.len()] = 0;

    HELLO.len() as c_int
}