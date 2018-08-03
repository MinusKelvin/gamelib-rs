use std::cell::Cell;

use gl::types::*;
use gl;

mod texture;
pub use self::texture::*;

mod framebuffer;
pub use self::framebuffer::*;

mod buffer;
pub use self::buffer::*;

mod render_command;
pub use self::render_command::*;

pub mod vertex;

pub mod shader;
pub use self::shader::{ Program, GlslType, glsl_type };

pub struct Context {
    default_fbo_created: Cell<bool>
}

impl Context {
    pub (crate) fn create() -> Self {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        Self { default_fbo_created: Cell::from(false) }
    }

    pub (crate) fn bind_framebuffer(&self, id: GLuint) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, id) };
    }

    pub (crate) fn bind_array_buffer(&self, id: GLuint) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, id) };
    }

    pub (crate) fn use_program(&self, id: GLuint) {
        unsafe { gl::UseProgram(id) };
    }
}
