use gl::types::*;

use gfx::Context;

pub struct Framebuffer<'a> {
    ctx: &'a Context,
    id: GLuint
}
