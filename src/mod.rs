extern crate sdl2;
extern crate gl;

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

fn main () {
  example::double_pendulum::main();
  // example::shapes::main();
}