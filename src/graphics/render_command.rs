use std::marker::PhantomData;
use std::mem::size_of;

use gl::types::*;
use gl;

use tlprog::{ TLNatural, TLOption, TLSome, TLNone };
use graphics::{ Context, VertexBuffer };
use graphics::vertex;
use graphics::shader;

#[derive(Debug)]
struct AttribPointerData {
    buf_id: GLuint,
    index: GLuint,
    size: GLint,
    gltype: GLenum,
    normalized: GLboolean,
    stride: GLint,
    offset: usize
}

pub struct RenderCommand<'a: 'b, 'b, L: vertex::Layout + 'b, U: shader::UniformList + 'b, C: TLOption<i32>> {
    ctx: &'a Context,
    bindings: Vec<AttribPointerData>,
    shader: GLuint,
    remaining_layout: L,
    vertex_count: C,
    _phantom: PhantomData<(U, &'b ())>
}

impl<'a, 'b, L: vertex::Layout, U: shader::UniformList> RenderCommand<'a, 'b, L, U, TLNone> {
    pub fn new(shader: &'b shader::Program<'a, L, U>) -> Self {
        RenderCommand {
            ctx: shader.ctx,
            bindings: Vec::new(),
            shader: shader.id,
            remaining_layout: shader.layout,
            vertex_count: TLNone,
            _phantom: PhantomData
        }
    }
}

impl<'a: 'b, 'b, U: shader::UniformList> RenderCommand<'a, 'b, vertex::layout::Nil, U, TLSome<i32>> {
    pub fn execute(&self) {
        unsafe {
            self.ctx.use_program(self.shader);
            for binding in &self.bindings {
                self.ctx.bind_array_buffer(binding.buf_id);
                gl::VertexAttribPointer(binding.index, binding.size, binding.gltype, binding.normalized, binding.stride, binding.offset as *const GLvoid);
                gl::EnableVertexAttribArray(binding.index);
            }
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count.0);

            for binding in &self.bindings {
                gl::DisableVertexAttribArray(binding.index);
            }
        }
    }
}

impl<'a: 'b, 'b, L: vertex::Layout, U: shader::UniformList, C: TLOption<i32>> RenderCommand<'a, 'b, L, U, C> {
    pub fn attach<A, F, I1, I2, S>(mut self, buf: &'b VertexBuffer<'a, S>, _name: A) -> RenderCommand<'a, 'b, L::Remainder, U, TLSome<i32>>
    where
        L: vertex::layout::Pluck<A, I1>,
        A: vertex::Attribute,
        F: vertex::Format<Type=A::Type>,
        S: vertex::structure::Offset<A, F, I2>,
        I1: TLNatural,
        I2: TLNatural

    {
        let (idx, r) = self.remaining_layout.pluck();
        self.bindings.push(AttribPointerData {
            buf_id: buf.id,
            index: idx.into(),
            size: F::COMPONENTS,
            gltype: F::GL_TYPE,
            normalized: F::NORMALIZED,
            stride: size_of::<S>() as i32,
            offset: S::OFFSET
        });
        RenderCommand {
            ctx: self.ctx,
            bindings: self.bindings,
            shader: self.shader,
            remaining_layout: r,
            vertex_count: TLSome(match self.vertex_count.reify() {
                Some(l) => l.min(buf.elems as i32),
                None => buf.elems as i32
            }),
            _phantom: PhantomData
        }
    }

    pub fn limit_vertices(self, limit: i32) -> RenderCommand<'a, 'b, L, U, TLSome<i32>> {
        RenderCommand {
            ctx: self.ctx,
            bindings: self.bindings,
            shader: self.shader,
            remaining_layout: self.remaining_layout,
            vertex_count: TLSome(match self.vertex_count.reify() {
                Some(l) => l.min(limit).max(0),
                None => limit.max(0)
            }),
            _phantom: self._phantom
        }
    }
}
