use std::{ffi::CString, marker::{self, PhantomData}};

use super::{create_whitespace_cstring_with_len, shader::Shader};

pub struct Program {
  id: gl::types::GLuint,
}

impl Program {
  pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
    let program_id = unsafe { gl::CreateProgram() };

    for shader in shaders {
      unsafe {
        gl::AttachShader(program_id, shader.id);
      }
    }

    unsafe {
      gl::LinkProgram(program_id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
      gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
      let mut len: gl::types::GLint = 0;
      unsafe {
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
      }

      let error = create_whitespace_cstring_with_len(len as usize);

      unsafe {
        gl::GetProgramInfoLog(
          program_id,
          len,
          std::ptr::null_mut(),
          error.as_ptr() as *mut gl::types::GLchar,
        );
      }

      return Err(error.to_string_lossy().into_owned());
    }

    for shader in shaders {
      unsafe {
        gl::DetachShader(program_id, shader.id);
      }
    }

    Ok(Program { id: program_id })
  }

  pub fn set_used(&self) {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  pub fn get_uniform<T>(&self, uniform_name: &str) -> Uniform<T> {
    let uniform_c_string = CString::new(uniform_name).unwrap();
    let id = unsafe { gl::GetUniformLocation(self.id, uniform_c_string.as_ptr()) };
    Uniform::<T> { id: id, _data: PhantomData }
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }
}

pub struct Uniform<T> {
  id: i32,
  _data: PhantomData<T>,
}

impl Uniform<f32> {
  pub fn set_uniform(&self, value: f32) {
    unsafe {
      gl::Uniform1f(self.id, value);
    }
  }
}

impl Uniform<[f32; 4]> {
  pub fn set_uniform(&self, value: [f32; 4]) {
    unsafe {
      gl::Uniform4f(self.id, value[0], value[1], value[2], value[3]);
    }
  }
}
