extern crate engine;

use engine::{
  graphics::{self, init_default_program, Graphics},
  paint::{Paint, Painted},
  start, App, Control, Draw, Mod,
};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
pub enum ModId {
  TRIANGLE,
}

#[derive(Clone, Copy)]
struct Triangle {
  rotation: f32,
}

#[derive(Clone, Copy)]
pub struct ModValue {
  triangle: Option<Triangle>,
  painted: Option<Painted>,
}

mod obj {
  use engine::graphics::program::{self, Program};
  use engine::paint::{clear, Paint};

  use crate::Control;
  use crate::{ModValue, Triangle};

  use super::matrix_rotate2d;

  pub fn step(delta_time: u128, id: &isize, value: &ModValue) -> ModValue {
    ModValue {
      triangle: Some(Triangle {
        rotation: value.triangle.unwrap().rotation + (delta_time as f32 / 1000f32) * 0.0000005f32,
      }),
      painted: value.painted,
    }
  }
  /*
  x | sin cos
  y | cos -sin
  */
  pub fn draw(paint: &mut Paint, value: &ModValue) -> ModValue {
    let rotation = value.triangle.unwrap().rotation;
    let height = 60f32.to_radians().sin();
    let ratio = -1. / 3.;
    let painted = paint.draw_triangles2d(
      &value.painted,
      &[
        matrix_rotate2d(rotation, [-0.5f32, ratio * height]),
        matrix_rotate2d(rotation, [0.5f32, ratio * height]),
        matrix_rotate2d(rotation, [0.0f32, (1. + ratio) * height]),
      ],
    );

    ModValue {
      triangle: value.triangle,
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
  let mut triangle = Some(Triangle { rotation: 0f32 });
  let mut data = HashMap::new();
  data.insert(
    0,
    ModValue {
      triangle,
      painted: None,
    },
  );
  let mut step = HashMap::new();
  {
    let mut set = HashSet::new();
    set.insert(0);
    step.insert(
      ModId::TRIANGLE,
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
      ModId::TRIANGLE,
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

pub fn matrix_rotate2d(radians: f32, input: [f32; 2]) -> [f32; 2] {
  let cos = radians.cos();
  let sin = radians.sin();
  [
    input[0] * cos - input[1] * sin,
    input[0] * sin + input[1] * cos,
  ]
}

mod tests {
  extern crate float_cmp;
  use super::*;
  use float_cmp::*;
  #[test]
  fn rotate2d_0() {
    let init = [1f32, 0f32];
    let result = matrix_rotate2d(0f32, init);
    assert!(approx_eq!(f32, result[0], 1f32, epsilon = 0.0002));
    assert!(approx_eq!(f32, result[1], 0f32, epsilon = 0.0002));
  }

  #[test]
  fn rotate2d_90() {
    let init = [1f32, 0f32];
    let result = matrix_rotate2d(90f32.to_radians(), init);
    println!("{:#?}", result);
    assert!(approx_eq!(f32, result[0], 0f32, epsilon = 0.0002));
    assert!(approx_eq!(f32, result[1], 1f32, epsilon = 0.0002));
  }

  #[test]
  fn rotate2d_negative_90() {
    let init = [1f32, 0f32];
    let result = matrix_rotate2d(-90f32.to_radians(), init);
    println!("{:#?}", result);
    assert!(approx_eq!(f32, result[0], 0f32, epsilon = 0.0002));
    assert!(approx_eq!(f32, result[1], -1f32, epsilon = 0.0002));
  }

  #[test]
  fn rotate2d_180() {
    let init = [1f32, 0f32];
    let result = matrix_rotate2d(180f32.to_radians(), init);
    println!("{:#?}", result);
    assert!(approx_eq!(f32, result[0], -1f32, epsilon = 0.0002));
    assert!(approx_eq!(f32, result[1], 0f32, epsilon = 0.0002));
  }
}
