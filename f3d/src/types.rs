#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DPoint3 {
    pub data: [f64; 3],
}

impl F3DPoint3 {
    pub const ZERO: F3DPoint3 = F3DPoint3 {
        data: [0.0, 0.0, 0.0],
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        F3DPoint3 { data: [x, y, z] }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const f64 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut f64 {
        self.data.as_mut_ptr()
    }

    pub fn default() -> Self {
        F3DPoint3::ZERO
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DVector3 {
    pub data: [f64; 3],
}

impl F3DVector3 {
    pub const ZERO: F3DVector3 = F3DVector3 {
        data: [0.0, 0.0, 0.0],
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        F3DVector3 { data: [x, y, z] }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const f64 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut f64 {
        self.data.as_mut_ptr()
    }

    pub fn default() -> Self {
        F3DVector3::ZERO
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DRatio {
    pub value: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DColor {
    pub data: [f64; 3],
}

impl F3DColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        F3DColor { data: [r, g, b] }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DDirection {
    pub data: [f64; 3],
}

impl F3DDirection {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        F3DDirection { data: [x, y, z] }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DTransform2d {
    pub data: [f64; 9],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DColormap {
    pub data: *mut f64,
    pub count: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DMesh {
    pub points: *mut f32,
    pub points_count: usize,
    pub normals: *mut f32,
    pub normals_count: usize,
    pub texture_coordinates: *mut f32,
    pub texture_coordinates_count: usize,
    pub face_sides: *mut u32,
    pub face_sides_count: usize,
    pub face_indices: *mut u32,
    pub face_indices_count: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum F3DLightType {
    Headlight = 1,
    CameraLight = 2,
    SceneLight = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F3DLightState {
    pub type_: F3DLightType,
    pub position: [f64; 3],
    pub color: F3DColor,
    pub direction: [f64; 3],
    pub positional_light: i32,
    pub intensity: f64,
    pub switch_state: i32,
}
