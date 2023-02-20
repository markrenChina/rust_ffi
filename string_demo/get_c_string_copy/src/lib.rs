extern crate libc;

use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn print_c_string(s: *const c_char){
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap().to_owned();
    println!("printed from rust: {:#?}",r_str);
}
