use std::fmt::Debug;

use gl::types::*;
use gl;

use cgmath::{ Vector2, Vector3, Vector4 };

use graphics::glsl_type;
use graphics::GlslType;

pub trait Format : Debug + Copy {
    type Type: GlslType;
    type Concrete: Copy + Debug;

    const COMPONENTS: GLint;
    const GL_TYPE: GLenum;
    const NORMALIZED: GLboolean;
}

macro_rules! vectorize {
    ($(format ($name1:ident: $glslty1:ty, $name2:ident: $glslty2:ty, $name3:ident: $glslty3:ty, $name4:ident: $glslty4:ty) {
        type Concrete = $conc:ty;
        const GL_TYPE = $glty:expr;
        const NORMALIZED = $norm:expr;
    })+) => {
        $(
            #[derive(Debug,Copy,Clone)]
            pub struct $name1;
            impl Format for $name1 {
                type Type = $glslty1;
                type Concrete = $conc;
                const COMPONENTS: GLint = 1;
                const GL_TYPE: GLenum = $glty;
                const NORMALIZED: GLboolean = $norm;
            }

            #[derive(Debug,Copy,Clone)]
            pub struct $name2;
            impl Format for $name2 {
                type Type = $glslty2;
                type Concrete = Vector2<$conc>;
                const COMPONENTS: GLint = 2;
                const GL_TYPE: GLenum = $glty;
                const NORMALIZED: GLboolean = $norm;
            }

            #[derive(Debug,Copy,Clone)]
            pub struct $name3;
            impl Format for $name3 {
                type Type = $glslty3;
                type Concrete = Vector3<$conc>;
                const COMPONENTS: GLint = 3;
                const GL_TYPE: GLenum = $glty;
                const NORMALIZED: GLboolean = $norm;
            }

            #[derive(Debug,Copy,Clone)]
            pub struct $name4;
            impl Format for $name4 {
                type Type = $glslty4;
                type Concrete = Vector4<$conc>;

                const COMPONENTS: GLint = 4;
                const GL_TYPE: GLenum = $glty;
                const NORMALIZED: GLboolean = $norm;
            }
        )*
    }
}

vectorize! {
    format (
        Float: glsl_type::Float,
        FloatVec2: glsl_type::Vec2,
        FloatVec3: glsl_type::Vec3,
        FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = f32;
        const GL_TYPE = gl::FLOAT;
        const NORMALIZED = gl::FALSE;
    }

    format (
        I32Float: glsl_type::Float,
        I32FloatVec2: glsl_type::Vec2,
        I32FloatVec3: glsl_type::Vec3,
        I32FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = i32;
        const GL_TYPE = gl::INT;
        const NORMALIZED = gl::FALSE;
    }

    format (
        NormalizedI32Float: glsl_type::Float,
        NormalizedI32FloatVec2: glsl_type::Vec2,
        NormalizedI32FloatVec3: glsl_type::Vec3,
        NormalizedI32FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = i32;
        const GL_TYPE = gl::INT;
        const NORMALIZED = gl::TRUE;
    }

    format (
        U32Float: glsl_type::Float,
        U32FloatVec2: glsl_type::Vec2,
        U32FloatVec3: glsl_type::Vec3,
        U32FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = u32;
        const GL_TYPE = gl::UNSIGNED_INT;
        const NORMALIZED = gl::FALSE;
    }

    format (
        NormalizedU32Float: glsl_type::Float,
        NormalizedU32FloatVec2: glsl_type::Vec2,
        NormalizedU32FloatVec3: glsl_type::Vec3,
        NormalizedU32FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = u32;
        const GL_TYPE = gl::UNSIGNED_INT;
        const NORMALIZED = gl::TRUE;
    }

    format (
        I16Float: glsl_type::Float,
        I16FloatVec2: glsl_type::Vec2,
        I16FloatVec3: glsl_type::Vec3,
        I16FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = i16;
        const GL_TYPE = gl::SHORT;
        const NORMALIZED = gl::FALSE;
    }

    format (
        NormalizedI16Float: glsl_type::Float,
        NormalizedI16FloatVec2: glsl_type::Vec2,
        NormalizedI16FloatVec3: glsl_type::Vec3,
        NormalizedI16FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = i16;
        const GL_TYPE = gl::SHORT;
        const NORMALIZED = gl::TRUE;
    }

    format (
        U16Float: glsl_type::Float,
        U16FloatVec2: glsl_type::Vec2,
        U16FloatVec3: glsl_type::Vec3,
        U16FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = u16;
        const GL_TYPE = gl::UNSIGNED_SHORT;
        const NORMALIZED = gl::FALSE;
    }

    format (
        NormalizedU16Float: glsl_type::Float,
        NormalizedU16FloatVec2: glsl_type::Vec2,
        NormalizedU16FloatVec3: glsl_type::Vec3,
        NormalizedU16FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = u16;
        const GL_TYPE = gl::UNSIGNED_SHORT;
        const NORMALIZED = gl::TRUE;
    }

    format (
        I8Float: glsl_type::Float,
        I8FloatVec2: glsl_type::Vec2,
        I8FloatVec3: glsl_type::Vec3,
        I8FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = i8;
        const GL_TYPE = gl::BYTE;
        const NORMALIZED = gl::FALSE;
    }

    format (
        NormalizedI8Float: glsl_type::Float,
        NormalizedI8FloatVec2: glsl_type::Vec2,
        NormalizedI8FloatVec3: glsl_type::Vec3,
        NormalizedI8FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = i8;
        const GL_TYPE = gl::BYTE;
        const NORMALIZED = gl::TRUE;
    }

    format (
        U8Float: glsl_type::Float,
        U8FloatVec2: glsl_type::Vec2,
        U8FloatVec3: glsl_type::Vec3,
        U8FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = u8;
        const GL_TYPE = gl::UNSIGNED_BYTE;
        const NORMALIZED = gl::FALSE;
    }

    format (
        NormalizedU8Float: glsl_type::Float,
        NormalizedU8FloatVec2: glsl_type::Vec2,
        NormalizedU8FloatVec3: glsl_type::Vec3,
        NormalizedU8FloatVec4: glsl_type::Vec4
    ) {
        type Concrete = u8;
        const GL_TYPE = gl::UNSIGNED_BYTE;
        const NORMALIZED = gl::TRUE;
    }
}
