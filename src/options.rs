use crate::sys::*;
use std::ffi::CString;
use std::ptr::NonNull;

pub struct Options {
    ptr: NonNull<f3d_options_t>,
}

impl Options {
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_options_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_options_t"),
        }
    }

    pub fn set_bool(&self, key: &str, value: bool) {
        let ckey = CString::new(key).unwrap();
        unsafe {
            f3d_options_set_as_bool(self.ptr.as_ptr(), ckey.as_ptr(), value as i32);
        }
    }

    pub fn set_int(&self, key: &str, value: i32) {
        let ckey = CString::new(key).unwrap();
        unsafe {
            f3d_options_set_as_int(self.ptr.as_ptr(), ckey.as_ptr(), value);
        }
    }

    pub fn set_double(&self, key: &str, value: f64) {
        let ckey = CString::new(key).unwrap();
        unsafe {
            f3d_options_set_as_double(self.ptr.as_ptr(), ckey.as_ptr(), value);
        }
    }

    pub fn set_string(&self, key: &str, value: &str) {
        let ckey = CString::new(key).unwrap();
        let cvalue = CString::new(value).unwrap();
        unsafe {
            f3d_options_set_as_string(self.ptr.as_ptr(), ckey.as_ptr(), cvalue.as_ptr());
        }
    }

    pub fn set_double_array(&self, key: &str, values: &[f64]) {
        let ckey = CString::new(key).unwrap();
        unsafe {
            f3d_options_set_as_double_vector(
                self.ptr.as_ptr(),
                ckey.as_ptr(),
                values.as_ptr() as *mut f64,
                values.len() as usize,
            );
        }
    }

    pub fn set_int_array(&self, key: &str, values: &[i32]) {
        let ckey = CString::new(key).unwrap();
        unsafe {
            f3d_options_set_as_int_vector(
                self.ptr.as_ptr(),
                ckey.as_ptr(),
                values.as_ptr() as *mut i32,
                values.len() as usize,
            );
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        let ckey = CString::new(key).unwrap();
        unsafe { f3d_options_get_as_bool(self.ptr.as_ptr(), ckey.as_ptr()) != 0 }
    }

    pub fn get_int(&self, key: &str) -> i32 {
        let ckey = CString::new(key).unwrap();
        unsafe { f3d_options_get_as_int(self.ptr.as_ptr(), ckey.as_ptr()) }
    }

    pub fn get_double(&self, key: &str) -> f64 {
        let ckey = CString::new(key).unwrap();
        unsafe { f3d_options_get_as_double(self.ptr.as_ptr(), ckey.as_ptr()) }
    }

    pub fn get_string(&self, key: &str) -> String {
        let ckey = CString::new(key).unwrap();
        unsafe {
            let cstr = f3d_options_get_as_string(self.ptr.as_ptr(), ckey.as_ptr()) as *mut i8;
            let result = CString::from_raw(cstr).to_string_lossy().into_owned();
            f3d_options_free_string(cstr);
            result
        }
    }

    pub fn get_string_representation(&self, key: &str) -> String {
        let ckey = CString::new(key).unwrap();
        unsafe {
            let cstr = f3d_options_get_as_string_representation(self.ptr.as_ptr(), ckey.as_ptr())
                as *mut i8;
            let result = CString::from_raw(cstr).into_string().unwrap();
            f3d_options_free_string(cstr);
            result
        }
    }

    pub fn get_double_array(&self, key: &str) -> Vec<f64> {
        let ckey = CString::new(key).unwrap();
        let mut count: usize = 0;
        unsafe {
            f3d_options_get_as_double_vector(
                self.ptr.as_ptr(),
                ckey.as_ptr(),
                std::ptr::null_mut(),
                &mut count as *mut usize,
            );
            let mut values = vec![0.0f64; count];
            if count > 0 {
                f3d_options_get_as_double_vector(
                    self.ptr.as_ptr(),
                    ckey.as_ptr(),
                    values.as_mut_ptr(),
                    &mut count as *mut usize,
                );
            }
            values
        }
    }

    pub fn get_int_array(&self, key: &str) -> Vec<i32> {
        let ckey = CString::new(key).unwrap();
        let mut count: usize = 0;
        unsafe {
            f3d_options_get_as_int_vector(
                self.ptr.as_ptr(),
                ckey.as_ptr(),
                std::ptr::null_mut(),
                &mut count as *mut usize,
            );
            let mut values = vec![0i32; count];
            if count > 0 {
                f3d_options_get_as_int_vector(
                    self.ptr.as_ptr(),
                    ckey.as_ptr(),
                    values.as_mut_ptr(),
                    &mut count as *mut usize,
                );
            }
            values
        }
    }
}
