use gl::types::*;
use gl;

mod texture;
pub use self::texture::*;

mod framebuffer;
pub(crate) use self::framebuffer::*;

mod surface;
pub use self::surface::*;

mod buffer;
pub use self::buffer::*;

mod render_command;
pub use self::render_command::{ RenderCommand };

pub mod vertex;

pub mod shader;
pub use self::shader::{ Program, GlslDataType, glsl_type };

pub struct Context {
}

impl Context {
    pub (crate) fn create() -> Self {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        Self {  }
    }

    pub(crate) fn bind_framebuffer(&self, id: GLuint) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, id) };
    }

    pub(crate) fn bind_array_buffer(&self, id: GLuint) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, id) };
    }

    pub(crate) fn bind_texture_2d(&self, id: GLuint) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, id) };
    }

    pub (crate) fn use_program(&self, id: GLuint) {
        unsafe { gl::UseProgram(id) };
    }
}
