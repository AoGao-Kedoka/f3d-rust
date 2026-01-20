use crate::sys::*;
use crate::interactor::Interactor;
use crate::options::Options;
use crate::scene::Scene;
use crate::window::Window;
use std::ptr::NonNull;

pub struct Engine {
    ptr: NonNull<f3d_engine_t>,
}

impl Engine {
    pub fn new() -> Self {
        unsafe {
            let ptr = f3d_engine_create(0);
            Self {
                ptr: NonNull::new(ptr).expect("f3d_engine_create returned null"),
            }
        }
    }

    pub fn auto_load_plugins(&self) {
        unsafe {
            f3d_engine_autoload_plugins();
        }
    }

    pub fn options(&self) -> Options {
        unsafe { Options::from_raw(f3d_engine_get_options(self.ptr.as_ptr())) }
    }

    pub fn scene(&self) -> Scene {
        unsafe { Scene::from_raw(f3d_engine_get_scene(self.ptr.as_ptr())) }
    }

    pub fn interactor(&self) -> Interactor {
        unsafe { Interactor::from_raw(f3d_engine_get_interactor(self.ptr.as_ptr())) }
    }

    pub fn window(&self) -> Window {
        unsafe {
            let ptr = f3d_engine_get_window(self.ptr.as_ptr());
            Window::from_raw(ptr)
        }
    }
    

    // TODO: Other backends
    // TODO: Plugins
    // TODO: lib and reader info
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            f3d_engine_delete(self.ptr.as_ptr());
        }
    }
}
