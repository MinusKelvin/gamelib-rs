use std::marker::PhantomData;

use gl::types::*;
use gl;

use gfx::Context;
use gfx::Framebuffer;

pub struct Surface<'a: 'b, 'b> {
    ctx: &'a Context,
    from: PhantomData<&'b mut Framebuffer<'a>>,
    id: GLuint,
    x: i32,
    y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32
}

impl<'a, 'b> Surface<'a, 'b> {
    pub fn clear_color(&mut self, color: &[f32; 4]) {
        self.bind();
        unsafe {
            gl::Scissor(self.x, self.y, self.width, self.height);
            gl::Enable(gl::SCISSOR_TEST);
            gl::ClearBufferfv(gl::COLOR, 0, color as *const GLfloat);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }

    pub fn clear_depth(&mut self, depth: f32) {
        self.bind();
        unsafe {
            gl::Scissor(self.x, self.y, self.width, self.height);
            gl::Enable(gl::SCISSOR_TEST);
            gl::ClearBufferfv(gl::DEPTH, 0, &depth as *const GLfloat);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }

    pub fn clear_stencil(&mut self, stencil: i32) {
        self.bind();
        unsafe {
            gl::Scissor(self.x, self.y, self.width, self.height);
            gl::Enable(gl::SCISSOR_TEST);
            gl::ClearBufferiv(gl::STENCIL, 0, &stencil as *const GLint);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }

    pub fn subsurface<'c>(&'c mut self, x: i32, y: i32, width: i32, height: i32) -> Surface<'a, 'c> where 'b: 'c {
        assert!(x >= 0 && x <= self.width, "x is out of bounds: {}", x);
        assert!(y >= 0 && y <= self.height, "y is out of bounds: {}", y);
        assert!(width >= 0 && x + width <= self.width, "width is out of bounds: {}", width);
        assert!(height >= 0 && y + height <= self.height, "height is out of bounds: {}", height);
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

    pub fn divide_vertical(self, x: i32) -> (Self, Self) {
        assert!(x >= 0 && x <= self.width, "x is out of bounds: {}", x);
        (
            Self {
                ctx: self.ctx,
                from: self.from,
                id: self.id,
                y: self.y,
                height: self.height,
                x: 0,
                width: x
            },
            Self {
                ctx: self.ctx,
                from: self.from,
                id: self.id,
                y: self.y,
                height: self.height,
                x: x,
                width: self.width - x
            }
        )
    }

    pub fn divide_horizontal(self, y: i32) -> (Self, Self) {
        assert!(y >= 0 && y <= self.y, "y is out of bounds: {}", y);
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

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub(crate) fn bind(&mut self) {
        self.ctx.bind_framebuffer(self.id);
        unsafe { gl::Viewport(self.x, self.y, self.width, self.height) };
    }
}

impl Context {
    pub(crate) fn create_screen_surface(&self, w: i32, h: i32) -> Surface {
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
