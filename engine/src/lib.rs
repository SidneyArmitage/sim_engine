extern crate gl;
extern crate sdl2;

use std::collections::{HashMap, HashSet};

use graphics::{App, program::Program};
pub mod graphics;
pub struct Mod<T> {
  pub function: T,
  // maps to Data in control
  pub value: HashSet<isize>,
}

pub struct Control<T, G> {
  pub index: isize,
  // simulation objects
  pub data: HashMap<isize, T>,
  pub draw: Vec<HashMap<G, Box<Mod<fn(&T) -> ()>>>>,
  pub step: HashMap<G, Box<Mod<fn(&isize, &T) -> T>>>,
}

pub fn sim_round<T, G>(control: &mut Control<T, G>, program: &Program) {
  //step
  for (_, module) in control.step.iter_mut() {
    for id in (**module).value.iter() {
      control.data.insert(
        *id,
        ((**module).function)(id, control.data.get(id).unwrap()),
      );
    }
  }
  program.set_used();
  unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
  }
  // draw graphics
  for mut draw in control.draw.iter_mut() {
    for (_, module) in draw.iter_mut() {
      for id in (**module).value.iter() {
          ((**module).function)(control.data.get(id).unwrap());
      }
    }
  }
  unsafe {
    gl::DrawArrays(
        gl::TRIANGLES, // mode
        0, // starting index in the enabled arrays
        3 // number of indices to be rendered
    );
  }
}

pub fn start<T, G>(control: &mut Control<T, G>) {
  App::new(control);
}