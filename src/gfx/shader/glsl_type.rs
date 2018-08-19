use cgmath::*;
use gfx::render_command::UniformData;

pub trait GlslType : Copy {
    const TYPE_STRING: &'static str;
}

pub trait GlslDataType : GlslType {
    type Data;
    fn into_uniform_data(v: Self::Data) -> UniformData;
}

#[derive(Copy, Clone, Debug)]
pub struct Float;
impl GlslType for Float {
    const TYPE_STRING: &'static str = "float";
}
impl GlslDataType for Float {
    type Data = f32;
    fn into_uniform_data(v: f32) -> UniformData { UniformData::Float(v) }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec2;
impl GlslType for Vec2 {
    const TYPE_STRING: &'static str = "vec2";
}
impl GlslDataType for Vec2 {
    type Data = Vector2<f32>;
    fn into_uniform_data(v: Vector2<f32>) -> UniformData { UniformData::Vec2(v) }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3;
impl GlslType for Vec3 {
    const TYPE_STRING: &'static str = "vec3";
}
impl GlslDataType for Vec3 {
    type Data = Vector3<f32>;
    fn into_uniform_data(v: Vector3<f32>) -> UniformData { UniformData::Vec3(v) }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec4;
impl GlslType for Vec4 {
    const TYPE_STRING: &'static str = "vec4";
}
impl GlslDataType for Vec4 {
    type Data = Vector4<f32>;
    fn into_uniform_data(v: Vector4<f32>) -> UniformData { UniformData::Vec4(v) }
}

#[derive(Copy, Clone, Debug)]
pub struct Mat2;
impl GlslType for Mat2 {
    const TYPE_STRING: &'static str = "mat2";
}
impl GlslDataType for Mat2 {
    type Data = Matrix2<f32>;
    fn into_uniform_data(v: Matrix2<f32>) -> UniformData { UniformData::Mat2(v) }
}

#[derive(Copy, Clone, Debug)]
pub struct Mat3;
impl GlslType for Mat3 {
    const TYPE_STRING: &'static str = "mat3";
}
impl GlslDataType for Mat3 {
    type Data = Matrix3<f32>;
    fn into_uniform_data(v: Matrix3<f32>) -> UniformData { UniformData::Mat3(v) }
}

#[derive(Copy, Clone, Debug)]
pub struct Mat4;
impl GlslType for Mat4 {
    const TYPE_STRING: &'static str = "mat4";
}
impl GlslDataType for Mat4 {
    type Data = Matrix4<f32>;
    fn into_uniform_data(v: Matrix4<f32>) -> UniformData { UniformData::Mat4(v) }
}

#[derive(Copy, Clone, Debug)]
pub struct Sampler2D;
impl GlslType for Sampler2D {
    const TYPE_STRING: &'static str = "sampler2D";
}
