use std::{ffi::CStr, ptr::null};

use gl::{
  types::{GLenum, GLint, GLuint},
  COMPILE_STATUS,
};

use super::create_whitespace_cstring_with_len;

pub struct Shader {
  pub id: GLuint,
}

impl Shader {
  fn from_source(source: &CStr, kind: GLenum) -> Result<Shader, String> {
    let mut success: GLint = 1;
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
      gl::ShaderSource(id, 1, &source.as_ptr(), null());
      gl::CompileShader(id);
      gl::GetShaderiv(id, COMPILE_STATUS, &mut success);
    };
    if success == 0 {
      let mut len: GLint = 0;
      unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) }
      let error = create_whitespace_cstring_with_len(len as usize);
      unsafe {
        gl::GetShaderInfoLog(
          id,
          len,
          std::ptr::null_mut(),
          error.as_ptr() as *mut gl::types::GLchar,
        );
      }
      return Err(error.to_string_lossy().into_owned());
    }
    Ok(Shader { id })
  }

  pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
    Shader::from_source(source, gl::FRAGMENT_SHADER)
  }

  pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
    Shader::from_source(source, gl::VERTEX_SHADER)
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteShader(self.id);
    }
  }
}