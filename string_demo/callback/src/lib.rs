use std::ffi::*;

type Callback = unsafe extern fn(*const c_char);

#[no_mangle]
pub unsafe extern "C" fn rust_call_c(callback: Callback ){
    let c_string = CString::new("su").expect("CString new failed");
    callback(c_string.as_ptr())
}