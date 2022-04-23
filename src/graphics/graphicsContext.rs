use gl::types::GLint;
use sdl2::video::gl_attr::GLAttr;

pub struct  GraphicsContext {}

impl GraphicsContext {
  pub fn new(gl_attr: GLAttr) {
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
  }
}

pub fn clearColour(r: f32, g: f32, b: f32, a: f32) {
  unsafe {
    gl::ClearColor(r, g, b, a);
  }
}

pub fn clear() {
  unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
  }
}

pub fn viewPort(x: GLint, y: GLint, w: GLint, h: GLint) {
  unsafe {
    gl::Viewport(x, y, w, h);
  }
}