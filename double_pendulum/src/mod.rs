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
  theta: [f64; 2],
  length: f64,
}

#[derive(Clone, Copy)]
struct Pendulum {
  dt: f64,
  last_position: [f64; 2],
  point: [f64; 2],
  points: [[f64; 2]; 3],
  polar: Polar,
  g: f64,
}

#[derive(Clone, Copy)]
pub struct ModValue {
  pendulum: Option<Pendulum>,
  painted: Option<Painted>,
}

pub fn polar_to_cartesian(Polar { theta, length }: &Polar) -> [[f64; 2]; 3] {
  let x = length * theta[0].sin();
  let y = -length * theta[0].cos();
  [
    [0., 0.],
    [x, y],
    [x + length * theta[1].sin(), y - length * theta[1].cos()],
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

  use engine::graphics::program::{self, Program};
  use engine::paint::{clear, Paint};

  use crate::{matrix_rotate2d, Control};
  use crate::{polar_to_cartesian, ModValue, Pendulum, Polar};
  // there is a bug here due to time. When time is paused, some state leaks leading to a change in velocity of the simulation.
  pub fn step(_: u128, _: &isize, value: &ModValue) -> ModValue {
    let Pendulum {
      last_position: _,
      point,
      points,
      polar,
      dt,
      g,
    } = value.pendulum.unwrap();

    let time = 0.0005f64;
    let theta = polar.theta;
    let length = polar.length;

    let expr1 = (theta[0] - theta[1]).cos();
    let expr2 = (theta[0] - theta[1]).sin();
    let expr3 = 1f64 + expr2.powf(2f64);
    let expr4 = (point[0] * point[1] * expr2) / expr3;
    let expr5 = (point[0].powf(2f64) + 2f64 * point[1].powf(2f64) - point[0] * point[1] * expr1)
      * (2f64 * (theta[0] - theta[1])).sin()
      / 2f64
      / expr3.powf(2f64);
    let expr6 = expr4 - expr5;
    let new_polar = Polar {
      theta: [
        theta[0] + time * (point[0] - point[1] * expr1) / expr3,
        theta[1] + time * (2f64 * point[1] - point[0] * expr1) / expr3,
      ],
      length: length,
    };
    let cartesian = polar_to_cartesian(&polar);
    ModValue {
      pendulum: Some(Pendulum {
        last_position: points[2],
        polar: new_polar,
        points: cartesian,
        point: [
          point[0] + time * (-2f64 * g * length * theta[0].sin() - expr6),
          point[1] + time * (-g * length * theta[1].sin() + expr6),
        ],
        dt: dt,
        g: g,
      }),
      painted: value.painted,
    }
  }

  pub fn draw(paint: &mut Paint, value: &ModValue) -> ModValue {
    let Pendulum {
      last_position: _,
      polar,
      point: _1,
      points,
      dt: _2,
      g: _3,
    } = value.pendulum.unwrap();
    // polar + 90 deg * len + cartesian
    let start_rotation = 90f32.to_radians() + (polar.theta[0] as f32);
    let end_rotation = 90f32.to_radians() + (polar.theta[1] as f32);
    //polar.theta[0] as f32
    let start_right = matrix_rotate2d(start_rotation, [0., 0.01]);
    let start_left = matrix_rotate2d(start_rotation, [0., -0.01]);
    let end_right = matrix_rotate2d(end_rotation, [0., 0.01]);
    let end_left = matrix_rotate2d(end_rotation, [0., -0.01]);
    let painted = paint.draw_triangles2d(
      &value.painted,
      &[
        start_left,
        start_right,
        [
          points[1][0] as f32 * 0.5 + start_right[0],
          points[1][1] as f32 * 0.5 + start_right[1],
        ],
        start_left,
        [
          points[1][0] as f32 * 0.5 + start_left[0],
          points[1][1] as f32 * 0.5 + start_left[1],
        ],
        [
          points[1][0] as f32 * 0.5 + start_right[0],
          points[1][1] as f32 * 0.5 + start_right[1],
        ],
        [
          points[1][0] as f32 * 0.5 + end_left[0],
          points[1][1] as f32 * 0.5 + end_left[1],
        ],
        [
          points[1][0] as f32 * 0.5 + end_right[0],
          points[1][1] as f32 * 0.5 + end_right[1],
        ],
        [
          points[2][0] as f32 * 0.5 + end_right[0],
          points[2][1] as f32 * 0.5 + end_right[1],
        ],
        [
          points[1][0] as f32 * 0.5 + end_left[0],
          points[1][1] as f32 * 0.5 + end_left[1],
        ],
        [
          points[2][0] as f32 * 0.5 + end_left[0],
          points[2][1] as f32 * 0.5 + end_left[1],
        ],
        [
          points[2][0] as f32 * 0.5 + end_right[0],
          points[2][1] as f32 * 0.5 + end_right[1],
        ],
      ],
    );
    ModValue {
      pendulum: value.pendulum,
      painted: Some(painted),
    }
  }

  pub fn draw_trail(paint: &mut Paint, value: &ModValue) -> ModValue {
    let Pendulum {
      last_position,
      polar: _,
      point: _1,
      points,
      dt: _2,
      g: _3,
    } = value.pendulum.unwrap();
    let direction = [
      points[2][0] - last_position[0],
      points[2][1] - last_position[1],
    ];
    let magnitude = direction[0].abs() + direction[1].abs();
    // rotate by 90 degrees to make it easier to work with
    let normalised = [
      (direction[1] / magnitude) * 0.01,
      -(direction[0] / magnitude) * 0.01,
    ];
    if normalised[0] > 1f64 || normalised[1] > 1f64 {
      println!(
        "{:?} - {:?} = {:?} then {:?}",
        points[2], last_position, direction, normalised
      );
      panic!("invalid normal");
    }
    let scaled_last_position = [last_position[0] / 2.0, last_position[1] / 2.0];
    let scaled_point = [points[2][0] / 2.0, points[2][1] / 2.0];
    paint.create_triangles2d(&[
      [
        (scaled_last_position[0] + normalised[0]) as f32,
        (scaled_last_position[1] + normalised[1]) as f32,
      ],
      [
        (scaled_last_position[0] - normalised[0]) as f32,
        (scaled_last_position[1] - normalised[1]) as f32,
      ],
      [
        (scaled_point[0] + normalised[0]) as f32,
        (scaled_point[1] + normalised[1]) as f32,
      ],
      [
        (scaled_last_position[0] - normalised[0]) as f32,
        (scaled_last_position[1] - normalised[1]) as f32,
      ],
      [
        (scaled_point[0] + normalised[0]) as f32,
        (scaled_point[1] + normalised[1]) as f32,
      ],
      [
        (scaled_point[0] - normalised[0]) as f32,
        (scaled_point[1] - normalised[1]) as f32,
      ],
    ]);
    *value
  }

  pub fn draw_post(paint: &Paint) {}
  pub fn draw_pre(program: &Program) {
    program.set_used();
  }
}

fn init_draw(
  graphics: &Graphics,
  color_array: [f32; 4],
  draw_function: fn(&mut Paint, &ModValue) -> ModValue,
) -> Draw<ModValue, ModId> {
  let mut map = HashMap::new();
  let mut set = HashSet::new();
  set.insert(0);
  map.insert(
    ModId::PENDULUM,
    Box::new(Mod {
      function: draw_function,
      value: set,
    }),
  );
  let program = init_default_program().unwrap();
  program.set_used();
  let aspect = program.get_uniform::<f32>("u_aspectRatio");
  aspect.set_uniform(700. / 900.);
  let color = program.get_uniform::<[f32; 4]>("u_color");
  color.set_uniform(color_array);
  let paint = Paint::new(&graphics.get_vertex_buffer());
  Draw {
    map,
    post: obj::draw_post,
    pre: obj::draw_pre,
    program,
    paint,
  }
}

pub fn init(graphics: Graphics) -> App<ModValue, ModId> {
  let polar = Polar {
    theta: [std::f64::consts::PI, std::f64::consts::PI - 0.01], // can change
    length: 1.0f64,
  };
  let mut pendulum = Some(Pendulum {
    last_position: [0f64, 0f64],
    polar: polar,
    point: [0f64, 2f64],
    points: [[0f64, 0f64], [0f64, 0f64], [0f64, 0f64]],
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
   
    let draw_trail = init_draw(
      &graphics,
      [0.0, 0.0, 0.5, 1.0],
      obj::draw_trail as fn(&mut Paint, &ModValue) -> ModValue,
    );
    let draw_pendulum = init_draw(
      &graphics,
      [0.5, 0.0, 0.0, 1.0],
      obj::draw as fn(&mut Paint, &ModValue) -> ModValue,
    );
    vec![draw_trail, draw_pendulum]
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
      theta: [std::f64::consts::PI, std::f64::consts::PI - 0.01], // can change
      length: 1.0f64,
    };
    Pendulum {
      last_position: [0f64, 0f64],
      point: [0f64, 0f64],
      points: [[0f64, 0f64], [0f64, 0f64], [0f64, 0f64]],
      polar: polar,
      dt: 0.01f64, // can change
      g: 9.81,
    };
  }

  #[test]
  fn polar_to_cartesian_runs() {
    let polar = Polar {
      theta: [0f64, std::f64::consts::PI / 2f64], // can change
      length: 1.0f64,
    };
    let out = polar_to_cartesian(&polar);
    assert_eq!([0f64, 0f64], out[0]);
    assert_eq!([0f64, -1f64], [out[1][0].round(), out[1][1].round()]);
    assert_eq!([1f64, -1f64], [out[2][0].round(), out[2][1].round()]);
  }
}
