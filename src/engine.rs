use crate::interactor::Interactor;
use crate::options::Options;
use crate::scene::Scene;
use crate::sys::*;
use crate::window::Window;
use std::ffi::CStr;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct F3DModuleInfo {
    pub name: String,
    pub available: bool,
}

#[derive(Debug, Clone)]
pub struct F3DLibInfo {
    pub version: String,
    pub version_full: String,
    pub build_date: String,
    pub build_system: String,
    pub compiler: String,
    pub modules: F3DModuleInfo,
    pub vtk_version: String,
    pub copyrights: Vec<String>,
    pub license: String,
}

#[derive(Debug, Clone)]
pub struct F3DReaderInfo {
    pub name: String,
    pub description: String,
    pub extensions: Vec<String>,
    pub mime_types: Vec<String>,
    pub plugin_name: String,
    pub has_scene_reader: i32,
    pub has_geometry_reader: i32,
}

pub enum EngineBackend {
    Auto { offscreen: bool },
    None,
    Glx { offscreen: bool },
    Wgl { offscreen: bool },
    Egl,   // offscreen EGL window
    Omesa, // offscreen Omesa window
    Cocoa,
}

pub enum EngineContext {
    Internal,
    ExternalPlatform,
    //TODO: External get_proc_address
}

pub struct Engine {
    ptr: NonNull<f3d_engine_t>,
}

impl Engine {
    pub fn new(backend: EngineBackend, context: EngineContext) -> Self {
        let ptr = unsafe {
            match (backend, context) {
                (EngineBackend::Auto { offscreen }, EngineContext::Internal) => {
                    f3d_engine_create(offscreen as i32)
                }

                (EngineBackend::None, EngineContext::Internal) => f3d_engine_create_none(),

                (EngineBackend::Glx { offscreen }, EngineContext::Internal) => {
                    f3d_engine_create_glx(offscreen as i32)
                }

                (EngineBackend::Wgl { offscreen }, EngineContext::Internal) => {
                    f3d_engine_create_wgl(offscreen as i32)
                }

                (EngineBackend::Egl, EngineContext::Internal) => f3d_engine_create_egl(),

                (EngineBackend::Omesa, EngineContext::Internal) => f3d_engine_create_osmesa(),

                (EngineBackend::Glx { .. }, EngineContext::ExternalPlatform) => {
                    f3d_engine_create_external_glx()
                }

                (EngineBackend::Wgl { .. }, EngineContext::ExternalPlatform) => {
                    f3d_engine_create_external_wgl()
                }

                (EngineBackend::Cocoa, EngineContext::ExternalPlatform) => {
                    f3d_engine_create_external_cocoa()
                }

                _ => panic!("invalid backend/context combination"),
            }
        };
        Self {
            ptr: NonNull::new(ptr).expect("f3d_engine_create returned null"),
        }
    }

    pub fn auto_load_plugins() {
        unsafe {
            f3d_engine_autoload_plugins();
        }
    }

    pub fn set_cache_path(&self, path: &str) {
        let c_path = std::ffi::CString::new(path).expect("CString::new failed");
        unsafe {
            f3d_engine_set_cache_path(self.ptr.as_ptr(), c_path.as_ptr());
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

    pub fn get_rendering_backends() -> Vec<(String, bool)> {
        unsafe {
            let mut count: i32 = 0;
            let list = f3d_engine_get_rendering_backend_list(&mut count as *mut i32);

            if list.is_null() {
                return Vec::new();
            }

            let mut backends = Vec::new();
            let mut i = 0;

            while !(*list.add(i)).name.is_null() {
                let info = &*list.add(i);
                let name = CStr::from_ptr(info.name).to_string_lossy().into_owned();
                backends.push((name, info.available != 0));
                i += 1;
            }

            // Free the list after use
            f3d_engine_free_backend_list(list);

            backends
        }
    }

    pub fn load_plugin(plugin_path: &str) {
        let c_plugin_path = std::ffi::CString::new(plugin_path).expect("CString::new failed");
        unsafe { f3d_engine_load_plugin(c_plugin_path.as_ptr()) }
    }

    pub fn get_plugins_list(path: &str) -> Vec<String> {
        let c_path = std::ffi::CString::new(path).expect("CString::new failed");
        unsafe {
            let ptr = f3d_engine_get_plugins_list(c_path.as_ptr());
            if ptr.is_null() {
                return Vec::new();
            }

            let mut plugins = Vec::new();
            let mut i = 0;
            loop {
                let current = *ptr.add(i);
                if current.is_null() {
                    break;
                }
                let name = CStr::from_ptr(current).to_string_lossy().into_owned();
                plugins.push(name);
                i += 1;
            }

            f3d_engine_free_string_array(ptr);

            plugins
        }
    }

    pub fn get_all_reader_option_names() -> Vec<String> {
        unsafe {
            let ptr = f3d_engine_get_all_reader_option_names();
            if ptr.is_null() {
                return Vec::new();
            }

            let mut names = Vec::new();
            let mut i = 0;
            loop {
                let current = *ptr.add(i);
                if current.is_null() {
                    break;
                }
                let name = CStr::from_ptr(current).to_string_lossy().into_owned();
                names.push(name);
                i += 1;
            }

            f3d_engine_free_string_array(ptr);

            names
        }
    }

    pub fn set_reader_option(name: &str, value: &str) {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        let c_value = std::ffi::CString::new(value).expect("CString::new failed");
        unsafe {
            f3d_engine_set_reader_option(c_name.as_ptr(), c_value.as_ptr());
        }
    }

    pub fn get_lib_info() -> F3DLibInfo {
        unsafe {
            let raw_info = f3d_engine_get_lib_info();
            if raw_info.is_null() {
                panic!("f3d_engine_get_lib_info returned null");
            }

            let info = &*raw_info;

            // Convert C strings and array directly
            let version = if info.version.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(info.version)
                    .to_string_lossy()
                    .into_owned()
            };

            let version_full = if info.version_full.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(info.version_full)
                    .to_string_lossy()
                    .into_owned()
            };

            let build_date = if info.build_date.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(info.build_date)
                    .to_string_lossy()
                    .into_owned()
            };

            let build_system = if info.build_system.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(info.build_system)
                    .to_string_lossy()
                    .into_owned()
            };

            let compiler = if info.compiler.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(info.compiler)
                    .to_string_lossy()
                    .into_owned()
            };

            let modules = F3DModuleInfo {
                name: if (*info.modules).name.is_null() {
                    String::new()
                } else {
                    std::ffi::CStr::from_ptr((*info.modules).name)
                        .to_string_lossy()
                        .into_owned()
                },
                available: (*info.modules).available != 0,
            };

            let vtk_version = if info.vtk_version.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(info.vtk_version)
                    .to_string_lossy()
                    .into_owned()
            };

            let mut copyrights = Vec::new();
            if !info.copyrights.is_null() {
                let mut i = 0;
                loop {
                    let ptr = *info.copyrights.add(i);
                    if ptr.is_null() {
                        break;
                    }
                    let s = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    copyrights.push(s);
                    i += 1;
                }
            }

            let license = if info.license.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(info.license)
                    .to_string_lossy()
                    .into_owned()
            };

            f3d_engine_free_lib_info(raw_info);

            F3DLibInfo {
                version,
                version_full,
                build_date,
                build_system,
                compiler,
                modules,
                vtk_version,
                copyrights: copyrights,
                license,
            }
        }
    }

    pub fn get_readers_info() -> Vec<F3DReaderInfo> {
        unsafe {
            let mut count: i32 = 0;
            let list = f3d_engine_get_readers_info(&mut count as *mut i32);

            if list.is_null() {
                return Vec::new();
            }

            let mut readers = Vec::new();
            let mut i = 0;

            while !(*list.add(i)).name.is_null() {
                let info = &*list.add(i);

                let name = if info.name.is_null() {
                    String::new()
                } else {
                    std::ffi::CStr::from_ptr(info.name)
                        .to_string_lossy()
                        .into_owned()
                };

                let description = if info.description.is_null() {
                    String::new()
                } else {
                    std::ffi::CStr::from_ptr(info.description)
                        .to_string_lossy()
                        .into_owned()
                };

                let plugin_name = if info.plugin_name.is_null() {
                    String::new()
                } else {
                    std::ffi::CStr::from_ptr(info.plugin_name)
                        .to_string_lossy()
                        .into_owned()
                };

                let mut extensions = Vec::new();
                if !info.extensions.is_null() {
                    let mut j = 0;
                    loop {
                        let ptr = *info.extensions.add(j);
                        if ptr.is_null() {
                            break;
                        }
                        extensions
                            .push(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned());
                        j += 1;
                    }
                }

                let mut mime_types = Vec::new();
                if !info.mime_types.is_null() {
                    let mut j = 0;
                    loop {
                        let ptr = *info.mime_types.add(j);
                        if ptr.is_null() {
                            break;
                        }
                        mime_types
                            .push(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned());
                        j += 1;
                    }
                }

                readers.push(F3DReaderInfo {
                    name,
                    description,
                    extensions,
                    mime_types,
                    plugin_name,
                    has_scene_reader: info.has_scene_reader,
                    has_geometry_reader: info.has_geometry_reader,
                });

                i += 1;
            }

            f3d_engine_free_readers_info(list);

            readers
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            f3d_engine_delete(self.ptr.as_ptr());
        }
    }
}
