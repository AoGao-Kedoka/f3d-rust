use crate::sys::*;
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImageChannelType {
    BYTE = 0,
    SHORT,
    FLOAT,
}

impl ImageChannelType {
    pub fn from_raw(v: u32) -> Self {
        match v {
            0 => ImageChannelType::BYTE,
            1 => ImageChannelType::SHORT,
            2 => ImageChannelType::FLOAT,
            _ => panic!("Unknown ImageChannelType value: {}", v),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImageFormat {
    PNG = 0,
    JPEG,
    TIF,
    DMP,
}

pub struct Image {
    ptr: NonNull<f3d_image_t>,
}

impl Image {
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_image_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_image_t"),
        }
    }

    pub fn new_params(
        width: u32,
        height: u32,
        channel_count: u32,
        channel_type: ImageChannelType,
    ) -> Option<Self> {
        let ptr =
            unsafe { f3d_image_new_params(width, height, channel_count, channel_type as u32) };

        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn new_empty() -> Option<Self> {
        let ptr = unsafe { f3d_image_new_empty() };
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn new_path(path: &str) -> Option<Self> {
        let c_path = std::ffi::CString::new(path).expect("CString::new failed");
        let ptr = unsafe { f3d_image_new_path(c_path.as_ptr()) };
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn as_ptr(&self) -> *mut f3d_image_t {
        self.ptr.as_ptr()
    }

    pub fn normalized_pixel(&self, x: i32, y: i32) -> Vec<f64> {
        let channel_count = self.get_channel_count() as usize;
        let mut pixel = vec![0.0f64; channel_count];

        unsafe {
            f3d_image_get_normalized_pixel(self.ptr.as_ptr(), x, y, pixel.as_mut_ptr());
        }

        pixel
    }

    pub fn supported_formats() -> Vec<String> {
        let count = unsafe { f3d_image_get_supported_formats_count() } as usize;
        let ptr = unsafe { f3d_image_get_supported_formats() };

        if ptr.is_null() || count == 0 {
            return Vec::new();
        }

        let formats = unsafe { std::slice::from_raw_parts(ptr, count) };

        formats
            .iter()
            .map(|&cstr| {
                assert!(!cstr.is_null());
                unsafe {
                    std::ffi::CStr::from_ptr(cstr)
                        .to_string_lossy()
                        .into_owned()
                }
            })
            .collect()
    }

    pub fn get_width(&self) -> u32 {
        unsafe { f3d_image_get_width(self.ptr.as_ptr()) }
    }

    pub fn get_height(&self) -> u32 {
        unsafe { f3d_image_get_height(self.ptr.as_ptr()) }
    }

    pub fn get_channel_count(&self) -> u32 {
        unsafe { f3d_image_get_channel_count(self.ptr.as_ptr()) }
    }

    pub fn channel_type(&self) -> ImageChannelType {
        ImageChannelType::from_raw(unsafe { f3d_image_get_channel_type(self.ptr.as_ptr()) })
    }

    pub fn set_content(&mut self, data: &[u8]) {
        unsafe {
            f3d_image_set_content(self.ptr.as_ptr(), data.as_ptr() as *mut std::ffi::c_void);
        }
    }

    pub fn get_content(&self) -> Vec<u8> {
        let size = (self.get_width() * self.get_height() * self.get_channel_count()) as usize;

        let ptr = unsafe { f3d_image_get_content(self.ptr.as_ptr()) as *const u8 };

        if ptr.is_null() || size == 0 {
            return Vec::new();
        }

        unsafe { std::slice::from_raw_parts(ptr, size).to_vec() }
    }

    /// Returns the SSIM between this image and the reference image.
    pub fn compare(&self, reference: &Image) -> f64 {
        unsafe { f3d_image_compare(self.ptr.as_ptr(), reference.ptr.as_ptr()) }
    }

    pub fn save(&self, path: &str, format: ImageFormat) {
        let c_path = std::ffi::CString::new(path).expect("CString::new failed");
        unsafe {
            f3d_image_save(self.ptr.as_ptr(), c_path.as_ptr(), format as u32);
        }
    }

    pub fn save_to_buffer(&self, format: ImageFormat) -> Vec<u8> {
        let mut size: u32 = 0;

        let buf_ptr = unsafe {
            f3d_image_save_buffer(self.ptr.as_ptr(), format as u32, &mut size as *mut u32)
        };

        if buf_ptr.is_null() || size == 0 {
            return Vec::new();
        }

        let slice = unsafe { std::slice::from_raw_parts(buf_ptr, size as usize) };
        let vec = slice.to_vec();

        unsafe {
            f3d_image_free_buffer(buf_ptr);
        }

        vec
    }

    pub fn to_terminal_text(&self) -> String {
        let ptr = unsafe { f3d_image_to_terminal_text_string(self.ptr.as_ptr()) };

        if ptr.is_null() {
            return String::new();
        }

        unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }

    pub fn set_metadata(&mut self, key: &str, value: &str) {
        let c_key = std::ffi::CString::new(key).expect("CString::new failed");
        let c_value = std::ffi::CString::new(value).expect("CString::new failed");

        unsafe {
            f3d_image_set_metadata(self.ptr.as_ptr(), c_key.as_ptr(), c_value.as_ptr());
        }
    }

    pub fn get_metadata(&self, key: &str) -> Option<String> {
        let c_key = std::ffi::CString::new(key).expect("CString::new failed");

        let ptr = unsafe { f3d_image_get_metadata(self.ptr.as_ptr(), c_key.as_ptr()) };
        if ptr.is_null() {
            return None;
        }

        Some(unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() })
    }

    pub fn all_metadata(&self) -> Vec<String> {
        let mut count: u32 = 0;

        let keys_ptr = unsafe { f3d_image_all_metadata(self.ptr.as_ptr(), &mut count as *mut u32) };
        if keys_ptr.is_null() || count == 0 {
            return Vec::new();
        }

        let keys_slice = unsafe { std::slice::from_raw_parts(keys_ptr, count as usize) };

        let keys: Vec<String> = keys_slice
            .iter()
            .filter_map(|&k| {
                if k.is_null() {
                    None
                } else {
                    Some(unsafe { std::ffi::CStr::from_ptr(k).to_string_lossy().into_owned() })
                }
            })
            .collect();

        unsafe { f3d_image_free_metadata_keys(keys_ptr, count) };

        keys
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        unsafe { f3d_image_equals(self.ptr.as_ptr(), other.ptr.as_ptr()) != 0 }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            f3d_image_delete(self.ptr.as_ptr());
        }
    }
}
