use opengl_graphics::GlGraphics;
use piston_window::Context;
use std::collections::{HashMap, HashSet};
use crate::{Control, Mod, graphics};

#[derive(PartialEq, Eq, Hash)]
pub enum ModId {
    PENDULUM,
}

#[derive(Clone, Copy)]
pub struct Polar {
  theta: (f64, f64),
  length: f64,
}

#[derive(Clone, Copy)]
struct Pendulum {
  polar: Polar,
  point: (f64, f64),
  dt: f64,
  g: f64,
}

#[derive(Clone, Copy)]
pub struct ModValue {
    pendulum: Option<Pendulum>,
}

pub fn polar_to_cartesian (Polar { theta, length }: &Polar) -> ((f64, f64), (f64, f64), (f64, f64)) {
  let x = length * theta.0.sin();
  let y = -length * theta.0.cos();
  (
    (0., 0.),
    (x, y),
    (x + length * theta.1.sin(), y - length * theta.1.cos())
  )
}
mod obj {
  use piston_window::{Line, Context};
  use piston_window::line::Shape;
  use crate::Control;
  use opengl_graphics::GlGraphics;
  use crate::example::double_pendulum::{Pendulum, ModValue, Polar, polar_to_cartesian};

  pub fn step (id: &isize, value: &ModValue) -> ModValue {
    let Pendulum {polar, point, dt, g} = value.pendulum.unwrap();
    let theta = polar.theta;
    let length = polar.length;
  
    let expr1 = (theta.0 - theta.1).cos();
    let expr2 = (theta.0 - theta.1).sin();
    let expr3 = 1f64 + expr2.powf(2f64);
    let expr4 = (point.0 * point.1 * expr2) / expr3; 
    let expr5 = (point.0.powf(2f64) + 2f64 * point.1.powf(2f64) - 
      point.0 * point.1 * expr1) * (2f64 * (theta.0 - theta.1)).sin() 
      / 2f64 / expr3.powf(2f64);
    let expr6 = expr4 - expr5;
    let new_polar = Polar {
      theta: (
        theta.0 + dt * (point.0 - point.1 * expr1) / expr3,
        theta.1 + dt * (2f64 * point.1 - point.0 * expr1) / expr3,
      ),
      length: length,
    };
    ModValue {
      pendulum: Some(Pendulum {
        polar: new_polar,
        point: (
          point.0 + dt * (-2f64 * g * length * theta.0.sin() - expr6),
          point.1 + dt * (-g * length * theta.1.sin() + expr6),
        ),
        dt: dt,
        g: g,
      })
    }
  }
  
  pub fn draw(c: &Context, g: &mut GlGraphics, value: &ModValue) {
    let cartesian = polar_to_cartesian(&value.pendulum.unwrap().polar);
    Line::new([1.0, 0.0, 0.0, 1.0], 1.0).draw_from_to([(20.0 + 10.0 * cartesian.0.0) * 30.0, (20.0 - 10.0 * cartesian.0.1) * 30.0], [(20.0 + 10.0 * cartesian.1.0) * 30.0, (20.0 - 10.0 * cartesian.1.1) * 30.0], &c.draw_state, c.transform, g);
    Line::new([1.0, 0.0, 0.0, 1.0], 1.0).draw_from_to([(20.0 + 10.0 * cartesian.1.0) * 30.0, (20.0 - 10.0 * cartesian.1.1) * 30.0], [(20.0 + 10.0 * cartesian.2.0) * 30.0, (20.0 - 10.0 * cartesian.2.1) * 30.0], &c.draw_state, c.transform, g);
    ()
  }

  pub fn postProcess(c: &Context, g: &mut GlGraphics, value: &ModValue) {

  }
  
}
fn print_cartesian(input: &((f64, f64), (f64, f64), (f64, f64))) -> String {
  format!("(({}, {}), ({}, {}), ({}, {}))", input.0.0, input.0.1, input.1.0, input.1.1, input.2.0, input.2.1)
}

pub fn main () {
  let polar =  Polar {
    theta: (std::f64::consts::PI, std::f64::consts::PI - 0.01), // can change
    length: 1.0f64,
  };
  let mut pendulum = Some(Pendulum {
    polar: polar,
    point: (0f64, 0f64),
    dt: 0.01f64, // can change
    g: 9.81,
  });
  let mut data = HashMap::new();
  data.insert(
      0,
      ModValue {
        pendulum,
      },
  );
  let mut step = HashMap::new();
  {
      let mut set = HashSet::new();
      set.insert(0);
      step.insert(
        ModId::PENDULUM,
          Box::new(Mod {
              function: obj::step as fn(&isize, &ModValue) -> ModValue,
              value: set,
          }),
      );
  }
  let mut draw = vec!();
  {
      let mut graphic = HashMap::new();
      let mut set = HashSet::new();
      set.insert(0);
      graphic.insert(
          ModId::PENDULUM,
          Box::new(Mod {
              function: obj::draw as fn(&Context, &mut GlGraphics, &ModValue) -> (),
              value: set,
          }),
      );
      draw.push(graphic);
  }
  // {
  //     let mut graphic = HashMap::new();
  //     let mut set = HashSet::new();
  //     set.insert(0);
  //     graphic.insert(
  //         ModId::PENDULUM,
  //         Box::new(Mod {
  //             function: obj::postProcess as fn(&Context, &mut GlGraphics, &ModValue) -> (),
  //             value: set,
  //         }),
  //     );
  //     draw.push(graphic);
  // }
  let mut control: Control<ModValue, ModId> = Control {
      index: 2,
      // simulation objects
      data,
      step,
      draw,
  };
  graphics::run(&mut control);
}

mod tests {
  use super::*;
  #[test]
  fn init () {
    let polar =  Polar {
      theta: (std::f64::consts::PI, std::f64::consts::PI - 0.01), // can change
      length: 1.0f64,
    };
    Pendulum {
      polar: polar,
      point: (0f64, 0f64),
      dt: 0.01f64, // can change
      g: 9.81,
    };
  }
  
  #[test]
  fn polar_to_cartesian_runs () {
    let polar =  Polar {
      theta: (0f64, std::f64::consts::PI / 2f64), // can change
      length: 1.0f64,
    };
    let out = polar_to_cartesian(&polar);
    assert_eq!(out.0, (0f64, 0f64));
    assert_eq!((out.1.0.round(), out.1.1.round()), (0f64, -1f64));
    assert_eq!((out.2.0.round(), out.2.1.round()), (1f64, -1f64));
  }
}
