use opengl_graphics::GlGraphics;
use piston_window::Context;
use std::collections::{HashMap, HashSet};
use drawable::{Drawable, Shape};
use crate::{Control, Mod, graphics};

pub mod step;
pub mod drawable;

#[derive(PartialEq, Eq, Hash)]
pub enum ModId {
    Draw,
    Down,
}


#[derive(Clone, Copy)]
pub struct ModValue {
    draw: Option<Drawable>,
    down: Option<()>,
}

pub fn main() {
    println!("Hello, world!");
    let mut data = HashMap::new();
    data.insert(
        0,
        ModValue {
            draw: Some(Drawable {
                x: 1.,
                y: 1.,
                shape: Shape::SQUARE,
            }),
            down: Some(()),
        },
    );
    data.insert(
        1,
        ModValue {
            draw: Some(Drawable {
                x: 1.,
                y: 1.,
                shape: Shape::CIRCLE,
            }),
            down: Some(()),
        },
    );
    data.insert(
        2,
        ModValue {
            draw: Some(Drawable {
                x: 200.,
                y: 200.,
                shape: Shape::SQUARE,
            }),
            down: Some(()),
        },
    );
    let mut step = HashMap::new();
    {
        let mut set = HashSet::new();
        set.insert(1);
        step.insert(
            ModId::Down,
            Box::new(Mod {
                function: step::down as fn(&isize, &ModValue) -> ModValue,
                value: set,
            }),
        );
    }
    let mut draw = HashMap::new();
    {
        let mut set = HashSet::new();
        set.insert(0);
        set.insert(1);
        set.insert(2);
        draw.insert(
            ModId::Draw,
            Box::new(Mod {
                function: drawable::draw as fn(&Context, &mut GlGraphics, &ModValue) -> (),
                value: set,
            }),
        );
    }
    let mut control: Control<ModValue, ModId> = Control {
        index: 2,
        // simulation objects
        data,
        step,
        draw,
    };
    graphics::run(&mut control);
}
