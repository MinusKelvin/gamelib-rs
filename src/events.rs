use std::mem;
use std::char;

use libc::{ c_int, c_float, c_double, c_uint };

use ffi::*;
use Game;

pub enum Event {
    Close,
    Focus,
    Unfocus,
    Resize(i32, i32),
    ContentScaleChange(f32, f32),
    Minimize,
    Unminimize,
    MousePress(MouseButton, Modifiers),
    MouseRelease(MouseButton, Modifiers),
    MousePos(i32, i32),
    Scroll(f64, f64),
    KeyPress(Option<Key>, Modifiers, i32),
    KeyRepeat(Option<Key>, Modifiers, i32),
    KeyRelease(Option<Key>, Modifiers, i32),
    Character(char, Modifiers),
    // Files(Vec<String>),
    // GamepadConnected(GamepadHandle),
    // GamepadDisconnected(GamepadHandle)
}

enum InternalEvent {
    User(Event),
    Refresh
}

pub(crate) struct Target<'a> {
    pub game: Box<Game + 'a>,
    pub ctx: &'a ::gfx::Context,
    pub queue: Vec<Event>,
    pub polling: bool,
    pub width: i32,
    pub height: i32,
}

fn send(window: *mut GLFWwindow, event: InternalEvent) {
    let p = unsafe { glfwGetWindowUserPointer(window) };
    if p.is_null() { return; }
    let target = unsafe { &mut *(p as *mut Target) };
    if target.polling {
        match event {
            InternalEvent::User(e) => {
                if let Event::Resize(width, height) = e {
                    target.width = width;
                    target.height = height;
                }
                target.game.event(e);
            },
            InternalEvent::Refresh => {
                target.game.frame(target.ctx.create_screen_surface(target.width, target.height), 0.0);
                unsafe { glfwSwapBuffers(window) };
            }
        }
    } else if let InternalEvent::User(e) = event {
        target.queue.push(e);
    }
}

pub extern "C" fn window_refresh(window: *mut GLFWwindow) {
    send(window, InternalEvent::Refresh);
}

pub extern "C" fn window_close(window: *mut GLFWwindow) {
    send(window, InternalEvent::User(Event::Close));
}

pub extern "C" fn window_focus(window: *mut GLFWwindow, focused: c_int) {
    send(window, InternalEvent::User(if focused == 1 {
        Event::Focus
    } else {
        Event::Unfocus
    }));
}

pub extern "C" fn framebuffer_size(window: *mut GLFWwindow, width: c_int, height: c_int) {
    send(window, InternalEvent::User(Event::Resize(width, height)));
}

pub extern "C" fn content_scale_change(window: *mut GLFWwindow, x: c_float, y: c_float) {
    send(window, InternalEvent::User(Event::ContentScaleChange(x, y)));
}

pub extern "C" fn window_iconify(window: *mut GLFWwindow, iconified: c_int) {
    send(window, InternalEvent::User(if iconified != 0 {
        Event::Minimize
    } else {
        Event::Unminimize
    }));
}

pub extern "C" fn mouse_button(window: *mut GLFWwindow, button: c_int, action: c_int, mods: c_int) {
    send(window, InternalEvent::User(if action != 0 {
        Event::MousePress(unsafe { mem::transmute(button) }, Modifiers::from_bits(mods).unwrap())
    } else {
        Event::MouseRelease(unsafe { mem::transmute(button) }, Modifiers::from_bits(mods).unwrap())
    }));
}

pub extern "C" fn mouse_pos(window: *mut GLFWwindow, x: c_double, y: c_double) {
    send(window, InternalEvent::User(Event::MousePos(x as i32, y as i32)));
}

pub extern "C" fn scroll(window: *mut GLFWwindow, x: c_double, y: c_double) {
    send(window, InternalEvent::User(Event::Scroll(x, y)));
}

pub extern "C" fn key(window: *mut GLFWwindow, key: c_int, scancode: c_int, action: c_int, mods: c_int) {
    let key = if key == GLFW_KEY_UNKNOWN { None } else { Some(unsafe { mem::transmute(key) }) };
    send(window, InternalEvent::User(match action {
        GLFW_PRESS => Event::KeyPress(key, Modifiers::from_bits(mods).unwrap(), scancode),
        GLFW_RELEASE => Event::KeyRelease(key, Modifiers::from_bits(mods).unwrap(), scancode),
        GLFW_REPEAT => Event::KeyRepeat(key, Modifiers::from_bits(mods).unwrap(), scancode),
        _ => unreachable!()
    }));
}

pub extern "C" fn char_mods(window: *mut GLFWwindow, codepoint: c_uint, mods: c_int) {
    send(window, InternalEvent::User(Event::Character(char::from_u32(codepoint).unwrap_or(char::REPLACEMENT_CHARACTER), Modifiers::from_bits(mods).unwrap())));
}

// pub struct GamepadHandle {
//     jid: c_int
// }

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum MouseButton {
        Left = GLFW_MOUSE_BUTTON_LEFT,
        Right = GLFW_MOUSE_BUTTON_RIGHT,
        Middle = GLFW_MOUSE_BUTTON_MIDDLE,
        Four = GLFW_MOUSE_BUTTON_4,
        Five = GLFW_MOUSE_BUTTON_5,
        Six = GLFW_MOUSE_BUTTON_6,
        Seven = GLFW_MOUSE_BUTTON_7,
        Eight = GLFW_MOUSE_BUTTON_8
    }
}
enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum Key {
        Unknown = GLFW_KEY_UNKNOWN,
        Space = GLFW_KEY_SPACE,
        Apostrophe = GLFW_KEY_APOSTROPHE,
        Comma = GLFW_KEY_COMMA,
        Minus = GLFW_KEY_MINUS,
        Period = GLFW_KEY_PERIOD,
        Slash = GLFW_KEY_SLASH,
        Zero = GLFW_KEY_0,
        One = GLFW_KEY_1,
        Two = GLFW_KEY_2,
        Three = GLFW_KEY_3,
        Four = GLFW_KEY_4,
        Five = GLFW_KEY_5,
        Six = GLFW_KEY_6,
        Seven = GLFW_KEY_7,
        Eight = GLFW_KEY_8,
        Nine = GLFW_KEY_9,
        Semicolon = GLFW_KEY_SEMICOLON,
        Equal = GLFW_KEY_EQUAL,
        A = GLFW_KEY_A,
        B = GLFW_KEY_B,
        C = GLFW_KEY_C,
        D = GLFW_KEY_D,
        E = GLFW_KEY_E,
        F = GLFW_KEY_F,
        G = GLFW_KEY_G,
        H = GLFW_KEY_H,
        I = GLFW_KEY_I,
        J = GLFW_KEY_J,
        K = GLFW_KEY_K,
        L = GLFW_KEY_L,
        M = GLFW_KEY_M,
        N = GLFW_KEY_N,
        O = GLFW_KEY_O,
        P = GLFW_KEY_P,
        Q = GLFW_KEY_Q,
        R = GLFW_KEY_R,
        S = GLFW_KEY_S,
        T = GLFW_KEY_T,
        U = GLFW_KEY_U,
        V = GLFW_KEY_V,
        W = GLFW_KEY_W,
        X = GLFW_KEY_X,
        Y = GLFW_KEY_Y,
        Z = GLFW_KEY_Z,
        LeftBracket = GLFW_KEY_LEFT_BRACKET,
        Backslash = GLFW_KEY_BACKSLASH,
        RightBracket = GLFW_KEY_RIGHT_BRACKET,
        GraveAccent = GLFW_KEY_GRAVE_ACCENT,
        World1 = GLFW_KEY_WORLD_1,
        World2 = GLFW_KEY_WORLD_2,

        Escape = GLFW_KEY_ESCAPE,
        Enter = GLFW_KEY_ENTER,
        Tab = GLFW_KEY_TAB,
        Backspace = GLFW_KEY_BACKSPACE,
        Insert = GLFW_KEY_INSERT,
        Delete = GLFW_KEY_DELETE,
        Right = GLFW_KEY_RIGHT,
        Left = GLFW_KEY_LEFT,
        Down = GLFW_KEY_DOWN,
        Up = GLFW_KEY_UP,
        PageUp = GLFW_KEY_PAGE_UP,
        PageDown = GLFW_KEY_PAGE_DOWN,
        Home = GLFW_KEY_HOME,
        End = GLFW_KEY_END,
        CapsLock = GLFW_KEY_CAPS_LOCK,
        ScrollLock = GLFW_KEY_SCROLL_LOCK,
        NumLock = GLFW_KEY_NUM_LOCK,
        PrintScreen = GLFW_KEY_PRINT_SCREEN,
        Pause = GLFW_KEY_PAUSE,
        F1 = GLFW_KEY_F1,
        F2 = GLFW_KEY_F2,
        F3 = GLFW_KEY_F3,
        F4 = GLFW_KEY_F4,
        F5 = GLFW_KEY_F5,
        F6 = GLFW_KEY_F6,
        F7 = GLFW_KEY_F7,
        F8 = GLFW_KEY_F8,
        F9 = GLFW_KEY_F9,
        F10 = GLFW_KEY_F10,
        F11 = GLFW_KEY_F11,
        F12 = GLFW_KEY_F12,
        F13 = GLFW_KEY_F13,
        F14 = GLFW_KEY_F14,
        F15 = GLFW_KEY_F15,
        F16 = GLFW_KEY_F16,
        F17 = GLFW_KEY_F17,
        F18 = GLFW_KEY_F18,
        F19 = GLFW_KEY_F19,
        F20 = GLFW_KEY_F20,
        F21 = GLFW_KEY_F21,
        F22 = GLFW_KEY_F22,
        F23 = GLFW_KEY_F23,
        F24 = GLFW_KEY_F24,
        F25 = GLFW_KEY_F25,
        Kp0 = GLFW_KEY_KP_0,
        Kp1 = GLFW_KEY_KP_1,
        Kp2 = GLFW_KEY_KP_2,
        Kp3 = GLFW_KEY_KP_3,
        Kp4 = GLFW_KEY_KP_4,
        Kp5 = GLFW_KEY_KP_5,
        Kp6 = GLFW_KEY_KP_6,
        Kp7 = GLFW_KEY_KP_7,
        Kp8 = GLFW_KEY_KP_8,
        Kp9 = GLFW_KEY_KP_9,
        KpDecimal = GLFW_KEY_KP_DECIMAL,
        KpDivide = GLFW_KEY_KP_DIVIDE,
        KpMultiply = GLFW_KEY_KP_MULTIPLY,
        KpSubtract = GLFW_KEY_KP_SUBTRACT,
        KpAdd = GLFW_KEY_KP_ADD,
        KpEnter = GLFW_KEY_KP_ENTER,
        KpEqual = GLFW_KEY_KP_EQUAL,
        LeftShift = GLFW_KEY_LEFT_SHIFT,
        LeftControl = GLFW_KEY_LEFT_CONTROL,
        LeftAlt = GLFW_KEY_LEFT_ALT,
        LeftSuper = GLFW_KEY_LEFT_SUPER,
        RightShift = GLFW_KEY_RIGHT_SHIFT,
        RightControl = GLFW_KEY_RIGHT_CONTROL,
        RightAlt = GLFW_KEY_RIGHT_ALT,
        RightSuper = GLFW_KEY_RIGHT_SUPER,
        Menu = GLFW_KEY_MENU
    }
}

bitflags! {
    pub struct Modifiers: c_int {
        const SHIFT = GLFW_MOD_SHIFT;
        const CONTROL = GLFW_MOD_CONTROL;
        const ALT = GLFW_MOD_ALT;
        const SUPER = GLFW_MOD_SUPER;
    }
}
