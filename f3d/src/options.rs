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
}

