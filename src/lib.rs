extern crate libc;
extern crate gl;
extern crate cgmath;
extern crate image;

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate bitflags;

use libc::{ c_void, c_int, c_char };

mod ffi;
use ffi::*;

mod events;
pub use events::{ Event, MouseButton, Key, Modifiers };

pub mod gfx;

pub trait Game {
    fn frame(&mut self, screen: gfx::Surface, delta: f64);

    fn should_exit(&mut self) -> bool;

    fn event(&mut self, event: Event);
}

pub fn launch<F>(config: Configuration, init: F)
where
    F: for<'a> FnOnce(&'a gfx::Context) -> Box<Game + 'a>
{
    unsafe {
        init_glfw();

        let title = to_cstring(config.title);
        let window = match config.mode {
            WindowMode::Windowed { width, height, resizeable, min_limits, max_limits } => {
                glfwWindowHint(GLFW_RESIZABLE, if resizeable { 1 } else { 0 });
                let w = glfwCreateWindow(width, height, title.as_ptr(), std::ptr::null_mut(), std::ptr::null_mut());
                glfwSetWindowSizeLimits(w, min_limits.0.unwrap_or(-1), min_limits.1.unwrap_or(-1), max_limits.0.unwrap_or(-1), max_limits.1.unwrap_or(-1));
                w
            },
            WindowMode::Maximised => {
                glfwWindowHint(GLFW_MAXIMIZED, 1);
                glfwCreateWindow(1280, 720, title.as_ptr(), std::ptr::null_mut(), std::ptr::null_mut())
            },
            WindowMode::Fullscreen => {
                let monitor = glfwGetPrimaryMonitor();
                let vidmode = &*glfwGetVideoMode(monitor);
                glfwWindowHint(GLFW_RED_BITS, vidmode.redBits);
                glfwWindowHint(GLFW_GREEN_BITS, vidmode.greenBits);
                glfwWindowHint(GLFW_BLUE_BITS, vidmode.blueBits);
                glfwWindowHint(GLFW_REFRESH_RATE, vidmode.refreshRate);
                glfwCreateWindow(vidmode.width, vidmode.height, title.as_ptr(), monitor, std::ptr::null_mut())
            }
        };
        drop(title);
        let gaurd = ScopeGaurd(|| {
            glfwDestroyWindow(window);
            glfwTerminate();
        });

        glfwMakeContextCurrent(window);
        gl::load_with(|s| {
            let c = to_cstring(s);
            glfwGetProcAddress(c.as_ptr())
        });

        setup_callbacks(window);

        let ctx = gfx::Context::create();
        let mut target = {
            let mut w = 0;
            let mut h = 0;
            glfwGetFramebufferSize(window, &mut w, &mut h);
            events::Target {
                game: init(&ctx),
                ctx: &ctx,
                queue: Vec::new(),
                polling: false,
                width: w as u32,
                height: h as u32
            }
        };
        glfwSetWindowUserPointer(window, &mut target as *mut events::Target as *mut c_void);

        let mut last_time = glfwGetTime();
        while !target.game.should_exit() {
            for e in target.queue.drain(0..) {
                if let Event::Resize(width, height) = e {
                    target.width = width;
                    target.height = height;
                }
                target.game.event(e);
            }
            target.polling = true;
            glfwPollEvents();
            target.polling = false;

            let now = glfwGetTime();
            let delta = now - last_time;
            last_time = now;
            target.game.frame(target.ctx.create_screen_surface(target.width, target.height), delta);

            glfwSwapBuffers(window);
        }

        glfwSetWindowUserPointer(window, std::ptr::null_mut());

        drop(target);
        drop(gaurd);
    }
}

pub struct Configuration<'a> {
    pub mode: WindowMode,
    pub title: &'a str
}

pub enum WindowMode {
    Windowed {
        width: i32,
        height: i32,
        resizeable: bool,
        min_limits: (Option<i32>, Option<i32>),
        max_limits: (Option<i32>, Option<i32>)
    },
    Maximised,
    Fullscreen
}

fn init_glfw() {
    unsafe {
        glfwSetErrorCallback(Some(error_panic));

        assert_eq!(glfwInit(), 1);

        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, 1);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    }
}

fn setup_callbacks(window: *mut GLFWwindow) {
    unsafe {
        glfwSetWindowCloseCallback(window, Some(events::window_close));
        glfwSetWindowFocusCallback(window, Some(events::window_focus));
        glfwSetWindowRefreshCallback(window, Some(events::window_refresh));
        glfwSetFramebufferSizeCallback(window, Some(events::framebuffer_size));
        glfwSetWindowContentScaleCallback(window, Some(events::content_scale_change));
        glfwSetWindowIconifyCallback(window, Some(events::window_iconify));
        glfwSetMouseButtonCallback(window, Some(events::mouse_button));
        glfwSetCursorPosCallback(window, Some(events::mouse_pos));
        glfwSetScrollCallback(window, Some(events::scroll));
        glfwSetKeyCallback(window, Some(events::key));
        glfwSetCharModsCallback(window, Some(events::char_mods));
    }
}

extern "C" fn error_panic(code: c_int, desc: *const c_char) {
    panic!("{}: {}", match code {
        GLFW_NOT_INITIALIZED => "GLFW_NOT_INITIALIZED",
        GLFW_NO_CURRENT_CONTEXT => "GLFW_NO_CURRENT_CONTEXT",
        GLFW_INVALID_ENUM => "GLFW_INVALID_ENUM",
        GLFW_INVALID_VALUE => "GLFW_INVALID_VALUE",
        GLFW_OUT_OF_MEMORY => "GLFW_OUT_OF_MEMORY",
        GLFW_API_UNAVAILABLE => "GLFW_API_UNAVAILABLE",
        GLFW_VERSION_UNAVAILABLE => "GLFW_VERSION_UNAVAILABLE",
        GLFW_PLATFORM_ERROR => "GLFW_PLATFORM_ERROR",
        GLFW_FORMAT_UNAVAILABLE => "GLFW_FORMAT_UNAVAILABLE",
        GLFW_NO_WINDOW_CONTEXT => "GLFW_NO_WINDOW_CONTEXT",
        _ => "Unknown Error"
    }, from_cstring(desc));
}

struct ScopeGaurd<F: FnMut()>(F);

impl<F: FnMut()> Drop for ScopeGaurd<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

pub mod tlprog {
    use std::marker::PhantomData;

    pub trait TLNatural {}

    pub enum Zero {}
    impl TLNatural for Zero {}

    pub struct Successor<T: TLNatural>(PhantomData<T>);
    impl<T: TLNatural> TLNatural for Successor<T> {}

    pub trait TLOption<T> {
        fn reify(self) -> Option<T>;
    }

    pub struct TLNone;
    impl<T> TLOption<T> for TLNone {
        fn reify(self) -> Option<T> { None }
    }

    pub struct TLSome<T>(pub T);
    impl<T> TLOption<T> for TLSome<T> {
        fn reify(self) -> Option<T> { Some(self.0) }
    }
}
