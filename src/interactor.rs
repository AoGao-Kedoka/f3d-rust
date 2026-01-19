use crate::sys::*;
use std::ffi::CString;
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

    //TODO: binding
    //TODO: animation toggle

    pub fn enable_camera_movement(&self) {
        unsafe {
            f3d_interactor_enable_camera_movement(self.ptr.as_ptr());
        }
    }

    pub fn disable_camera_movement(&self) {
        unsafe {
            f3d_interactor_disable_camera_movement(self.ptr.as_ptr());
        }
    }

    //TODO: event trigger

    pub fn play_interaction(&self, file_path: &str, delta_time: f64) {
        let cfile_path = CString::new(file_path).unwrap();
        unsafe {
            f3d_interactor_play_interaction(self.ptr.as_ptr(), cfile_path.as_ptr(), delta_time);
        }
    }

    pub fn record_interaction(&self, file_path: &str) {
        let cfile_path = CString::new(file_path).unwrap();
        unsafe {
            f3d_interactor_record_interaction(self.ptr.as_ptr(), cfile_path.as_ptr());
        }
    }

    pub fn start(&self, framerate: f64) {
        unsafe {
            f3d_interactor_start(self.ptr.as_ptr(), framerate);
        }
    }

    //TODO: Start with callback

    pub fn stop(&self) {
        unsafe {
            f3d_interactor_stop(self.ptr.as_ptr());
        }
    }

    pub fn request_render(&self) {
        unsafe {
            f3d_interactor_request_render(self.ptr.as_ptr());
        }
    }

    pub fn request_stop(&self) {
        unsafe {
            f3d_interactor_request_stop(self.ptr.as_ptr());
        }
    }
}
