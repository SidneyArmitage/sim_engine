use gl::types::GLuint;
use sdl2::{
  self,
  video::{Window},
  EventPump,
};
use std::ffi::CString;
use std::{
  sync::mpsc::{self, Receiver, SendError, Sender, TryRecvError},
};

use crate::{Control, sim_round};

use self::{shader::Shader, program::Program};

pub mod program;
mod shader;
pub struct App {
  tx: Sender<()>,
}

fn start<T, G>(rx: &Receiver<()>, event_pump: &mut EventPump, window: Window, vao: GLuint, program: Program, control: &mut Control<T, G>) {
  'main: loop {
    match rx.try_recv() {
      Ok(_) | Err(TryRecvError::Disconnected) => {
        println!("Terminating.");
        break 'main;
      }
      Err(TryRecvError::Empty) => {}
    }
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. } => {
          print!("quit message received");
          break 'main;
        }
        _ => {}
      }
    }
    unsafe {
      gl::BindVertexArray(vao);
    }
    sim_round(control, &program);
    window.gl_swap_window();
  }
}

fn init_program() -> Result<Program, String> {
  let shaders = [
    Shader::from_frag_source(&CString::new(include_str!("../data/graphics/basicProgram/frag.glsl")).unwrap()).unwrap(),
    Shader::from_vert_source(&CString::new(include_str!("../data/graphics/basicProgram/vert.glsl")).unwrap()).unwrap(),
  ];
  Program::from_shaders(&shaders)
}

impl App {
  pub fn new<T, G>(control: &mut Control<T, G>) -> Self {
    let (tx, rx) = mpsc::channel::<()>();
      let sdl = sdl2::init().unwrap();
      let video_subsystem = sdl.video().unwrap();
      let gl_attr = video_subsystem.gl_attr();
      gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
      gl_attr.set_context_version(4, 5);
      let window = video_subsystem
        .window("test", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
      let mut event_pump = sdl.event_pump().unwrap();
      let gl_context = window.gl_create_context().unwrap();
      let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
      unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.5, 0.5, 0.5, 1.0);
      }
      // to be moved out later
      let program = init_program().unwrap();
      program.set_used();
      let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
      ];
      let mut vertex_buffer_object: GLuint = 0;
      let mut vertex_array_object: GLuint = 0;
      unsafe {
        gl::GenBuffers(1, &mut vertex_buffer_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
        gl::BufferData(
          gl::ARRAY_BUFFER,
          (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
          vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
          gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::BindVertexArray(vertex_array_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
      }
      start(&rx, &mut event_pump, window, vertex_array_object, program, control);
    Self { tx }
  }

  pub fn close(&mut self) -> Result<(), SendError<()>> {
    self.tx.send(())
  }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  // allocate buffer of correct size
  let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
  // fill it with len spaces
  buffer.extend([b' '].iter().cycle().take(len));
  // convert buffer to CString
  unsafe { CString::from_vec_unchecked(buffer) }
}
