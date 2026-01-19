use crate::sys::*;
use std::ptr::NonNull;

pub struct Interactor {
    ptr: NonNull<f3d_interactor_t>,
}

impl Interactor {
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_interactor_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_interactor_t"),
        }
    }

    pub fn start(&self, framerate: f64) {
        unsafe {
            f3d_interactor_start(self.ptr.as_ptr(), framerate);
        }
    }
}

