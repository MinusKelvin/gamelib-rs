#![allow(bad_style)]
#![allow(unused)]

use libc::{ c_int, c_char, c_float, c_double, c_uint, c_ushort, c_uchar, uint64_t, c_void };

pub const GLFW_VERSION_MAJOR: c_int = 3;
pub const GLFW_VERSION_MINOR: c_int = 3;
pub const GLFW_VERSION_REVISION: c_int = 0;

pub const GLFW_RELEASE: c_int = 0;
pub const GLFW_PRESS: c_int = 1;
pub const GLFW_REPEAT: c_int = 2;

pub const GLFW_HAT_CENTERED: c_int = 0;
pub const GLFW_HAT_UP: c_int = 1;
pub const GLFW_HAT_RIGHT: c_int = 2;
pub const GLFW_HAT_DOWN: c_int = 4;
pub const GLFW_HAT_LEFT: c_int = 8;
pub const GLFW_HAT_RIGHT_UP: c_int = (GLFW_HAT_RIGHT | GLFW_HAT_UP);
pub const GLFW_HAT_RIGHT_DOWN: c_int = (GLFW_HAT_RIGHT | GLFW_HAT_DOWN);
pub const GLFW_HAT_LEFT_UP: c_int = (GLFW_HAT_LEFT  | GLFW_HAT_UP);
pub const GLFW_HAT_LEFT_DOWN: c_int = (GLFW_HAT_LEFT  | GLFW_HAT_DOWN);

pub const GLFW_KEY_UNKNOWN: c_int = -1;

pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_APOSTROPHE: c_int = 39;
pub const GLFW_KEY_COMMA: c_int = 44;
pub const GLFW_KEY_MINUS: c_int = 45;
pub const GLFW_KEY_PERIOD: c_int = 46;
pub const GLFW_KEY_SLASH: c_int = 47;
pub const GLFW_KEY_0: c_int = 48;
pub const GLFW_KEY_1: c_int = 49;
pub const GLFW_KEY_2: c_int = 50;
pub const GLFW_KEY_3: c_int = 51;
pub const GLFW_KEY_4: c_int = 52;
pub const GLFW_KEY_5: c_int = 53;
pub const GLFW_KEY_6: c_int = 54;
pub const GLFW_KEY_7: c_int = 55;
pub const GLFW_KEY_8: c_int = 56;
pub const GLFW_KEY_9: c_int = 57;
pub const GLFW_KEY_SEMICOLON: c_int = 59;
pub const GLFW_KEY_EQUAL: c_int = 61;
pub const GLFW_KEY_A: c_int = 65;
pub const GLFW_KEY_B: c_int = 66;
pub const GLFW_KEY_C: c_int = 67;
pub const GLFW_KEY_D: c_int = 68;
pub const GLFW_KEY_E: c_int = 69;
pub const GLFW_KEY_F: c_int = 70;
pub const GLFW_KEY_G: c_int = 71;
pub const GLFW_KEY_H: c_int = 72;
pub const GLFW_KEY_I: c_int = 73;
pub const GLFW_KEY_J: c_int = 74;
pub const GLFW_KEY_K: c_int = 75;
pub const GLFW_KEY_L: c_int = 76;
pub const GLFW_KEY_M: c_int = 77;
pub const GLFW_KEY_N: c_int = 78;
pub const GLFW_KEY_O: c_int = 79;
pub const GLFW_KEY_P: c_int = 80;
pub const GLFW_KEY_Q: c_int = 81;
pub const GLFW_KEY_R: c_int = 82;
pub const GLFW_KEY_S: c_int = 83;
pub const GLFW_KEY_T: c_int = 84;
pub const GLFW_KEY_U: c_int = 85;
pub const GLFW_KEY_V: c_int = 86;
pub const GLFW_KEY_W: c_int = 87;
pub const GLFW_KEY_X: c_int = 88;
pub const GLFW_KEY_Y: c_int = 89;
pub const GLFW_KEY_Z: c_int = 90;
pub const GLFW_KEY_LEFT_BRACKET: c_int = 91;
pub const GLFW_KEY_BACKSLASH: c_int = 92;
pub const GLFW_KEY_RIGHT_BRACKET: c_int = 93;
pub const GLFW_KEY_GRAVE_ACCENT: c_int = 96;
pub const GLFW_KEY_WORLD_1: c_int = 161;
pub const GLFW_KEY_WORLD_2: c_int = 162;

pub const GLFW_KEY_ESCAPE: c_int = 256;
pub const GLFW_KEY_ENTER: c_int = 257;
pub const GLFW_KEY_TAB: c_int = 258;
pub const GLFW_KEY_BACKSPACE: c_int = 259;
pub const GLFW_KEY_INSERT: c_int = 260;
pub const GLFW_KEY_DELETE: c_int = 261;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;
pub const GLFW_KEY_PAGE_UP: c_int = 266;
pub const GLFW_KEY_PAGE_DOWN: c_int = 267;
pub const GLFW_KEY_HOME: c_int = 268;
pub const GLFW_KEY_END: c_int = 269;
pub const GLFW_KEY_CAPS_LOCK: c_int = 280;
pub const GLFW_KEY_SCROLL_LOCK: c_int = 281;
pub const GLFW_KEY_NUM_LOCK: c_int = 282;
pub const GLFW_KEY_PRINT_SCREEN: c_int = 283;
pub const GLFW_KEY_PAUSE: c_int = 284;
pub const GLFW_KEY_F1: c_int = 290;
pub const GLFW_KEY_F2: c_int = 291;
pub const GLFW_KEY_F3: c_int = 292;
pub const GLFW_KEY_F4: c_int = 293;
pub const GLFW_KEY_F5: c_int = 294;
pub const GLFW_KEY_F6: c_int = 295;
pub const GLFW_KEY_F7: c_int = 296;
pub const GLFW_KEY_F8: c_int = 297;
pub const GLFW_KEY_F9: c_int = 298;
pub const GLFW_KEY_F10: c_int = 299;
pub const GLFW_KEY_F11: c_int = 300;
pub const GLFW_KEY_F12: c_int = 301;
pub const GLFW_KEY_F13: c_int = 302;
pub const GLFW_KEY_F14: c_int = 303;
pub const GLFW_KEY_F15: c_int = 304;
pub const GLFW_KEY_F16: c_int = 305;
pub const GLFW_KEY_F17: c_int = 306;
pub const GLFW_KEY_F18: c_int = 307;
pub const GLFW_KEY_F19: c_int = 308;
pub const GLFW_KEY_F20: c_int = 309;
pub const GLFW_KEY_F21: c_int = 310;
pub const GLFW_KEY_F22: c_int = 311;
pub const GLFW_KEY_F23: c_int = 312;
pub const GLFW_KEY_F24: c_int = 313;
pub const GLFW_KEY_F25: c_int = 314;
pub const GLFW_KEY_KP_0: c_int = 320;
pub const GLFW_KEY_KP_1: c_int = 321;
pub const GLFW_KEY_KP_2: c_int = 322;
pub const GLFW_KEY_KP_3: c_int = 323;
pub const GLFW_KEY_KP_4: c_int = 324;
pub const GLFW_KEY_KP_5: c_int = 325;
pub const GLFW_KEY_KP_6: c_int = 326;
pub const GLFW_KEY_KP_7: c_int = 327;
pub const GLFW_KEY_KP_8: c_int = 328;
pub const GLFW_KEY_KP_9: c_int = 329;
pub const GLFW_KEY_KP_DECIMAL: c_int = 330;
pub const GLFW_KEY_KP_DIVIDE: c_int = 331;
pub const GLFW_KEY_KP_MULTIPLY: c_int = 332;
pub const GLFW_KEY_KP_SUBTRACT: c_int = 333;
pub const GLFW_KEY_KP_ADD: c_int = 334;
pub const GLFW_KEY_KP_ENTER: c_int = 335;
pub const GLFW_KEY_KP_EQUAL: c_int = 336;
pub const GLFW_KEY_LEFT_SHIFT: c_int = 340;
pub const GLFW_KEY_LEFT_CONTROL: c_int = 341;
pub const GLFW_KEY_LEFT_ALT: c_int = 342;
pub const GLFW_KEY_LEFT_SUPER: c_int = 343;
pub const GLFW_KEY_RIGHT_SHIFT: c_int = 344;
pub const GLFW_KEY_RIGHT_CONTROL: c_int = 345;
pub const GLFW_KEY_RIGHT_ALT: c_int = 346;
pub const GLFW_KEY_RIGHT_SUPER: c_int = 347;
pub const GLFW_KEY_MENU: c_int = 348;

pub const GLFW_KEY_LAST: c_int = GLFW_KEY_MENU;

pub const GLFW_MOD_SHIFT: c_int = 0x0001;
pub const GLFW_MOD_CONTROL: c_int = 0x0002;
pub const GLFW_MOD_ALT: c_int = 0x0004;
pub const GLFW_MOD_SUPER: c_int = 0x0008;
pub const GLFW_MOD_CAPS_LOCK: c_int = 0x0010;
pub const GLFW_MOD_NUM_LOCK: c_int = 0x0020;

pub const GLFW_MOUSE_BUTTON_1: c_int = 0;
pub const GLFW_MOUSE_BUTTON_2: c_int = 1;
pub const GLFW_MOUSE_BUTTON_3: c_int = 2;
pub const GLFW_MOUSE_BUTTON_4: c_int = 3;
pub const GLFW_MOUSE_BUTTON_5: c_int = 4;
pub const GLFW_MOUSE_BUTTON_6: c_int = 5;
pub const GLFW_MOUSE_BUTTON_7: c_int = 6;
pub const GLFW_MOUSE_BUTTON_8: c_int = 7;
pub const GLFW_MOUSE_BUTTON_LAST: c_int = GLFW_MOUSE_BUTTON_8;
pub const GLFW_MOUSE_BUTTON_LEFT: c_int = GLFW_MOUSE_BUTTON_1;
pub const GLFW_MOUSE_BUTTON_RIGHT: c_int = GLFW_MOUSE_BUTTON_2;
pub const GLFW_MOUSE_BUTTON_MIDDLE: c_int = GLFW_MOUSE_BUTTON_3;

pub const GLFW_JOYSTICK_1: c_int = 0;
pub const GLFW_JOYSTICK_2: c_int = 1;
pub const GLFW_JOYSTICK_3: c_int = 2;
pub const GLFW_JOYSTICK_4: c_int = 3;
pub const GLFW_JOYSTICK_5: c_int = 4;
pub const GLFW_JOYSTICK_6: c_int = 5;
pub const GLFW_JOYSTICK_7: c_int = 6;
pub const GLFW_JOYSTICK_8: c_int = 7;
pub const GLFW_JOYSTICK_9: c_int = 8;
pub const GLFW_JOYSTICK_10: c_int = 9;
pub const GLFW_JOYSTICK_11: c_int = 10;
pub const GLFW_JOYSTICK_12: c_int = 11;
pub const GLFW_JOYSTICK_13: c_int = 12;
pub const GLFW_JOYSTICK_14: c_int = 13;
pub const GLFW_JOYSTICK_15: c_int = 14;
pub const GLFW_JOYSTICK_16: c_int = 15;
pub const GLFW_JOYSTICK_LAST: c_int = GLFW_JOYSTICK_16;

pub const GLFW_GAMEPAD_BUTTON_A: c_int = 0;
pub const GLFW_GAMEPAD_BUTTON_B: c_int = 1;
pub const GLFW_GAMEPAD_BUTTON_X: c_int = 2;
pub const GLFW_GAMEPAD_BUTTON_Y: c_int = 3;
pub const GLFW_GAMEPAD_BUTTON_LEFT_BUMPER: c_int = 4;
pub const GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER: c_int = 5;
pub const GLFW_GAMEPAD_BUTTON_BACK: c_int = 6;
pub const GLFW_GAMEPAD_BUTTON_START: c_int = 7;
pub const GLFW_GAMEPAD_BUTTON_GUIDE: c_int = 8;
pub const GLFW_GAMEPAD_BUTTON_LEFT_THUMB: c_int = 9;
pub const GLFW_GAMEPAD_BUTTON_RIGHT_THUMB: c_int = 10;
pub const GLFW_GAMEPAD_BUTTON_DPAD_UP: c_int = 11;
pub const GLFW_GAMEPAD_BUTTON_DPAD_RIGHT: c_int = 12;
pub const GLFW_GAMEPAD_BUTTON_DPAD_DOWN: c_int = 13;
pub const GLFW_GAMEPAD_BUTTON_DPAD_LEFT: c_int = 14;
pub const GLFW_GAMEPAD_BUTTON_LAST: c_int = GLFW_GAMEPAD_BUTTON_DPAD_LEFT;

pub const GLFW_GAMEPAD_BUTTON_CROSS: c_int = GLFW_GAMEPAD_BUTTON_A;
pub const GLFW_GAMEPAD_BUTTON_CIRCLE: c_int = GLFW_GAMEPAD_BUTTON_B;
pub const GLFW_GAMEPAD_BUTTON_SQUARE: c_int = GLFW_GAMEPAD_BUTTON_X;
pub const GLFW_GAMEPAD_BUTTON_TRIANGLE: c_int = GLFW_GAMEPAD_BUTTON_Y;

pub const GLFW_GAMEPAD_AXIS_LEFT_X: c_int = 0;
pub const GLFW_GAMEPAD_AXIS_LEFT_Y: c_int = 1;
pub const GLFW_GAMEPAD_AXIS_RIGHT_X: c_int = 2;
pub const GLFW_GAMEPAD_AXIS_RIGHT_Y: c_int = 3;
pub const GLFW_GAMEPAD_AXIS_LEFT_TRIGGER: c_int = 4;
pub const GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER: c_int = 5;
pub const GLFW_GAMEPAD_AXIS_LAST: c_int = GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER;

pub const GLFW_NO_ERROR: c_int = 0;
pub const GLFW_NOT_INITIALIZED: c_int = 0x00010001;
pub const GLFW_NO_CURRENT_CONTEXT: c_int = 0x00010002;
pub const GLFW_INVALID_ENUM: c_int = 0x00010003;
pub const GLFW_INVALID_VALUE: c_int = 0x00010004;
pub const GLFW_OUT_OF_MEMORY: c_int = 0x00010005;
pub const GLFW_API_UNAVAILABLE: c_int = 0x00010006;
pub const GLFW_VERSION_UNAVAILABLE: c_int = 0x00010007;
pub const GLFW_PLATFORM_ERROR: c_int = 0x00010008;
pub const GLFW_FORMAT_UNAVAILABLE: c_int = 0x00010009;
pub const GLFW_NO_WINDOW_CONTEXT: c_int = 0x0001000A;

pub const GLFW_FOCUSED: c_int = 0x00020001;
pub const GLFW_ICONIFIED: c_int = 0x00020002;
pub const GLFW_RESIZABLE: c_int = 0x00020003;
pub const GLFW_VISIBLE: c_int = 0x00020004;
pub const GLFW_DECORATED: c_int = 0x00020005;
pub const GLFW_AUTO_ICONIFY: c_int = 0x00020006;
pub const GLFW_FLOATING: c_int = 0x00020007;
pub const GLFW_MAXIMIZED: c_int = 0x00020008;
pub const GLFW_CENTER_CURSOR: c_int = 0x00020009;
pub const GLFW_TRANSPARENT_FRAMEBUFFER: c_int = 0x0002000A;
pub const GLFW_HOVERED: c_int = 0x0002000B;
pub const GLFW_FOCUS_ON_SHOW: c_int = 0x0002000C;

pub const GLFW_RED_BITS: c_int = 0x00021001;
pub const GLFW_GREEN_BITS: c_int = 0x00021002;
pub const GLFW_BLUE_BITS: c_int = 0x00021003;
pub const GLFW_ALPHA_BITS: c_int = 0x00021004;
pub const GLFW_DEPTH_BITS: c_int = 0x00021005;
pub const GLFW_STENCIL_BITS: c_int = 0x00021006;
pub const GLFW_ACCUM_RED_BITS: c_int = 0x00021007;
pub const GLFW_ACCUM_GREEN_BITS: c_int = 0x00021008;
pub const GLFW_ACCUM_BLUE_BITS: c_int = 0x00021009;
pub const GLFW_ACCUM_ALPHA_BITS: c_int = 0x0002100A;
pub const GLFW_AUX_BUFFERS: c_int = 0x0002100B;
pub const GLFW_STEREO: c_int = 0x0002100C;
pub const GLFW_SAMPLES: c_int = 0x0002100D;
pub const GLFW_SRGB_CAPABLE: c_int = 0x0002100E;
pub const GLFW_REFRESH_RATE: c_int = 0x0002100F;
pub const GLFW_DOUBLEBUFFER: c_int = 0x00021010;

pub const GLFW_CLIENT_API: c_int = 0x00022001;
pub const GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
pub const GLFW_CONTEXT_VERSION_MINOR: c_int = 0x00022003;
pub const GLFW_CONTEXT_REVISION: c_int = 0x00022004;
pub const GLFW_CONTEXT_ROBUSTNESS: c_int = 0x00022005;
pub const GLFW_OPENGL_FORWARD_COMPAT: c_int = 0x00022006;
pub const GLFW_OPENGL_DEBUG_CONTEXT: c_int = 0x00022007;
pub const GLFW_OPENGL_PROFILE: c_int = 0x00022008;
pub const GLFW_CONTEXT_RELEASE_BEHAVIOR: c_int = 0x00022009;
pub const GLFW_CONTEXT_NO_ERROR: c_int = 0x0002200A;
pub const GLFW_CONTEXT_CREATION_API: c_int = 0x0002200B;

pub const GLFW_COCOA_RETINA_FRAMEBUFFER: c_int = 0x00023001;
pub const GLFW_COCOA_FRAME_NAME: c_int = 0x00023002;
pub const GLFW_COCOA_GRAPHICS_SWITCHING: c_int = 0x00023003;

pub const GLFW_X11_CLASS_NAME: c_int = 0x00024001;
pub const GLFW_X11_INSTANCE_NAME: c_int = 0x00024002;

pub const GLFW_NO_API: c_int = 0;
pub const GLFW_OPENGL_API: c_int = 0x00030001;
pub const GLFW_OPENGL_ES_API: c_int = 0x00030002;

pub const GLFW_NO_ROBUSTNESS: c_int = 0;
pub const GLFW_NO_RESET_NOTIFICATION: c_int = 0x00031001;
pub const GLFW_LOSE_CONTEXT_ON_RESET: c_int = 0x00031002;

pub const GLFW_OPENGL_ANY_PROFILE: c_int = 0;
pub const GLFW_OPENGL_CORE_PROFILE: c_int = 0x00032001;
pub const GLFW_OPENGL_COMPAT_PROFILE: c_int = 0x00032002;

pub const GLFW_CURSOR: c_int = 0x00033001;
pub const GLFW_STICKY_KEYS: c_int = 0x00033002;
pub const GLFW_STICKY_MOUSE_BUTTONS: c_int = 0x00033003;
pub const GLFW_LOCK_KEY_MODS: c_int = 0x00033004;

pub const GLFW_CURSOR_NORMAL: c_int = 0x00034001;
pub const GLFW_CURSOR_HIDDEN: c_int = 0x00034002;
pub const GLFW_CURSOR_DISABLED: c_int = 0x00034003;

pub const GLFW_ANY_RELEASE_BEHAVIOR: c_int = 0;
pub const GLFW_RELEASE_BEHAVIOR_FLUSH: c_int = 0x00035001;
pub const GLFW_RELEASE_BEHAVIOR_NONE: c_int = 0x00035002;

pub const GLFW_NATIVE_CONTEXT_API: c_int = 0x00036001;
pub const GLFW_EGL_CONTEXT_API: c_int = 0x00036002;
pub const GLFW_OSMESA_CONTEXT_API: c_int = 0x00036003;

pub const GLFW_ARROW_CURSOR: c_int = 0x00036001;
pub const GLFW_IBEAM_CURSOR: c_int = 0x00036002;
pub const GLFW_CROSSHAIR_CURSOR: c_int = 0x00036003;
pub const GLFW_HAND_CURSOR: c_int = 0x00036004;
pub const GLFW_HRESIZE_CURSOR: c_int = 0x00036005;
pub const GLFW_VRESIZE_CURSOR: c_int = 0x00036006;

pub const GLFW_CONNECTED: c_int = 0x00040001;
pub const GLFW_DISCONNECTED: c_int = 0x00040002;

pub const GLFW_JOYSTICK_HAT_BUTTONS: c_int = 0x00050001;

pub const GLFW_COCOA_CHDIR_RESOURCES: c_int = 0x00051001;
pub const GLFW_COCOA_MENUBAR: c_int = 0x00051002;

pub const GLFW_DONT_CARE: c_int = -1;

// pub type GLFWglproc = extern "C" fn();
pub type GLFWglproc = *const ::std::os::raw::c_void;

pub type GLFWerrorfun = extern "C" fn(c_int, *const c_char);
pub type GLFWwindowposfun = extern "C" fn(*mut GLFWwindow, c_int, c_int);
pub type GLFWwindowsizefun = extern "C" fn(*mut GLFWwindow,c_int,c_int);
pub type GLFWwindowclosefun = extern "C" fn(*mut GLFWwindow);
pub type GLFWwindowrefreshfun = extern "C" fn(*mut GLFWwindow);
pub type GLFWwindowfocusfun = extern "C" fn(*mut GLFWwindow,c_int);
pub type GLFWwindowiconifyfun = extern "C" fn(*mut GLFWwindow,c_int);
pub type GLFWwindowmaximizefun = extern "C" fn(*mut GLFWwindow,c_int);
pub type GLFWframebuffersizefun = extern "C" fn(*mut GLFWwindow,c_int,c_int);
pub type GLFWwindowcontentscalefun = extern "C" fn(*mut GLFWwindow,c_float,c_float);
pub type GLFWmousebuttonfun = extern "C" fn(*mut GLFWwindow,c_int,c_int,c_int);
pub type GLFWcursorposfun = extern "C" fn(*mut GLFWwindow,c_double,c_double);
pub type GLFWcursorenterfun = extern "C" fn(*mut GLFWwindow,c_int);
pub type GLFWscrollfun = extern "C" fn(*mut GLFWwindow,c_double,c_double);
pub type GLFWkeyfun = extern "C" fn(*mut GLFWwindow,c_int,c_int,c_int,c_int);
pub type GLFWcharfun = extern "C" fn(*mut GLFWwindow,c_uint);
pub type GLFWcharmodsfun = extern "C" fn(*mut GLFWwindow,c_uint,c_int);
pub type GLFWdropfun = extern "C" fn(*mut GLFWwindow,c_int,*const *const c_char);
pub type GLFWmonitorfun = extern "C" fn(*mut GLFWmonitor,c_int);
pub type GLFWjoystickfun = extern "C" fn(c_int,c_int);

pub enum GLFWmonitor {}
pub enum GLFWwindow {}
pub enum GLFWcursor {}

#[repr(C)]
pub struct GLFWvidmode {
    pub width: c_int,
    pub height: c_int,
    pub redBits: c_int,
    pub greenBits: c_int,
    pub blueBits: c_int,
    pub refreshRate: c_int
}

#[repr(C)]
pub struct GLFWgammaramp {
    pub red: *mut c_ushort,
    pub green: *mut c_ushort,
    pub blue: *mut c_ushort,
    pub size: c_uint
}

#[repr(C)]
pub struct GLFWimage {
    width: c_int,
    height: c_int,
    pixels: *mut c_uchar
}

#[repr(C)]
pub struct GLFWgamepadstate {
    buttons: [c_uchar; 15],
    axes: [c_float; 6]
}

#[link(name = "glfw3", kind = "static")]
extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwInitHint(hint: c_int, value: c_int);
    pub fn glfwGetVersion(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    pub fn glfwGetVersionString() -> *const c_char;
    pub fn glfwGetError(description: *mut *const c_char) -> c_int;
    pub fn glfwSetErrorCallback(cbfun: Option<GLFWerrorfun>) -> Option<GLFWerrorfun>;
    pub fn glfwGetMonitors(count: *mut c_int) -> *const *mut GLFWmonitor;
    pub fn glfwGetPrimaryMonitor() -> *mut GLFWmonitor;
    pub fn glfwGetMonitorPos(monitor: *mut GLFWmonitor, xpos: *mut c_int, ypos: *mut c_int);
    pub fn glfwGetMonitorPhysicalSize(monitor: *mut GLFWmonitor, widthMM: *mut c_int, heightMM: *mut c_int);
    pub fn glfwGetMonitorContentScale(monitor: *mut GLFWmonitor, xscale: *mut c_float, yscale: *mut c_float);
    pub fn glfwGetMonitorName(monitor: *mut GLFWmonitor) -> *const c_char;
    pub fn glfwSetMonitorUserPointer(monitor: *mut GLFWmonitor, pointer: *mut c_void);
    pub fn glfwGetMonitorUserPointer(monitor: *mut GLFWmonitor) -> *mut c_void;
    pub fn glfwSetMonitorCallback(cbfun: Option<GLFWmonitorfun>) -> Option<GLFWmonitorfun>;
    pub fn glfwGetVideoModes(monitor: *mut GLFWmonitor, count: *mut c_int) -> *const GLFWvidmode;
    pub fn glfwGetVideoMode(monitor: *mut GLFWmonitor) -> *const GLFWvidmode;
    pub fn glfwSetGamma(monitor: *mut GLFWmonitor, gamma: c_float);
    pub fn glfwGetGammaRamp(monitor: *mut GLFWmonitor) -> *const GLFWgammaramp;
    pub fn glfwSetGammaRamp(monitor: *mut GLFWmonitor, ramp: *const GLFWgammaramp);
    pub fn glfwDefaultWindowHints();
    pub fn glfwWindowHint(hint: c_int, value: c_int);
    pub fn glfwWindowHintString(hint: c_int, value: *mut c_char);
    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSetWindowShouldClose(window: *mut GLFWwindow, value: c_int);
    pub fn glfwSetWindowTitle(window: *mut GLFWwindow, title: *mut c_char);
    pub fn glfwSetWindowIcon(window: *mut GLFWwindow, count: c_int, images: *const GLFWimage);
    pub fn glfwGetWindowPos(window: *mut GLFWwindow, xpos: *mut c_int, ypos: *mut c_int);
    pub fn glfwSetWindowPos(window: *mut GLFWwindow, xpos: c_int, ypos: c_int);
    pub fn glfwGetWindowSize(window: *mut GLFWwindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwSetWindowSizeLimits(window: *mut GLFWwindow, minwidth: c_int, minheight: c_int, maxwidth: c_int, maxheight: c_int);
    pub fn glfwSetWindowAspectRatio(window: *mut GLFWwindow, numer: c_int, denom: c_int);
    pub fn glfwSetWindowSize(window: *mut GLFWwindow, width: c_int, height: c_int);
    pub fn glfwGetFramebufferSize(window: *mut GLFWwindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwGetWindowFrameSize(window: *mut GLFWwindow, left: *mut c_int, top: *mut c_int, right: *mut c_int, bottom: *mut c_int);
    pub fn glfwGetWindowContentScale(window: *mut GLFWwindow, xscale: *mut c_float, yscale: *mut c_float);
    pub fn glfwGetWindowOpacity(window: *mut GLFWwindow) -> c_float;
    pub fn glfwSetWindowOpacity(window: *mut GLFWwindow, opacity: c_float);
    pub fn glfwIconifyWindow(window: *mut GLFWwindow);
    pub fn glfwRestoreWindow(window: *mut GLFWwindow);
    pub fn glfwMaximizeWindow(window: *mut GLFWwindow);
    pub fn glfwShowWindow(window: *mut GLFWwindow);
    pub fn glfwHideWindow(window: *mut GLFWwindow);
    pub fn glfwFocusWindow(window: *mut GLFWwindow);
    pub fn glfwRequestWindowAttention(window: *mut GLFWwindow);
    pub fn glfwGetWindowMonitor(window: *mut GLFWwindow) -> *mut GLFWmonitor;
    pub fn glfwSetWindowMonitor(window: *mut GLFWwindow, monitor: *mut GLFWmonitor, xpos: c_int, ypos: c_int, width: c_int, height: c_int, refreshRate: c_int);
    pub fn glfwGetWindowAttrib(window: *mut GLFWwindow, attrib: c_int) -> c_int;
    pub fn glfwSetWindowAttrib(window: *mut GLFWwindow, attrib: c_int, value: c_int);
    pub fn glfwSetWindowUserPointer(window: *mut GLFWwindow, pointer: *mut c_void);
    pub fn glfwGetWindowUserPointer(window: *mut GLFWwindow) -> *mut c_void;
    pub fn glfwSetWindowPosCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowposfun>) -> Option<GLFWwindowposfun>;
    pub fn glfwSetWindowSizeCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowsizefun>) -> Option<GLFWwindowsizefun>;
    pub fn glfwSetWindowCloseCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowclosefun>) -> Option<GLFWwindowclosefun>;
    pub fn glfwSetWindowRefreshCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowrefreshfun>) -> Option<GLFWwindowrefreshfun>;
    pub fn glfwSetWindowFocusCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowfocusfun>) -> Option<GLFWwindowfocusfun>;
    pub fn glfwSetWindowIconifyCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowiconifyfun>) -> Option<GLFWwindowiconifyfun>;
    pub fn glfwSetWindowMaximizeCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowmaximizefun>) -> Option<GLFWwindowmaximizefun>;
    pub fn glfwSetFramebufferSizeCallback(window: *mut GLFWwindow, cbfun: Option<GLFWframebuffersizefun>) -> Option<GLFWframebuffersizefun>;
    pub fn glfwSetWindowContentScaleCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowcontentscalefun>) -> Option<GLFWwindowcontentscalefun>;
    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();
    pub fn glfwWaitEventsTimeout(timeout: c_double);
    pub fn glfwPostEmptyEvent();
    pub fn glfwGetInputMode(window: *mut GLFWwindow, mode: c_int) -> c_int;
    pub fn glfwSetInputMode(window: *mut GLFWwindow, mode: c_int, value: c_int);
    pub fn glfwGetKeyName(key: c_int, scancode: c_int) -> *const c_char;
    pub fn glfwGetKeyScancode(key: c_int) -> c_int;
    pub fn glfwGetKey(window: *mut GLFWwindow, key: c_int) -> c_int;
    pub fn glfwGetMouseButton(window: *mut GLFWwindow, button: c_int) -> c_int;
    pub fn glfwGetCursorPos(window: *mut GLFWwindow, xpos: *mut c_double, ypos: *mut c_double);
    pub fn glfwSetCursorPos(window: *mut GLFWwindow, xpos: c_double, ypos: c_double);
    pub fn glfwCreateCursor(image: *const GLFWimage, xhot: c_int, yhot: c_int) -> *mut GLFWcursor;
    pub fn glfwCreateStandardCursor(shape: c_int) -> *mut GLFWcursor;
    pub fn glfwDestroyCursor(cursor: *mut GLFWcursor);
    pub fn glfwSetCursor(window: *mut GLFWwindow, cursor: *mut GLFWcursor);
    pub fn glfwSetKeyCallback(window: *mut GLFWwindow, cbfun: Option<GLFWkeyfun>) -> Option<GLFWkeyfun>;
    pub fn glfwSetCharCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcharfun>) -> Option<GLFWcharfun>;
    pub fn glfwSetCharModsCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcharmodsfun>) -> Option<GLFWcharmodsfun>;
    pub fn glfwSetMouseButtonCallback(window: *mut GLFWwindow, cbfun: Option<GLFWmousebuttonfun>) -> Option<GLFWmousebuttonfun>;
    pub fn glfwSetCursorPosCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcursorposfun>) -> Option<GLFWcursorposfun>;
    pub fn glfwSetCursorEnterCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcursorenterfun>) -> Option<GLFWcursorenterfun>;
    pub fn glfwSetScrollCallback(window: *mut GLFWwindow, cbfun: Option<GLFWscrollfun>) -> Option<GLFWscrollfun>;
    pub fn glfwSetDropCallback(window: *mut GLFWwindow, cbfun: Option<GLFWdropfun>) -> Option<GLFWdropfun>;
    pub fn glfwJoystickPresent(jid: c_int) -> c_int;
    pub fn glfwGetJoystickAxes(jid: c_int, count: *mut c_int) -> *const c_float;
    pub fn glfwGetJoystickButtons(jid: c_int, count: *mut c_int) -> *const c_uchar;
    pub fn glfwGetJoystickHats(jid: c_int, count: *mut c_int) -> *const c_uchar;
    pub fn glfwGetJoystickName(jid: c_int) -> *const c_char;
    pub fn glfwGetJoystickGUID(jid: c_int) -> *const c_char;
    pub fn glfwSetJoystickUserPointer(jid: c_int, pointer: *mut c_void);
    pub fn glfwGetJoystickUserPointer(jid: c_int) -> *mut c_void;
    pub fn glfwJoystickIsGamepad(jid: c_int) -> c_int;
    pub fn glfwSetJoystickCallback(cbfun: Option<GLFWjoystickfun>) -> Option<GLFWjoystickfun>;
    pub fn glfwUpdateGamepadMappings(string: *mut c_char) -> c_int;
    pub fn glfwGetGamepadName(jid: c_int) -> *const c_char;
    pub fn glfwGetGamepadState(jid: c_int, state: *mut GLFWgamepadstate) -> c_int;
    pub fn glfwSetClipboardString(window: *mut GLFWwindow, string: *mut c_char);
    pub fn glfwGetClipboardString(window: *mut GLFWwindow) -> *const c_char;
    pub fn glfwGetTime() -> c_double;
    pub fn glfwSetTime(time: c_double);
    pub fn glfwGetTimerValue() -> uint64_t;
    pub fn glfwGetTimerFrequency() -> uint64_t;
    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwGetCurrentContext() -> *mut GLFWwindow;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwSwapInterval(interval: c_int);
    pub fn glfwExtensionSupported(extension: *const c_char) -> c_int;
    pub fn glfwGetProcAddress(procname: *const c_char) -> GLFWglproc;
}

#[cfg(target_os="windows")]
#[link(name = "opengl32")]
#[link(name = "gdi32")]
#[link(name = "user32")]
extern {}

#[cfg(any(target_os="linux", target_os="freebsd", target_os="dragonfly"))]
#[link(name = "X11")]
#[link(name = "GL")]
#[link(name = "Xxf86vm")]
#[link(name = "Xrandr")]
#[link(name = "Xi")]
#[link(name = "Xcursor")]
#[link(name = "Xinerama")]
extern {}

#[cfg(target_os="macos")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "OpenGL", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
extern {}
