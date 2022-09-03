extern crate gl;
extern crate sdl2;

use std::collections::{HashMap, HashSet};

use graphics::App;
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
  pub step: HashMap<G, Box<Mod<fn(&isize, &T) -> T>>>,
}

pub fn sim_round<T, G>(control: &mut Control<T, G>) {
  for (_, module) in control.step.iter_mut() {
    for id in (**module).value.iter() {
      control.data.insert(
        *id,
        ((**module).function)(id, control.data.get(id).unwrap()),
      );
    }
  }
}

pub fn start() {
  let app = App::new();
  loop {}
}