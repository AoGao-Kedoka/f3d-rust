use crate::sys::*;
use std::ffi::CString;
use std::ptr::NonNull;

pub struct Window {
    ptr: NonNull<f3d_window_t>,
}

impl Window {
    /// Internal constructor from raw pointer
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_window_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_window_t"),
        }
    }

    /// Get the window type
    // pub fn window_type(&self) -> WindowType {
    //     unsafe {
    //         f3d_window_get_type(self.ptr.as_ptr()).into()
    //     }
    // }

    /// Returns true if the window is offscreen
    pub fn is_offscreen(&self) -> bool {
        unsafe {
            f3d_window_is_offscreen(self.ptr.as_ptr()) != 0
        }
    }

    /// Get the camera associated with the window
    // pub fn camera(&self) -> Camera {
    //     unsafe {
    //         Camera::from_raw(f3d_window_get_camera(self.ptr.as_ptr()))
    //     }
    // }

    /// Render the window to the screen
    pub fn render(&self) -> bool {
        unsafe {
            f3d_window_render(self.ptr.as_ptr()) != 0
        }
    }

    /// Render the window to an image
    // pub fn render_to_image(&self, no_background: bool) -> Option<Image> {
    //     unsafe {
    //         let img = f3d_window_render_to_image(
    //             self.ptr.as_ptr(),
    //             no_background as i32,
    //         );
    //         NonNull::new(img).map(|p| Image::from_raw(p.as_ptr()))
    //     }
    // }

    /// Set the window size in pixels
    pub fn set_size(&self, width: i32, height: i32) {
        unsafe {
            f3d_window_set_size(self.ptr.as_ptr(), width, height);
        }
    }

    /// Get window width
    pub fn width(&self) -> i32 {
        unsafe {
            f3d_window_get_width(self.ptr.as_ptr())
        }
    }

    /// Get window height
    pub fn height(&self) -> i32 {
        unsafe {
            f3d_window_get_height(self.ptr.as_ptr())
        }
    }

    /// Set window position
    pub fn set_position(&self, x: i32, y: i32) {
        unsafe {
            f3d_window_set_position(self.ptr.as_ptr(), x, y);
        }
    }

    /// Set window icon (raw RGBA bytes)
    pub fn set_icon(&self, icon: &[u8]) {
        unsafe {
            f3d_window_set_icon(
                self.ptr.as_ptr(),
                icon.as_ptr(),
                icon.len(),
            );
        }
    }

    /// Set window title
    pub fn set_title(&self, title: &str) {
        let ctitle = CString::new(title).unwrap();
        unsafe {
            f3d_window_set_window_name(self.ptr.as_ptr(), ctitle.as_ptr());
        }
    }

    // pub fn world_from_display(&self, display: [f64; 3]) -> [f64; 3] {
    //     let mut world = [0.0; 3];
    //     unsafe {
    //         f3d_window_get_world_from_display(
    //             self.ptr.as_ptr(),
    //             display,
    //             world.as_mut_ptr(),
    //         );
    //     }
    //     world
    // }

    // pub fn display_from_world(&self, world: [f64; 3]) -> [f64; 3] {
    //     let mut display = [0.0; 3];
    //     unsafe {
    //         f3d_window_get_display_from_world(
    //             self.ptr.as_ptr(),
    //             world,
    //             display.as_mut_ptr(),
    //         );
    //     }
    //     display
    // }
}

