use std::marker::PhantomData;
use std::ffi::CString;
use std::ptr;

use gl::types::*;
use gl;

use graphics::{ Context };
use graphics::vertex;
use graphics::shader::UniformList;

pub struct Program<'a, L: vertex::Layout, U: UniformList> {
    pub (crate) ctx: &'a Context,
    pub (crate) id: GLuint,
    pub (crate) layout: L,
    _phantom: PhantomData<U>
}

impl Context {
    pub fn create_shader_program<'a, L: vertex::Layout, U: UniformList>(&'a self, layout: L, vs_code: &str, fs_code: &str) -> Program<'a, L, U> {
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

        Program {
            ctx: self,
            layout: layout,
            id: unsafe {
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
            },
            _phantom: PhantomData
        }
    }
}

impl<'a, L: vertex::Layout, U: UniformList> Drop for Program<'a, L, U> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

// macro_rules! shader_program {
//     ($ctx:expr, ($($vinput:tt)*) => vertex $vcode:tt -> ($($finput:tt)*) => fragment $fcode:tt -> ($(foutput:tt)*)) => {
//         $ctx.create_shader_program(
//
//         )
//     };
// }
//
// shader_program! { ctx,
//     () => vertex {
//
//     } -> () => fragment {
//
//     } -> ()
// }
