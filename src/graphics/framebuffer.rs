use gl::types::*;
use gl;

use graphics::Context;

pub struct RenderTarget<'a> {
    id: GLuint,
    ctx: &'a Context,
    pub(crate) width: i32,
    pub(crate) height: i32
}

impl Context {
    pub(crate) fn create_default_framebuffer(&self) -> RenderTarget {
        RenderTarget { id: 0, ctx: self, width: 0, height: 0 }
    }
}

impl<'a> RenderTarget<'a> {
    pub fn clear_color(&mut self, color: &[f32; 4]) {
        self.bind();
        unsafe { gl::ClearBufferfv(gl::COLOR, 0, color as *const GLfloat) };
    }

    pub fn clear_depth(&mut self, depth: f32) {
        self.bind();
        unsafe { gl::ClearBufferfv(gl::DEPTH, 0, &depth as *const GLfloat) };
    }

    pub fn clear_stencil(&mut self, stencil: i32) {
        self.bind();
        unsafe { gl::ClearBufferiv(gl::STENCIL, 0, &stencil as *const GLint) };
    }

    pub (crate) fn bind(&mut self) {
        self.ctx.bind_framebuffer(self.id);
    }
}
