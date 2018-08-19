use std::ptr;

use gl::types::*;
use gl;
use image::{ ImageLuma8, ImageLumaA8, ImageRgb8, ImageRgba8, DynamicImage, GenericImage };

use gfx::Context;

pub struct Texture2D<'a> {
    ctx: &'a Context,
    pub(crate) id: GLuint,
    width: u32,
    height: u32
}

impl<'a> Drop for Texture2D<'a> {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}

impl<'a> Texture2D<'a> {
    pub(crate) fn allocate(&mut self, w: u32, h: u32, format: GLenum) {
        unsafe {
            self.bind();
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as GLint, w as GLint, h as GLint, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());
            self.width = w;
            self.height = h;
        }
    }

    fn allocate_data(&mut self, data: &DynamicImage) {
        self.bind();
        match data {
            ImageLuma8(img) => unsafe {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::R8 as GLint, img.width() as GLint, img.height() as GLint, 0, gl::RED, gl::UNSIGNED_BYTE, img.as_ptr() as *const GLvoid);
            }
            ImageLumaA8(img) => unsafe {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RG8 as GLint, img.width() as GLint, img.height() as GLint, 0, gl::RG, gl::UNSIGNED_BYTE, img.as_ptr() as *const GLvoid);
            }
            ImageRgb8(img) => unsafe {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB8 as GLint, img.width() as GLint, img.height() as GLint, 0, gl::RGB, gl::UNSIGNED_BYTE, img.as_ptr() as *const GLvoid);
            }
            ImageRgba8(img) => unsafe {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as GLint, img.width() as GLint, img.height() as GLint, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_ptr() as *const GLvoid);
            }
        }
        self.width = data.width();
        self.height = data.height();
    }

    pub(crate) fn bind(&mut self) {
        self.ctx.bind_texture_2d(self.id);
    }

    pub fn set_minify_filter(&mut self, filter: TextureFilter) {
        self.bind();
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter as GLint) };
    }

    pub fn set_magnify_filter(&mut self, filter: TextureFilter) {
        self.bind();
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter as GLint) };
    }
}

impl Context {
    pub(crate) fn create_raw_texture_2d(&self) -> Texture2D {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            self.bind_texture_2d(id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
        }
        Texture2D {
            ctx: self,
            id: id,
            width: 0,
            height: 0
        }
    }

    pub fn create_texture_2d(&self, from: &DynamicImage) -> Texture2D {
        let mut tex = self.create_raw_texture_2d();
        tex.allocate_data(from);
        tex
    }
}

#[repr(u32)]
pub enum TextureFilter {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR
}
