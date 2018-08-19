use std::marker::PhantomData;

use gl::types::*;
use gl;

use gfx::Context;
use gfx::Framebuffer;

pub struct Surface<'a: 'b, 'b> {
    ctx: &'a Context,
    from: PhantomData<&'b mut Framebuffer<'a>>,
    id: GLuint,
    x: u32,
    y: u32,
    pub(crate) width: u32,
    pub(crate) height: u32
}

impl<'a, 'b> Surface<'a, 'b> {
    pub fn clear_color(&mut self, color: &[f32; 4]) {
        self.bind();
        unsafe {
            gl::Scissor(self.x as GLint, self.y as GLint, self.width as GLint, self.height as GLint);
            gl::Enable(gl::SCISSOR_TEST);
            gl::ClearBufferfv(gl::COLOR, 0, color as *const GLfloat);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }

    pub fn clear_depth(&mut self, depth: f32) {
        self.bind();
        unsafe {
            gl::Scissor(self.x as GLint, self.y as GLint, self.width as GLint, self.height as GLint);
            gl::Enable(gl::SCISSOR_TEST);
            gl::ClearBufferfv(gl::DEPTH, 0, &depth as *const GLfloat);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }

    pub fn clear_stencil(&mut self, stencil: i32) {
        self.bind();
        unsafe {
            gl::Scissor(self.x as GLint, self.y as GLint, self.width as GLint, self.height as GLint);
            gl::Enable(gl::SCISSOR_TEST);
            gl::ClearBufferiv(gl::STENCIL, 0, &stencil as *const GLint);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }

    pub fn subsurface<'c>(&'c mut self, x: u32, y: u32, width: u32, height: u32) -> Surface<'a, 'c> where 'b: 'c {
        assert!(x <= self.width, "x is out of bounds: {}", x);
        assert!(y <= self.height, "y is out of bounds: {}", y);
        assert!(x + width <= self.width, "width is out of bounds: {}", width);
        assert!(y + height <= self.height, "height is out of bounds: {}", height);
        Surface {
            ctx: self.ctx,
            from: PhantomData,
            id: self.id,
            x: self.x + x,
            y: self.y + y,
            width: width,
            height: height
        }
    }

    pub fn split_vertical<'c>(&'c mut self, x: u32) -> (Surface<'a, 'c>, Surface<'a, 'c>) where 'b: 'c {
        assert!(x <= self.width, "x is out of bounds: {}", x);
        (
            Surface {
                ctx: self.ctx,
                from: PhantomData,
                id: self.id,
                y: self.y,
                height: self.height,
                x: 0,
                width: x
            },
            Surface {
                ctx: self.ctx,
                from: PhantomData,
                id: self.id,
                y: self.y,
                height: self.height,
                x: x,
                width: self.width - x
            }
        )
    }

    pub fn split_horizontal<'c>(&'c mut self, y: u32) -> (Surface<'a, 'c>, Surface<'a, 'c>) {
        assert!(y <= self.y, "y is out of bounds: {}", y);
        (
            Self {
                ctx: self.ctx,
                from: self.from,
                id: self.id,
                x: self.x,
                width: self.width,
                y: 0,
                height: y
            },
            Self {
                ctx: self.ctx,
                from: self.from,
                id: self.id,
                x: self.x,
                width: self.width,
                y: y,
                height: self.height - y
            }
        )
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub(crate) fn bind(&mut self) {
        self.ctx.bind_framebuffer(self.id);
        unsafe { gl::Viewport(self.x as GLint, self.y as GLint, self.width as GLint, self.height as GLint) };
    }
}

impl Context {
    pub(crate) fn create_screen_surface(&self, w: u32, h: u32) -> Surface {
        Surface {
            id: 0,
            ctx: self,
            from: PhantomData,
            x: 0,
            y: 0,
            width: w,
            height: h
        }
    }
}
