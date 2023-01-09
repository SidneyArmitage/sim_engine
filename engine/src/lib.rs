extern crate gl;
extern crate sdl2;

use std::collections::{HashMap, HashSet};

use gl::types::GLuint;
use graphics::{Graphics, program::Program};
use paint::{Paint, clear};
pub mod graphics;
pub mod paint;
pub struct Mod<T> {
  pub function: T,
  // maps to Data in control
  pub value: HashSet<isize>,
}

pub struct Draw<T, G> {
  pub map: HashMap<G, Box<Mod<fn(&mut Paint, &T) -> T>>>,
  pub post: fn(&Paint) -> (),
  pub pre: fn(&Program) -> (),
  pub program: Program,
  pub paint: Paint,
}

pub struct Control<T, G> {
  pub index: isize,
  // simulation objects
  pub data: HashMap<isize, T>,
  pub draw: Vec<Draw<T, G>>,
  pub step: HashMap<G, Box<Mod<fn(u128, &isize, &T) -> T>>>,
}

pub fn sim_round<T, G>(delta_time: u128, control: &mut Control<T, G>) {
  //step
  for (_, module) in control.step.iter_mut() {
    for id in (**module).value.iter() {
      control.data.insert(
        *id,
        ((**module).function)(delta_time, id, control.data.get(id).unwrap()),
      );
    }
  }
  clear();
  // draw graphics
  for mut draw in control.draw.iter_mut() {
    draw.program.set_used();
    (draw.pre)(&draw.program);
    for (_, module) in draw.map.iter_mut() {
      for id in (**module).value.iter() {
        control.data.insert(
          *id,
          ((**module).function)(&mut draw.paint, control.data.get(id).unwrap())
        );
      }
    }
    (draw.post)(&draw.paint);
    draw.paint.set_draw_triangles();
    draw.paint.publish();
  }
}

pub fn start<T, G>(init: fn (app: Graphics) -> App<T, G>) {
  let graphics = Graphics::new();
  let mut app = init(graphics);
  app.start();
}


pub struct App<T, G> {
  control: Control<T, G>,
  graphics: Graphics,
}

impl<T, G> App<T, G> {
  pub fn new(control: Control<T, G>, graphics: Graphics) -> Self {
    App {
      control,
      graphics
    }
  }

  fn start(&mut self) {
    self.graphics.start(&mut self.control);
  }
}