

use piston_window::Context;

use crate::drawable::{Drawable, Shape};
use std::collections::{HashSet, HashMap};
use opengl_graphics::GlGraphics;

pub mod drawable;
pub mod step;
pub mod graphics;

#[derive(PartialEq, Eq, Hash)]
pub enum ModId {
    Draw,
    Down,
}

struct Mod<T> {
    pub function: T,
    // maps to Data in control
    pub value: HashSet<isize>,
}

#[derive(Clone, Copy)]
pub struct ModValue {
    draw: Option<Drawable>,
    down: Option<()>,
}

pub struct Control {
    index: isize,
    // simulation objects
    data: HashMap<isize, ModValue>,
    step: HashMap<ModId, Box<Mod<fn(&isize, &ModValue) -> ModValue>>>,
    draw: HashMap<ModId, Box<Mod<fn(&Context, &mut GlGraphics, &ModValue) -> ()>>>,
}

pub fn sim_round(control: &mut Control, context: &Context, graphics: &mut GlGraphics) {
    for (_, module ) in control.step.iter_mut() {
        for id in (**module).value.iter() {
            control.data.insert(*id, ((**module).function)(id, control.data.get(id).unwrap()));
        }
    }
    // draw graphics
    for (_, module ) in control.draw.iter_mut() {
        for id in (**module).value.iter() {
            ((**module).function)(context, graphics, control.data.get(id).unwrap());
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut data = HashMap::new();
    data.insert(0, ModValue {
        draw: Some(Drawable {
            x: 1.,
            y: 1.,
            shape: Shape::SQUARE,
        }),
        down: Some(()),
    });
    data.insert(1, ModValue {
        draw: Some(Drawable {
            x: 1.,
            y: 1.,
            shape: Shape::CIRCLE,
        }),
        down: Some(()),
    });
    data.insert(2, ModValue {
        draw: Some(Drawable {
            x: 200.,
            y: 200.,
            shape: Shape::SQUARE,
        }),
        down: Some(()),
    });
    let mut step = HashMap::new();
    {
        let mut set = HashSet::new();
        set.insert(1);
        step.insert(ModId::Down, Box::new(Mod {
            function: step::down as fn(&isize, &ModValue) -> ModValue,
            value: set,
        }));
    }
    let mut draw = HashMap::new();
    {
        let mut set = HashSet::new();
        set.insert(0);
        set.insert(1);
        set.insert(2);
        draw.insert(ModId::Draw, Box::new(Mod {
            function: drawable::draw as fn(&Context, &mut GlGraphics, &ModValue) -> (),
            value: set,
        }));
    }
    let mut control = Control {
        index: 2,
        // simulation objects
        data,
        step,
        draw,
    };
    graphics::run(&mut control);
}
