// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum WindowType {
//     None,
//     External,
//     Glx,
//     Wgl,
//     Cocoa,
//     Egl,
//     Osmesa,
//     Wasm,
//     Unknown,
// }
// 
// impl From<f3d_window_type_t> for WindowType {
//     fn from(value: f3d_window_type_t) -> Self {
//         match value {
//             f3d_window_type_t::F3D_WINDOW_NONE => Self::None,
//             f3d_window_type_t::F3D_WINDOW_EXTERNAL => Self::External,
//             f3d_window_type_t::F3D_WINDOW_GLX => Self::Glx,
//             f3d_window_type_t::F3D_WINDOW_WGL => Self::Wgl,
//             f3d_window_type_t::F3D_WINDOW_COCOA => Self::Cocoa,
//             f3d_window_type_t::F3D_WINDOW_EGL => Self::Egl,
//             f3d_window_type_t::F3D_WINDOW_OSMESA => Self::Osmesa,
//             f3d_window_type_t::F3D_WINDOW_WASM => Self::Wasm,
//             _ => Self::Unknown,
//         }
//     }
// }
// 
