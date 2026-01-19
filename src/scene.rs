use crate::sys::*;
use crate::types::{F3DLightState, F3DMesh, F3DMeshFFI};
use std::ffi::{CString, NulError};
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

    pub fn add(&self, path: &str) -> Result<i32, NulError> {
        let cpath = CString::new(path).expect("path contains interior null bytes");
        unsafe { Ok(f3d_scene_add(self.ptr.as_ptr(), cpath.as_ptr())) }
    }

    pub fn add_multiple(&self, paths: Vec<&str>) -> Result<i32, NulError> {
        let cstrings: Vec<CString> = paths
            .into_iter()
            .map(|p| CString::new(p).expect("path contains interior null bytes"))
            .collect();
        let cptrs: Vec<*const i8> = cstrings.iter().map(|cs| cs.as_ptr()).collect();
        unsafe {
            Ok(f3d_scene_add_multiple(
                self.ptr.as_ptr(),
                cptrs.as_ptr() as *mut *const i8,
                cptrs.len() as usize,
            ))
        }
    }

    pub fn add_mesh(&self, mesh: &F3DMesh) -> Result<i32, NulError> {
        let f3d_mesh_ffi = mesh.as_f3d_mesh();
        unsafe {
            Ok(f3d_scene_add_mesh(
                self.ptr.as_ptr(),
                &f3d_mesh_ffi as *const F3DMeshFFI as *const f3d_mesh_t,
            ))
        }
    }

    pub fn add_buffer(&self, buffer: &mut [u8]) -> Result<i32, NulError> {
        unsafe {
            Ok(f3d_scene_add_buffer(
                self.ptr.as_ptr(),
                buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                buffer.len() as usize,
            ))
        }
    }

    pub fn clear(&self) {
        unsafe {
            f3d_scene_clear(self.ptr.as_ptr());
        }
    }

    pub fn add_light(&self, light_state: F3DLightState) -> i32 {
        unsafe {
            f3d_scene_add_light(
                self.ptr.as_ptr(),
                &light_state as *const F3DLightState as *const f3d_light_state_t,
            )
        }
    }

    pub fn get_light_count(&self) -> i32 {
        unsafe { f3d_scene_get_light_count(self.ptr.as_ptr()) }
    }

    pub fn get_light(&self, index: i32) -> Option<F3DLightState> {
        unsafe {
            let light_ptr = f3d_scene_get_light(self.ptr.as_ptr(), index);
            if light_ptr.is_null() {
                None
            } else {
                let light = *(light_ptr as *const F3DLightState);
                f3d_light_state_free(light_ptr);
                Some(light)
            }
        }
    }

    pub fn update_light(&self, index: i32, light_state: F3DLightState) -> Result<i32, NulError> {
        unsafe {
            Ok(f3d_scene_update_light(
                self.ptr.as_ptr(),
                index,
                &light_state as *const F3DLightState as *const f3d_light_state_t,
            ))
        }
    }

    pub fn remove_light(&self, index: i32) -> Result<i32, NulError> {
        unsafe { Ok(f3d_scene_remove_light(self.ptr.as_ptr(), index)) }
    }

    pub fn remove_all_lights(&self) {
        unsafe {
            f3d_scene_remove_all_lights(self.ptr.as_ptr());
        }
    }

    pub fn supports(&self, file_path: &str) -> bool {
        let c_file_path = CString::new(file_path).expect("CString::new failed");
        unsafe { f3d_scene_supports(self.ptr.as_ptr(), c_file_path.as_ptr()) == 1 }
    }

    pub fn load_animation_time(&self, time_value: f64) {
        unsafe {
            f3d_scene_load_animation_time(self.ptr.as_ptr(), time_value);
        }
    }

    pub fn animation_time_range(&self) -> (f64, f64) {
        let mut min_time: f64 = 0.0;
        let mut max_time: f64 = 0.0;
        unsafe {
            f3d_scene_animation_time_range(
                self.ptr.as_ptr(),
                &mut min_time as *mut f64,
                &mut max_time as *mut f64,
            );
        }
        (min_time, max_time)
    }

    pub fn available_animations(&self) -> u32 {
        unsafe { f3d_scene_available_animations(self.ptr.as_ptr()) }
    }
}
