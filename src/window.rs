use crate::sys::*;
use crate::camera::Camera;
use crate::types::F3DPoint3;
use std::ffi::CString;
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    None = 0,
    External,
    GLX,
    WGL,
    Cocoa,
    EGL,
    OSMesa,
    Wasm,
    Unknown,
}

pub struct Window {
    ptr: NonNull<f3d_window_t>,
}

impl Window {
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_window_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_window_t"),
        }
    }

    pub fn window_type(&self) -> WindowType {
        unsafe { std::mem::transmute(f3d_window_get_type(self.ptr.as_ptr())) }
    }

    pub fn is_offscreen(&self) -> bool {
        unsafe { f3d_window_is_offscreen(self.ptr.as_ptr()) != 0 }
    }

    pub fn camera(&self) -> Camera {
        unsafe {
            Camera::from_raw(f3d_window_get_camera(self.ptr.as_ptr()))
        }
    }

    pub fn render(&self) -> bool {
        unsafe { f3d_window_render(self.ptr.as_ptr()) != 0 }
    }

    //TODO: render to image

    pub fn set_size(&self, width: i32, height: i32) {
        unsafe {
            f3d_window_set_size(self.ptr.as_ptr(), width, height);
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { f3d_window_get_width(self.ptr.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { f3d_window_get_height(self.ptr.as_ptr()) }
    }

    pub fn set_position(&self, x: i32, y: i32) {
        unsafe {
            f3d_window_set_position(self.ptr.as_ptr(), x, y);
        }
    }

    pub fn set_icon(&self, icon: &[u8]) {
        unsafe {
            f3d_window_set_icon(self.ptr.as_ptr(), icon.as_ptr(), icon.len());
        }
    }

    pub fn set_title(&self, title: &str) {
        let ctitle = CString::new(title).unwrap();
        unsafe {
            f3d_window_set_window_name(self.ptr.as_ptr(), ctitle.as_ptr());
        }
    }

    pub fn world_from_display(&self, display:F3DPoint3) -> F3DPoint3 {
        let mut world = F3DPoint3::default();
        unsafe {
            f3d_window_get_world_from_display(
                self.ptr.as_ptr(),
                display.as_ptr() as *mut f64,
                world.as_mut_ptr(),
            );
        }
        world
    }

    pub fn display_from_world(&self, world:F3DPoint3) -> F3DPoint3 {
        let mut display = F3DPoint3::default();
        unsafe {
            f3d_window_get_display_from_world(
                self.ptr.as_ptr(),
                world.as_ptr() as *mut f64,
                display.as_mut_ptr(),
            );
        }
        display
    }
}
