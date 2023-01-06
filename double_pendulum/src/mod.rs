extern crate engine;

use engine::{
  graphics::{self, init_default_program, Graphics},
  paint::{Paint, Painted},
  start, App, Control, Draw, Mod,
};
use std::collections::{HashMap, HashSet};

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
  painted: Option<Painted>,
}

pub fn polar_to_cartesian(Polar { theta, length }: &Polar) -> [[f64; 2]; 3] {
  let x = length * theta.0.sin();
  let y = -length * theta.0.cos();
  [
    [0., 0.],
    [x, y],
    [x + length * theta.1.sin(), y - length * theta.1.cos()],
  ]
}


pub fn matrix_rotate2d(radians: f32, input: [f32; 2]) -> [f32; 2] {
  let cos = radians.cos();
  let sin = radians.sin();
  [
    input[0] * cos - input[1] * sin,
    input[0] * sin + input[1] * cos,
  ]
}

mod obj {
  use std::f32::consts::PI;

use engine::graphics::program::{self, Program};
  use engine::paint::{clear, Paint};

  use crate::{Control, matrix_rotate2d};
  use crate::{polar_to_cartesian, ModValue, Pendulum, Polar};

  pub fn step(delta_time: u128, id: &isize, value: &ModValue) -> ModValue {
    let Pendulum {
      polar,
      point,
      dt,
      g,
    } = value.pendulum.unwrap();

    let time = ((delta_time as f64 / 1000f64) * 0.0000005f64);
    let theta = polar.theta;
    let length = polar.length;

    let expr1 = (theta.0 - theta.1).cos();
    let expr2 = (theta.0 - theta.1).sin();
    let expr3 = 1f64 + expr2.powf(2f64);
    let expr4 = (point.0 * point.1 * expr2) / expr3;
    let expr5 = (point.0.powf(2f64) + 2f64 * point.1.powf(2f64) - point.0 * point.1 * expr1)
      * (2f64 * (theta.0 - theta.1)).sin()
      / 2f64
      / expr3.powf(2f64);
    let expr6 = expr4 - expr5;
    let new_polar = Polar {
      theta: (
        theta.0 + time * (point.0 - point.1 * expr1) / expr3,
        theta.1 + time * (2f64 * point.1 - point.0 * expr1) / expr3,
      ),
      length: length,
    };
    ModValue {
      pendulum: Some(Pendulum {
        polar: new_polar,
        point: (
          point.0 + time * (-2f64 * g * length * theta.0.sin() - expr6),
          point.1 + time * (-g * length * theta.1.sin() + expr6),
        ),
        dt: dt,
        g: g,
      }),
      painted: value.painted,
    }
  }

  pub fn draw(paint: &mut Paint, value: &ModValue) -> ModValue {
    let Pendulum {
      polar,
      point,
      dt,
      g,
    } = value.pendulum.unwrap();
    let points = polar_to_cartesian(&polar);
    // polar + 90 deg * len + cartesian
    let start_rotation = 90f32.to_radians() + (polar.theta.0 as f32);
    let end_rotation = 90f32.to_radians() + (polar.theta.1 as f32);
    //polar.theta.0 as f32
    let start_right = matrix_rotate2d(start_rotation, [0., 0.01]);
    let start_left = matrix_rotate2d(start_rotation, [0., -0.01]);
    let end_right = matrix_rotate2d(end_rotation, [0., 0.01]);
    let end_left = matrix_rotate2d(end_rotation, [0., -0.01]);
    let painted = paint.draw_triangles2d(
      &value.painted,
      &[
        start_left,
        start_right,
        [points[1][0] as f32 * 0.5 + start_right[0], points[1][1] as f32 * 0.5 + start_right[1]],
        start_left,
        [points[1][0] as f32 * 0.5 + start_left[0], points[1][1] as f32 * 0.5 + start_left[1]],
        [points[1][0] as f32 * 0.5 + start_right[0], points[1][1] as f32 * 0.5 + start_right[1]],
        [
          points[1][0] as f32 * 0.5 + end_left[0],
          points[1][1] as f32 * 0.5 + end_left[1],
        ],
        [
          points[1][0] as f32 * 0.5 + end_right[0],
          points[1][1] as f32 * 0.5 + end_right[1],
        ],
        [points[2][0] as f32 * 0.5 + end_right[0], points[2][1] as f32 * 0.5 + end_right[1]],
        [
          points[1][0] as f32 * 0.5 + end_left[0],
          points[1][1] as f32 * 0.5 + end_left[1],
        ],
        [points[2][0] as f32 * 0.5 + end_left[0], points[2][1] as f32 * 0.5 + end_left[1]],
        [points[2][0] as f32 * 0.5 + end_right[0], points[2][1] as f32 * 0.5 + end_right[1]],
      ],
    );
    ModValue {
      pendulum: value.pendulum,
      painted: Some(painted),
    }
  }

  pub fn draw_post(paint: &Paint) {
    paint.set_draw_triangles();
    paint.publish();
  }
  pub fn draw_pre(program: &Program) {
    program.set_used();
    clear();
  }
}

pub fn init(graphics: Graphics) -> App<ModValue, ModId> {
  let polar = Polar {
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
      painted: None,
    },
  );
  let mut step = HashMap::new();
  {
    let mut set = HashSet::new();
    set.insert(0);
    step.insert(
      ModId::PENDULUM,
      Box::new(Mod {
        function: obj::step as fn(u128, &isize, &ModValue) -> ModValue,
        value: set,
      }),
    );
  }
  let mut draw = {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    set.insert(0);
    map.insert(
      ModId::PENDULUM,
      Box::new(Mod {
        function: obj::draw as fn(&mut Paint, &ModValue) -> ModValue,
        value: set,
      }),
    );
    let program = init_default_program().unwrap();
    program.set_used();
    let uniform_ptr = program.get_uniform::<f32>("u_aspectRatio");
    uniform_ptr.set_uniform(700. / 900.);
    let paint = Paint::new(&graphics.get_vertex_buffer());
    vec![Draw {
      map,
      post: obj::draw_post,
      pre: obj::draw_pre,
      program,
      paint,
    }]
  };
  App::new(
    Control {
      index: 2,
      // simulation objects
      data,
      draw,
      step,
    },
    graphics,
  )
}

fn main() {
  start(init);
}

mod tests {
  use super::*;
  #[test]
  fn init() {
    let polar = Polar {
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
  fn polar_to_cartesian_runs() {
    let polar = Polar {
      theta: (0f64, std::f64::consts::PI / 2f64), // can change
      length: 1.0f64,
    };
    let out = polar_to_cartesian(&polar);
    assert_eq!([0f64, 0f64], out[0]);
    assert_eq!([0f64, -1f64], [out[1][0].round(), out[1][1].round()]);
    assert_eq!([1f64, -1f64], [out[2][0].round(), out[2][1].round()]);
  }
}
