use num_bigint::{BigInt};
use std::os::raw::{c_int};
use std::ffi::{c_char, CString};
use std::slice;

#[no_mangle]
pub extern "C" fn sum_of_squares(arr: *const c_int, len: c_int) -> *mut c_char {
    let mut sum = BigInt::from(0);
    unsafe {
        let numbers = slice::from_raw_parts(arr, len as usize);
        for &val in numbers {
            let big_val = BigInt::from(val);
            sum += &big_val * &big_val;
        }
    }

    let sum_str = sum.to_string();
    let c_str = CString::new(sum_str).unwrap();
    let ptr = c_str.into_raw();
    ptr
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}