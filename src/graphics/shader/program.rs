use std::marker::PhantomData;
use std::ffi::CString;
use std::ptr;

use gl::types::*;
use gl;

use graphics::{ Context };
use graphics::vertex;
use graphics::shader::{ UniformList, UniformListBuilder };

pub struct Program<'a, L: vertex::Layout, UL: UniformList> {
    pub (crate) ctx: &'a Context,
    pub (crate) id: GLuint,
    pub (crate) layout: L,
    pub (crate) uniforms: UL,
    _phantom: PhantomData<UL>
}

impl Context {
    pub fn create_shader_program<'a, L, UL>(&'a self, layout: L, uniforms: UL, vs_code: &str, fs_code: &str) -> Program<'a, L::Layout, UL::UniformList>
    where
        L: vertex::LayoutBuilder,
        UL: UniformListBuilder
    {
        unsafe fn compile_shader(shader_type: GLenum, code: &str) -> GLuint {
            let shader = gl::CreateShader(shader_type);

            let c_code = ::ffi::to_cstring(code);
            let p = c_code.as_ptr();
            gl::ShaderSource(shader, 1, &p, ptr::null());
            gl::CompileShader(shader);

            let mut status = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            if status == gl::FALSE as i32 {
                let mut info_log_length = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let c_info_log = CString::new(Vec::with_capacity(info_log_length as usize)).unwrap().into_raw();
                gl::GetShaderInfoLog(shader, info_log_length, ptr::null_mut(), c_info_log);
                panic!("Failed to compile shader: {}", CString::from_raw(c_info_log).to_string_lossy());
            }
            shader
        }

        let id = unsafe {
            let vs = compile_shader(gl::VERTEX_SHADER, vs_code);
            let fs = compile_shader(gl::FRAGMENT_SHADER, fs_code);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);

            gl::LinkProgram(program);

            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status == gl::FALSE as i32 {
                let mut info_log_length = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let c_info_log = CString::new(Vec::with_capacity(info_log_length as usize)).unwrap().into_raw();
                gl::GetProgramInfoLog(program, info_log_length, ptr::null_mut(), c_info_log);
                panic!("Failed to link shader program: {}", CString::from_raw(c_info_log).to_string_lossy());
            }

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
            program
        };

        Program {
            ctx: self,
            layout: layout.into(|s| unsafe {
                let loc = gl::GetAttribLocation(id, ::ffi::to_cstring(s).as_ptr());
                assert_ne!(loc, -1, "Attribute '{}' not found in the shader program", s);
                loc
            }),
            uniforms: uniforms.into(|s| unsafe {
                let loc = gl::GetUniformLocation(id, ::ffi::to_cstring(s).as_ptr());
                assert_ne!(loc, -1, "Uniform '{}' not found in the shader program", s);
                loc
            }),
            id: id,
            _phantom: PhantomData
        }
    }
}

impl<'a, L: vertex::Layout, UL: UniformList> Drop for Program<'a, L, UL> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[macro_export]
macro_rules! shader_program {
    (@layout ()) => { $crate::graphics::vertex::layout::BuilderNil };
    (@layout ($name:ident: $type:ty)) => {{
        let l: $crate::graphics::vertex::layout::BuilderCons<$type, _> = $crate::graphics::vertex::layout::BuilderCons::new(stringify!($name), shader_program!(@layout ()));
        l
    }};
    (@layout ($name:ident: $type:ty, $($rest:tt)*)) => {{
        let l: $crate::graphics::vertex::layout::BuilderCons<$type, _> = $crate::graphics::vertex::layout::BuilderCons::new(stringify!($name), shader_program!(@layout ($($rest)*)));
        l
    }};

    (@uniforms {
        $(uniform $name1:ident: $t1:ty;)*
        code $_1:tt
    } {
        $(uniform $name2:ident: $t2:ty;)*
        code $_2:tt
    }) => { shader_program!(@uniformlist $($name1: $t1;)* $($name2: $t2;)*); };
    (@uniformlist) => { $crate::graphics::shader::uniform::BuilderNil };
    (@uniformlist $name:ident: $type:ty;) => {{
        let l: $crate::graphics::shader::uniform::BuilderCons<$type, _> = $crate::graphics::shader::uniform::BuilderCons::new(stringify!($name), shader_program!(@uniformlist));
        l
    }};
    (@uniformlist $name:ident: $type:ty; $($rest:tt)*) => {{
        let l: $crate::graphics::shader::uniform::BuilderCons<$type, _> = $crate::graphics::shader::uniform::BuilderCons::new(stringify!($name), shader_program!(@uniformlist $($rest)*));
        l
    }};

    (@vcode ($($i_name:ident: $i_t:ty),*) $a:tt $b:tt) => {
        shader_program!(@code ($($i_name: <$i_t as $crate::graphics::vertex::Attribute>::Type),*) $a $b)
    };

    (@code ($($i_name:ident: $i_t:ty),*) {$(uniform $u_name:ident: $u_t:ty;)* code { $($code:expr),* } } ($($o_name:ident: $o_t:ty),*)) => {
        &format!(
            concat!(
                "#version 330 core\n",
                $("in {} ", stringify!($i_name), ";",)*
                $("uniform {} ", stringify!($u_name), ";",)*
                $("out {} ", stringify!($o_name), ";",)*
                "{}"
            ),
            $(<$i_t as $crate::graphics::shader::GlslType>::TYPE_STRING,)*
            $(<<$u_t as $crate::graphics::shader::Uniform>::Type as $crate::graphics::shader::GlslType>::TYPE_STRING,)*
            $(<$o_t as $crate::graphics::shader::GlslType>::TYPE_STRING,)*
            concat!($($code, "\n"),*)
        )
    };

    ($ctx:expr, $vinput:tt => vertex $vs_code:tt -> $finput:tt => fragment $fs_code:tt -> $foutput:tt) => {
        $ctx.create_shader_program(
            shader_program!(@layout $vinput),
            shader_program!(@uniforms $vs_code $fs_code),
            shader_program!(@vcode $vinput $vs_code $finput),
            shader_program!(@code $finput $fs_code $foutput)
        )
    };
}
