use std::marker::PhantomData;
use std::mem::size_of;

use gl::types::*;
use gl;

use cgmath::*;

use tlprog::{ TLNatural, TLOption, TLSome, TLNone };
use graphics::{ Context, VertexBuffer };
use graphics::framebuffer::RenderTarget;
use graphics::vertex;
use graphics::shader;
use graphics::shader::GlslType;

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

pub enum UniformData {
    Float(f32),
    Vec2(Vector2<f32>),
    Vec3(Vector3<f32>),
    Vec4(Vector4<f32>),
    Mat2(Matrix2<f32>),
    Mat3(Matrix3<f32>),
    Mat4(Matrix4<f32>),
}

pub struct RenderCommand<'a: 'b, 'b, L: vertex::Layout + 'b, UL: shader::UniformList + 'b, C: TLOption<i32>> {
    ctx: &'a Context,
    bindings: Vec<AttribPointerData>,
    uniforms: Vec<(GLint, UniformData)>,
    shader: GLuint,
    remaining_layout: L,
    remaining_uniforms: UL,
    vertex_count: C,
    _phantom: PhantomData<&'b ()>
}

impl<'a, 'b, L: vertex::Layout, UL: shader::UniformList> RenderCommand<'a, 'b, L, UL, TLNone> {
    pub fn new(shader: &'b shader::Program<'a, L, UL>) -> Self {
        RenderCommand {
            ctx: shader.ctx,
            bindings: Vec::new(),
            uniforms: Vec::new(),
            shader: shader.id,
            remaining_layout: shader.layout,
            remaining_uniforms: shader.uniforms,
            vertex_count: TLNone,
            _phantom: PhantomData
        }
    }
}

impl<'a: 'b, 'b> RenderCommand<'a, 'b, vertex::layout::Nil, shader::uniform::Nil, TLSome<i32>> {
    pub fn execute(&self, to: &mut RenderTarget) {
        unsafe {
            to.bind();
            self.ctx.use_program(self.shader);
            for binding in &self.bindings {
                self.ctx.bind_array_buffer(binding.buf_id);
                gl::VertexAttribPointer(binding.index, binding.size, binding.gltype, binding.normalized, binding.stride, binding.offset as *const GLvoid);
                gl::EnableVertexAttribArray(binding.index);
            }
            for (index, data) in &self.uniforms {
                data.submit(*index);
            }
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count.0);

            for binding in &self.bindings {
                gl::DisableVertexAttribArray(binding.index);
            }
        }
    }
}

impl<'a: 'b, 'b, L: vertex::Layout, UL: shader::UniformList, C: TLOption<i32>> RenderCommand<'a, 'b, L, UL, C> {
    pub fn attach<A, F, I1, I2, S>(mut self, buf: &'b VertexBuffer<'a, S>, _name: A) -> RenderCommand<'a, 'b, L::Remainder, UL, TLSome<i32>>
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
            index: idx as u32,
            size: F::COMPONENTS,
            gltype: F::GL_TYPE,
            normalized: F::NORMALIZED,
            stride: size_of::<S>() as i32,
            offset: S::OFFSET
        });
        RenderCommand {
            ctx: self.ctx,
            bindings: self.bindings,
            uniforms: self.uniforms,
            shader: self.shader,
            remaining_layout: r,
            remaining_uniforms: self.remaining_uniforms,
            vertex_count: TLSome(match self.vertex_count.reify() {
                Some(l) => l.min(buf.elems as i32),
                None => buf.elems as i32
            }),
            _phantom: PhantomData
        }
    }

    pub fn limit_vertices(self, limit: i32) -> RenderCommand<'a, 'b, L, UL, TLSome<i32>> {
        RenderCommand {
            ctx: self.ctx,
            bindings: self.bindings,
            uniforms: self.uniforms,
            shader: self.shader,
            remaining_layout: self.remaining_layout,
            remaining_uniforms: self.remaining_uniforms,
            vertex_count: TLSome(match self.vertex_count.reify() {
                Some(l) => l.min(limit).max(0),
                None => limit.max(0)
            }),
            _phantom: self._phantom
        }
    }

    pub fn uniform<U, I>(mut self, _name: U, value: <<U as shader::Uniform>::Type as GlslType>::Data) -> RenderCommand<'a, 'b, L, UL::Remainder, C>
    where
        U: shader::Uniform,
        UL: shader::uniform::Pluck<U, I>,
        I: TLNatural
    {
        let (idx, r) = self.remaining_uniforms.pluck();
        self.uniforms.push((idx, U::Type::into_uniform_data(value)));
        RenderCommand {
            ctx: self.ctx,
            bindings: self.bindings,
            uniforms: self.uniforms,
            shader: self.shader,
            remaining_layout: self.remaining_layout,
            remaining_uniforms: r,
            vertex_count: self.vertex_count,
            _phantom: PhantomData
        }
    }
}

impl UniformData {
    fn submit(&self, index: GLint) {
        unsafe {
            match self {
                UniformData::Float(v) => gl::Uniform1f(index, *v),
                UniformData::Vec2(v) => gl::Uniform2f(index, v.x, v.y),
                UniformData::Vec3(v) => gl::Uniform3f(index, v.x, v.y, v.z),
                UniformData::Vec4(v) => gl::Uniform4f(index, v.x, v.y, v.z, v.w),
                UniformData::Mat2(v) => gl::UniformMatrix2fv(index, 1, gl::FALSE, v.as_ptr()),
                UniformData::Mat3(v) => gl::UniformMatrix3fv(index, 1, gl::FALSE, v.as_ptr()),
                UniformData::Mat4(v) => gl::UniformMatrix4fv(index, 1, gl::FALSE, v.as_ptr()),
            }
        }
    }
}
