use std::marker::PhantomData;
use std::mem::size_of;
use std::os::raw::c_void;

use gl::types::*;
use gl;

use gfx::{ Context };
use gfx::vertex;

pub struct VertexBuffer<'a, S: vertex::Struct> {
    pub (crate) id: GLuint,
    pub (crate) elems: usize,
    ctx: &'a Context,
    _phantom: PhantomData<S>,
}

impl Context {
    pub fn create_vertex_buffer<'a, S: vertex::Struct>(&'a self) -> VertexBuffer<'a, S> {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        VertexBuffer {
            id: id,
            elems: 0,
            ctx: self,
            _phantom: PhantomData,
        }
    }
}

impl<'a, S: vertex::Struct> VertexBuffer<'a, S> {
    pub fn allocate(&mut self, data: &[S]) {
        self.bind();
        self.elems = data.len();
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, (data.len() * size_of::<S>()) as isize, data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }

    pub fn set_slice(&mut self, start: usize, data: &[S]) {
        self.bind();
        if start + data.len() > self.elems {
            panic!("out of bounds: {}", start + data.len());
        }
        unsafe {
            gl::BufferSubData(gl::ARRAY_BUFFER, start as isize, (data.len() * size_of::<S>()) as isize, data.as_ptr() as *const c_void);
        }
    }

    pub (crate) fn bind(&mut self) {
        self.ctx.bind_array_buffer(self.id);
    }
}
