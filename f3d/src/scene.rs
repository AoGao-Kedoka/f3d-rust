use crate::sys::*;
use std::ffi::CString;
use std::ptr::NonNull;

pub struct Scene {
    ptr: NonNull<f3d_scene_t>,
}

impl Scene {
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_scene_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_scene_t"),
        }
    }

    pub fn add(&self, path: &str) -> i32 {
        let cpath = CString::new(path).expect("path contains interior null bytes");
        unsafe {
            f3d_scene_add(self.ptr.as_ptr(), cpath.as_ptr())
        }
    }
}

