use std::ffi::{CStr, CString};
use std::option::Option;
use std::os::raw::c_char;
use std::str::Utf8Error;

extern "C" {
    fn getenv(name: *const c_char) -> *const c_char;
}

fn getenv_sys(name: &str) -> Result<Option<&str>, Utf8Error> {
    let key = CString::new(name).expect("CString::new failed");
    unsafe {
        let val = getenv(key.as_ptr());
        if val.is_null() {
            Ok(None)
        } else {
            let c_str = CStr::from_ptr(val);
            match c_str.to_str() {
                Ok(str) => Ok(Some(str)),
                Err(err) => Err(err),
            }
        }
    }
}

fn main() {
    match getenv_sys("PATH") {
        Ok(maybe_value) => match maybe_value {
            Some(value) => println!("Found: {value}"),
            None => println!("Not found"),
        },
        Err(err) => println!("String conversion error: {err}"),
    }
    match getenv_sys("FOO") {
        Ok(maybe_value) => match maybe_value {
            Some(value) => println!("Found: {value}"),
            None => println!("Not found"),
        },
        Err(err) => println!("String conversion error: {err}"),
    }
}
