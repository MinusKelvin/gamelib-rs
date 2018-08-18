use std::marker::PhantomData;

use gl::types::*;
use gl;

use gfx::Context;

pub struct Texture2D<'a> {
    id: GLuint,
    _phantom: PhantomData<&'a Context>
}

impl<'a> Drop for Texture2D<'a> {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}

impl Context {
    pub fn create_texture_2d(&self) -> Texture2D {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id) };
        Texture2D {
            id: id,
            _phantom: PhantomData
        }
    }
}
