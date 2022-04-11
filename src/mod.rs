use piston_window::Context;
use opengl_graphics::GlGraphics;
use std::collections::{HashMap, HashSet};

pub mod graphics;
mod example;

pub struct Mod<T> {
    pub function: T,
    // maps to Data in control
    pub value: HashSet<isize>,
}

pub struct Control<T, G> {
    index: isize,
    // simulation objects
    data: HashMap<isize, T>,
    step: HashMap<G, Box<Mod<fn(&isize, &T) -> T>>>,
    draw: Vec<HashMap<G, Box<Mod<fn(&Context, &mut GlGraphics, &T) -> ()>>>>,
}

pub fn sim_round<T, G>(control: &mut Control<T, G>, context: &Context, graphics: &mut GlGraphics) {
    for (_, module) in control.step.iter_mut() {
        for id in (**module).value.iter() {
            control.data.insert(
                *id,
                ((**module).function)(id, control.data.get(id).unwrap()),
            );
        }
    }
    // draw graphics
    for mut draw in control.draw.iter_mut() {
        for (_, module) in draw.iter_mut() {
            for id in (**module).value.iter() {
                ((**module).function)(context, graphics, control.data.get(id).unwrap());
            }
        }
    }
}

fn main () {
  example::double_pendulum::main();
  // example::shapes::main();
}