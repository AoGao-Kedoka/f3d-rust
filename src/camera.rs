use crate::sys::*;
use crate::types;
use std::ptr::NonNull;

pub struct CameraState {
    position: types::F3DPoint3,
    focal_point: types::F3DPoint3,
    view_up: types::F3DVector3,
    view_angle: f64,
}

pub struct Camera {
    ptr: NonNull<f3d_camera_t>,
}

impl Camera {
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_camera_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_camera_t"),
        }
    }

    pub fn set_position(&self, pos: types::F3DPoint3) {
        unsafe {
            f3d_camera_set_position(self.ptr.as_ptr(), pos.as_ptr() as *mut f64);
        }
    }

    pub fn get_position(&self) -> types::F3DPoint3 {
        let mut pos = types::F3DPoint3::default();
        unsafe {
            f3d_camera_get_position(self.ptr.as_ptr(), pos.as_mut_ptr());
        }
        pos
    }

    pub fn set_focal_point(&self, fp: types::F3DPoint3) {
        unsafe {
            f3d_camera_set_focal_point(self.ptr.as_ptr(), fp.as_ptr() as *mut f64);
        }
    }

    pub fn get_focal_point(&self) -> types::F3DPoint3 {
        let mut fp = types::F3DPoint3::default();
        unsafe {
            f3d_camera_get_focal_point(self.ptr.as_ptr(), fp.as_mut_ptr());
        }
        fp
    }

    pub fn set_view_up(&self, vu: types::F3DVector3) {
        unsafe {
            f3d_camera_set_view_up(self.ptr.as_ptr(), vu.as_ptr() as *mut f64);
        }
    }

    pub fn get_view_up(&self) -> types::F3DVector3 {
        let mut vu = types::F3DVector3::default();
        unsafe {
            f3d_camera_get_view_up(self.ptr.as_ptr(), vu.as_mut_ptr());
        }
        vu
    }

    pub fn set_view_angle(&self, angle: f64) {
        unsafe {
            f3d_camera_set_view_angle(self.ptr.as_ptr(), angle);
        }
    }

    pub fn get_view_angle(&self) -> f64 {
        unsafe { f3d_camera_get_view_angle(self.ptr.as_ptr()) }
    }

    pub fn get_state(&self) -> CameraState {
        unsafe {
            let mut state_raw = std::mem::MaybeUninit::<f3d_camera_state_t>::uninit();

            f3d_camera_get_state(self.ptr.as_ptr(), state_raw.as_mut_ptr());

            let state_raw = state_raw.assume_init();

            CameraState {
                position: types::F3DPoint3::from_ptr(&state_raw.position[0]),
                focal_point: types::F3DPoint3::from_ptr(&state_raw.focal_point[0]),
                view_up: types::F3DVector3::from_ptr(&state_raw.view_up[0]),
                view_angle: state_raw.view_angle,
            }
        }
    }

    pub fn camera_dolly(&self, val: f64) {
        unsafe {
            f3d_camera_dolly(self.ptr.as_ptr(), val);
        }
    }

    pub fn camera_pan(&self, right: f64, up: f64, forward: f64) {
        unsafe {
            f3d_camera_pan(self.ptr.as_ptr(), right, up, forward);
        }
    }

    pub fn camera_zoom(&self, factor: f64) {
        unsafe {
            f3d_camera_zoom(self.ptr.as_ptr(), factor);
        }
    }

    pub fn camera_roll(&self, angle: f64) {
        unsafe {
            f3d_camera_roll(self.ptr.as_ptr(), angle);
        }
    }

    pub fn camera_azimuth(&self, angle: f64) {
        unsafe {
            f3d_camera_azimuth(self.ptr.as_ptr(), angle);
        }
    }

    pub fn camera_yaw(&self, angle: f64) {
        unsafe {
            f3d_camera_yaw(self.ptr.as_ptr(), angle);
        }
    }

    pub fn camera_elevation(&self, angle: f64) {
        unsafe {
            f3d_camera_elevation(self.ptr.as_ptr(), angle);
        }
    }

    pub fn camera_pitch(&self, angle: f64) {
        unsafe {
            f3d_camera_pitch(self.ptr.as_ptr(), angle);
        }
    }

    pub fn set_current_as_default(&self) {
        unsafe {
            f3d_camera_set_current_as_default(self.ptr.as_ptr());
        }
    }

    pub fn reset_to_default(&self) {
        unsafe {
            f3d_camera_reset_to_default(self.ptr.as_ptr());
        }
    }

    pub fn reset_to_bounds(&self, zoom_factor: f64) {
        unsafe {
            f3d_camera_reset_to_bounds(self.ptr.as_ptr(), zoom_factor);
        }
    }
}
