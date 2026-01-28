use crate::sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InteractionBindModifierKeys {
    InteractionBindAny = 0x80,
    InteractionBindNone = 0x00,
    InteractionBindCtrl = 0x01,
    InteractionBindShift = 0x02,
    InteractionBindCtrlShft = 0x03,
}

impl InteractionBindModifierKeys {
    fn to_raw(&self) -> u8 {
        match self {
            InteractionBindModifierKeys::InteractionBindAny => 0x80,
            InteractionBindModifierKeys::InteractionBindNone => 0x00,
            InteractionBindModifierKeys::InteractionBindCtrl => 0x01,
            InteractionBindModifierKeys::InteractionBindShift => 0x02,
            InteractionBindModifierKeys::InteractionBindCtrlShft => 0x03,
        }
    }

    fn from_raw(value: u8) -> Self {
        match value {
            0x80 => InteractionBindModifierKeys::InteractionBindAny,
            0x00 => InteractionBindModifierKeys::InteractionBindNone,
            0x01 => InteractionBindModifierKeys::InteractionBindCtrl,
            0x02 => InteractionBindModifierKeys::InteractionBindShift,
            0x03 => InteractionBindModifierKeys::InteractionBindCtrlShft,
            _ => panic!("Invalid modifier key value"),
        }
    }
}

pub struct InteractionBind {
    pub modifier_keys: InteractionBindModifierKeys,
    pub interaction_key: [char; 256],
}

impl InteractionBind {
    fn from_raw(raw: &f3d_interaction_bind_t) -> Self {
        unsafe {
            let mut interaction_key = ['\0'; 256];

            let cstr = std::ffi::CStr::from_ptr(&raw.inter[0]);
            for (i, ch) in cstr.to_string_lossy().chars().enumerate() {
                if i >= 256 {
                    break;
                }
                interaction_key[i] = ch;
            }

            Self {
                modifier_keys: InteractionBindModifierKeys::from_raw(raw.mod_ as u8),
                interaction_key,
            }
        }
    }

    fn to_raw(&self) -> f3d_interaction_bind_t {
        let mut raw = f3d_interaction_bind_t {
            mod_: self.modifier_keys.to_raw() as u32,
            inter: [0; 256],
        };

        for (i, ch) in self.interaction_key.iter().enumerate() {
            if *ch == '\0' {
                break;
            }
            raw.inter[i] = *ch as i8;
        }

        raw
    }

    pub fn parse(binding_str: &str) -> Self {
        let cstr = CString::new(binding_str).expect("Failed to create CString from input string");

        let mut raw = f3d_interaction_bind_t {
            mod_: 0,
            inter: [0; 256],
        };

        unsafe {
            f3d_interaction_bind_parse(cstr.as_ptr(), &mut raw as *mut f3d_interaction_bind_t);
        }

        InteractionBind::from_raw(&raw)
    }
}

impl std::fmt::Display for InteractionBind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let raw = self.to_raw();
            let mut output = [0u8; 512];

            f3d_interaction_bind_format(
                &raw as *const f3d_interaction_bind_t,
                output.as_mut_ptr() as *mut i8,
                output.len() as i32,
            );

            let s = std::ffi::CStr::from_ptr(output.as_ptr() as *const i8).to_string_lossy();

            write!(f, "{s}")
        }
    }
}

impl PartialEq for InteractionBind {
    fn eq(&self, other: &Self) -> bool {
        unsafe { f3d_interaction_bind_equals(&self.to_raw(), &other.to_raw()) == 1 }
    }
}

impl PartialOrd for InteractionBind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe {
            let lhs = self.to_raw();
            let rhs = other.to_raw();

            if f3d_interaction_bind_less_than(&lhs, &rhs) == 1 {
                Some(std::cmp::Ordering::Less)
            } else if f3d_interaction_bind_equals(&lhs, &rhs) == 1 {
                Some(std::cmp::Ordering::Equal)
            } else {
                Some(std::cmp::Ordering::Greater)
            }
        }
    }
}

pub enum InteractorBindType {
    InteractorBindingCyclic = 0,
    InteractorBindingNumerical = 1,
    InteractorBindingToggle = 2,
    InteractorBindingOther = 3,
}

impl InteractorBindType {
    fn to_raw(&self) -> u32 {
        match self {
            InteractorBindType::InteractorBindingCyclic => 0,
            InteractorBindType::InteractorBindingNumerical => 1,
            InteractorBindType::InteractorBindingToggle => 2,
            InteractorBindType::InteractorBindingOther => 3,
        }
    }

    fn from_raw(value: u32) -> Self {
        match value {
            0 => InteractorBindType::InteractorBindingCyclic,
            1 => InteractorBindType::InteractorBindingNumerical,
            2 => InteractorBindType::InteractorBindingToggle,
            3 => InteractorBindType::InteractorBindingOther,
            _ => panic!("Invalid InteractorBindType value"),
        }
    }
}

pub enum InteractorMouseButton {
    InteractorMouseButtonLeft = 0,
    InteractorMouseButtonRight = 1,
    InteractorMouseButtonMiddle = 2,
}

impl InteractorMouseButton {
    fn to_raw(&self) -> u32 {
        match self {
            InteractorMouseButton::InteractorMouseButtonLeft => 0,
            InteractorMouseButton::InteractorMouseButtonRight => 1,
            InteractorMouseButton::InteractorMouseButtonMiddle => 2,
        }
    }

    fn from_raw(value: u32) -> Self {
        match value {
            0 => InteractorMouseButton::InteractorMouseButtonLeft,
            1 => InteractorMouseButton::InteractorMouseButtonRight,
            2 => InteractorMouseButton::InteractorMouseButtonMiddle,
            _ => panic!("Invalid InteractorMouseButton value"),
        }
    }
}

pub enum InteractorWheelDirection {
    InteractorWheelForward = 0,
    InteractorWheelBackward = 1,
    InteractorWheelLeft = 2,
    InteractorWheelRight = 3,
}

impl InteractorWheelDirection {
    fn to_raw(&self) -> u32 {
        match self {
            InteractorWheelDirection::InteractorWheelForward => 0,
            InteractorWheelDirection::InteractorWheelBackward => 1,
            InteractorWheelDirection::InteractorWheelLeft => 2,
            InteractorWheelDirection::InteractorWheelRight => 3,
        }
    }

    fn from_raw(value: u32) -> Self {
        match value {
            0 => InteractorWheelDirection::InteractorWheelForward,
            1 => InteractorWheelDirection::InteractorWheelBackward,
            2 => InteractorWheelDirection::InteractorWheelLeft,
            3 => InteractorWheelDirection::InteractorWheelRight,
            _ => panic!("Invalid InteractorWheelDirection value"),
        }
    }
}

pub enum InteractorInputaction {
    InteractorInputPress = 0,
    InteractorInputRelease = 1,
}

impl InteractorInputaction {
    fn to_raw(&self) -> u32 {
        match self {
            InteractorInputaction::InteractorInputPress => 0,
            InteractorInputaction::InteractorInputRelease => 1,
        }
    }

    fn from_raw(value: u32) -> Self {
        match value {
            0 => InteractorInputaction::InteractorInputPress,
            1 => InteractorInputaction::InteractorInputRelease,
            _ => panic!("Invalid interactorInputaction value"),
        }
    }
}

pub enum InteractorInputModifier {
    InteractorInputNone = 0,
    InteractorInputCtrl = 1,
    InteractorInputShift = 2,
    InteractorInputCtrlShift = 3,
}

impl InteractorInputModifier {
    fn to_raw(&self) -> u32 {
        match self {
            InteractorInputModifier::InteractorInputNone => 0,
            InteractorInputModifier::InteractorInputCtrl => 1,
            InteractorInputModifier::InteractorInputShift => 2,
            InteractorInputModifier::InteractorInputCtrlShift => 3,
        }
    }

    fn from_raw(value: u32) -> Self {
        match value {
            0 => InteractorInputModifier::InteractorInputNone,
            1 => InteractorInputModifier::InteractorInputCtrl,
            2 => InteractorInputModifier::InteractorInputShift,
            3 => InteractorInputModifier::InteractorInputCtrlShift,
            _ => panic!("Invalid InteractorInputModifier value"),
        }
    }
}

pub enum InteractorAnimationDirection {
    InteractorAnimationForward = 0,
    InteractorAnimationBackward = 1,
}

impl InteractorAnimationDirection {
    fn to_raw(&self) -> u32 {
        match self {
            InteractorAnimationDirection::InteractorAnimationForward => 0,
            InteractorAnimationDirection::InteractorAnimationBackward => 1,
        }
    }

    fn from_raw(value: u32) -> Self {
        match value {
            0 => InteractorAnimationDirection::InteractorAnimationForward,
            1 => InteractorAnimationDirection::InteractorAnimationBackward,
            _ => panic!("Invalid InteractorAnimationDirection value"),
        }
    }
}

pub struct BindingDocumentation {
    doc: String,
    value: String,
}

impl BindingDocumentation {
    fn from_raw(raw: &f3d_binding_documentation_t) -> Self {
        unsafe {
            let doc_cstr = std::ffi::CStr::from_ptr(raw.doc.as_ptr());
            let values_cstr = std::ffi::CStr::from_ptr(raw.value.as_ptr());

            Self {
                doc: doc_cstr.to_string_lossy().into_owned(),
                value: values_cstr.to_string_lossy().into_owned(),
            }
        }
    }

    fn to_raw(&self) -> f3d_binding_documentation_t {
        let mut raw = f3d_binding_documentation_t {
            doc: [0; 512],
            value: [0; 256],
        };

        let doc_bytes = self.doc.as_bytes();
        let values_bytes = self.value.as_bytes();

        for (i, &b) in doc_bytes.iter().enumerate() {
            if i >= 512 {
                break;
            }
            raw.doc[i] = b as i8;
        }

        for (i, &b) in values_bytes.iter().enumerate() {
            if i >= 256 {
                break;
            }
            raw.value[i] = b as i8;
        }

        raw
    }
}

pub struct Interactor {
    ptr: NonNull<f3d_interactor_t>,
}

impl Interactor {
    pub(crate) unsafe fn from_raw(ptr: *mut f3d_interactor_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("null f3d_interactor_t"),
        }
    }

    pub fn init_commands(&self) {
        unsafe {
            f3d_interactor_init_commands(self.ptr.as_ptr());
        }
    }

    extern "C" fn command_callback(
        args: *mut *const c_char,
        arg_count: c_int,
        user_data: *mut c_void,
    ) {
        unsafe {
            let args_slice = std::slice::from_raw_parts(args, arg_count as usize);

            let args: Vec<String> = args_slice
                .iter()
                .map(|&arg| CStr::from_ptr(arg).to_string_lossy().into_owned())
                .collect();

            let closure = &*(user_data as *mut Box<dyn Fn(&[String]) + Send>);
            closure(&args);
        }
    }

    pub fn add_command<F>(&self, action: &str, callback: F)
    where
        F: Fn(&[String]) + Send + 'static,
    {
        let action_cstr = CString::new(action).expect("Invalid action");

        // Box the closure and leak it
        let boxed: Box<Box<dyn Fn(&[String]) + Send>> = Box::new(Box::new(callback));

        let user_data = Box::into_raw(boxed) as *mut c_void;

        unsafe {
            f3d_interactor_add_command(
                self.ptr.as_ptr(),
                action_cstr.as_ptr(),
                Some(
                    Self::command_callback
                        as unsafe extern "C" fn(*mut *const c_char, c_int, *mut c_void),
                ),
                user_data,
            );
        }
    }

    pub fn remove_command(&self, action: &str) {
        let action_cstr = CString::new(action).expect("Invalid action");
        unsafe {
            f3d_interactor_remove_command(self.ptr.as_ptr(), action_cstr.as_ptr());
        }
    }

    pub fn get_command_actions(&self) -> Vec<String> {
        let mut count: i32 = 0;
        unsafe {
            let raw_actions =
                f3d_interactor_get_command_actions(self.ptr.as_ptr(), &mut count as *mut i32);

            let mut actions = Vec::new();
            for i in 0..count {
                let cstr = CStr::from_ptr(*raw_actions.add(i as usize));
                actions.push(cstr.to_string_lossy().into_owned());
            }

            f3d_interactor_free_string_array(raw_actions, count);

            actions
        }
    }

    pub fn trigger_command(&self, command: String, keep_comments: bool) {
        let ccommand = CString::new(command).unwrap();
        unsafe {
            f3d_interactor_trigger_command(
                self.ptr.as_ptr(),
                ccommand.as_ptr(),
                keep_comments as i32,
            );
        }
    }

    pub fn init_bindings(&self) {
        unsafe {
            f3d_interactor_init_bindings(self.ptr.as_ptr());
        }
    }

    pub fn add_binding(&self, bind: InteractionBind, commands: Vec<String>, group: Option<&str>) {
        let bind_raw = bind.to_raw();
        let commands_cstr: Vec<CString> = commands
            .iter()
            .map(|cmd| CString::new(cmd.as_str()).expect("Failed to create CString"))
            .collect();
        let commands_ptrs: Vec<*const i8> = commands_cstr.iter().map(|cmd| cmd.as_ptr()).collect();
        let cgroup = group.map(|g| CString::new(g).expect("Failed to create CString for group"));

        unsafe {
            f3d_interactor_add_binding(
                self.ptr.as_ptr(),
                &bind_raw as *const f3d_interaction_bind_t,
                commands_ptrs.as_ptr() as *mut *const i8,
                commands_ptrs.len() as i32,
                cgroup.map_or(std::ptr::null(), |cg| cg.as_ptr()),
            );
        }
    }

    pub fn remove_binding(&self, bind: InteractionBind) {
        let bind_raw = bind.to_raw();
        unsafe {
            f3d_interactor_remove_binding(
                self.ptr.as_ptr(),
                &bind_raw as *const f3d_interaction_bind_t,
            );
        }
    }

    pub fn get_bindings_for_group(&self, group: &str) -> Vec<InteractionBind> {
        let cgroup = CString::new(group).expect("Failed to create CString for group");
        let mut count: i32 = 0;
        unsafe {
            let raw_bindings = f3d_interactor_get_binds_for_group(
                self.ptr.as_ptr(),
                cgroup.as_ptr(),
                &mut count as *mut i32,
            );

            let mut bindings = Vec::new();
            for i in 0..count {
                let raw_binding = *raw_bindings.add(i as usize);
                bindings.push(InteractionBind::from_raw(&raw_binding));
            }

            f3d_interactor_free_bind_array(raw_bindings);

            bindings
        }
    }

    pub fn get_binding_documentation(&self, bind: InteractionBind) -> BindingDocumentation {
        let bind_raw = bind.to_raw();
        unsafe {
            let mut raw_doc = f3d_binding_documentation_t {
                doc: [0; 512],
                value: [0; 256],
            };

            f3d_interactor_get_binding_documentation(
                self.ptr.as_ptr(),
                &bind_raw as *const f3d_interaction_bind_t,
                &mut raw_doc as *mut f3d_binding_documentation_t,
            );

            BindingDocumentation::from_raw(&raw_doc)
        }
    }

    pub fn get_binding_type(&self, bind: InteractionBind) -> InteractorBindType {
        let bind_raw = bind.to_raw();
        unsafe {
            let raw_type = f3d_interactor_get_binding_type(
                self.ptr.as_ptr(),
                &bind_raw as *const f3d_interaction_bind_t,
            );

            InteractorBindType::from_raw(raw_type)
        }
    }

    pub fn toggle_animation(&self, direction: InteractorAnimationDirection) {
        unsafe {
            f3d_interactor_toggle_animation(self.ptr.as_ptr(), direction.to_raw());
        }
    }

    pub fn start_animation(&self, direction: InteractorAnimationDirection) {
        unsafe {
            f3d_interactor_start_animation(self.ptr.as_ptr(), direction.to_raw());
        }
    }

    pub fn stop_animation(&self) {
        unsafe {
            f3d_interactor_stop_animation(self.ptr.as_ptr());
        }
    }

    pub fn is_playing_animation(&self) -> bool {
        unsafe { f3d_interactor_is_playing_animation(self.ptr.as_ptr()) != 0 }
    }

    pub fn get_animation_direction(&self) -> InteractorAnimationDirection {
        unsafe {
            let raw_dir = f3d_interactor_get_animation_direction(self.ptr.as_ptr());
            InteractorAnimationDirection::from_raw(raw_dir)
        }
    }

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

    pub fn trigger_mod_update(&self, modifier: InteractorInputModifier) {
        unsafe {
            f3d_interactor_trigger_mod_update(self.ptr.as_ptr(), modifier.to_raw());
        }
    }

    pub fn trigger_mouse_button(
        &self,
        action: InteractorInputaction,
        button: InteractorMouseButton,
    ) {
        unsafe {
            f3d_interactor_trigger_mouse_button(
                self.ptr.as_ptr(),
                action.to_raw(),
                button.to_raw(),
            );
        }
    }

    pub fn trigger_mouse_position(&self, x: f64, y: f64) {
        unsafe {
            f3d_interactor_trigger_mouse_position(self.ptr.as_ptr(), x, y);
        }
    }

    pub fn trigger_mouse_wheel(&self, direction: InteractorWheelDirection) {
        unsafe {
            f3d_interactor_trigger_mouse_wheel(self.ptr.as_ptr(), direction.to_raw());
        }
    }

    pub fn trigger_keyboard_key(&self, action: InteractorInputaction, key: char) {
        unsafe {
            f3d_interactor_trigger_keyboard_key(
                self.ptr.as_ptr(),
                action.to_raw(),
                &(key as i8) as *const i8 as *const _,
            );
        }
    }

    pub fn trigger_text_character(&self, codepoint: u32) {
        unsafe {
            f3d_interactor_trigger_text_character(self.ptr.as_ptr(), codepoint);
        }
    }

    pub fn trigger_event_loop(&self, delta_time: f64) {
        unsafe {
            f3d_interactor_trigger_event_loop(self.ptr.as_ptr(), delta_time);
        }
    }

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
