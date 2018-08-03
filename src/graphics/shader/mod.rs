mod program;
pub use self::program::*;

pub mod uniform;
pub use self::uniform::UniformList;

pub mod glsl_type {
    pub trait GlslType : Copy {}
    #[derive(Copy, Clone, Debug)] pub struct Float;  impl GlslType for Float {}
    #[derive(Copy, Clone, Debug)] pub struct Vec2;   impl GlslType for Vec2 {}
    #[derive(Copy, Clone, Debug)] pub struct Vec3;   impl GlslType for Vec3 {}
    #[derive(Copy, Clone, Debug)] pub struct Vec4;   impl GlslType for Vec4 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat2;   impl GlslType for Mat2 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat2x3; impl GlslType for Mat2x3 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat2x4; impl GlslType for Mat2x4 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat3x2; impl GlslType for Mat3x2 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat3;   impl GlslType for Mat3 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat3x4; impl GlslType for Mat3x4 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat4x2; impl GlslType for Mat4x2 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat4x3; impl GlslType for Mat4x3 {}
    #[derive(Copy, Clone, Debug)] pub struct Mat4;   impl GlslType for Mat4 {}
}
pub use self::glsl_type::GlslType;
