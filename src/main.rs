

use crate::drawable::{Drawable, Shape};
use std::collections::HashMap;

pub mod drawable;

#[derive(PartialEq, Eq, Hash)]
pub enum ModId {
    Draw,
    Down,
}

struct Mod {
    pub step: fn(&isize, &ModValue) -> ModValue,
    // maps to Data in control
    pub value: Vec<isize>,
}

#[derive(Clone, Copy)]
pub struct ModValue {
    draw: Option<Drawable>,
    down: Option<()>,
}

struct Control {
    index: isize,
    // simulation objects
    data: HashMap<isize, ModValue>,
    mods: HashMap<ModId, Box<Mod>>,
}

mod step {
    use crate::Drawable;
    use crate::ModValue;

    pub fn down(id: &isize, value: &ModValue) -> ModValue {
        let draw = value.draw.unwrap();
        ModValue {
            down: Some(()),
            draw: Some ( Drawable {
                y: draw.y - 1.,
                x: draw.x,
                shape: draw.shape,
            }),
        }
    }
}

fn sim_round(control: &mut Control) {
    for (_, module ) in control.mods.iter_mut() {
        for id in (**module).value.iter_mut() {
            control.data.insert(*id, ((**module).step)(id, control.data.get(id).unwrap()));
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
            x: 1.,
            y: 1.,
            shape: Shape::SQUARE,
        }),
        down: Some(()),
    });
    let mut mods = HashMap::new();
    mods.insert(ModId::Draw, Box::new(Mod {
        step: drawable::draw,
        value: vec![0, 1, 2],
    }));
    mods.insert(ModId::Down, Box::new(Mod {
        step: step::down,
        value: vec![1],
    }));
    let mut control = Control {
        index: 2,
        // simulation objects
        data,
        mods,
    };
    sim_round(&mut control);
    println!("next");
    sim_round(&mut control);
}
