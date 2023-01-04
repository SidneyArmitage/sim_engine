use gl::types::GLuint;

use crate::graphics::Graphics;

pub struct Paint {
  vertices: Vec<f32>,
  vertex_buffer_object: GLuint,
}

#[derive(Clone, Copy)]
pub struct Painted {
  start: usize,
  length: usize,
}

impl Paint {
  pub fn new(vao: &GLuint) -> Self {
    Paint {
      vertices: vec![],
      vertex_buffer_object: *vao,
    }
  }

  pub fn create_triangle2d(&mut self, points: &[[f32; 2]; 3]) -> Painted {
    let initial = self.vertices.len();
    self.vertices.extend_from_slice(&points[0]);
    self.vertices.push(0f32);
    self.vertices.extend_from_slice(&points[1]);
    self.vertices.push(0f32);
    self.vertices.extend_from_slice(&points[2]);
    self.vertices.push(0f32);
    Painted {
      start: initial,
      length: 9,
    }
  }

  pub fn draw_triangle2d(&mut self, painted: &Option<Painted>, points: &[[f32; 2]; 3]) -> Painted {
    if painted.is_some() {
      let unwrapped_painted = painted.unwrap();
      for i in 0..=2 {
        self
          .vertices[i * 3 + unwrapped_painted.start + 0] =  points[i][0];
        self
          .vertices[i * 3 + unwrapped_painted.start + 1] = points[i][1];
        self
          .vertices[i * 3 + unwrapped_painted.start + 2] = 0f32;
      }
      return unwrapped_painted;
    }
    self.create_triangle2d(points)
  }

  pub fn create_triangles2d(&mut self, points: &[[f32; 2]]) -> Painted {
    if points.len() % 3 != 0 {
      panic!("expected 3 points for each triangle.");
    }
    let initial = self.vertices.len();
    let length = points.len() / 3;
    for i in 0..length {
      self.vertices.extend_from_slice(&points[i * 3 + 0]);
      self.vertices.push(0f32);
      self.vertices.extend_from_slice(&points[i * 3 + 1]);
      self.vertices.push(0f32);
      self.vertices.extend_from_slice(&points[i * 3 + 2]);
      self.vertices.push(0f32);
    }
    Painted {
      start: initial,
      length: points.len() * 3,
    }
  }

  pub fn draw_triangles2d(&mut self, painted: &Option<Painted>, points: &[[f32; 2]]) -> Painted {
    if points.len() % 3 != 0 {
      panic!("expected 3 points for each triangle.");
    }
    if painted.is_some() {
      let unwrapped_painted = painted.unwrap();
      for i in 0..=(points.len() - 1) {
        self
          .vertices[i * 3 + unwrapped_painted.start + 0] =  points[i][0];
        self
          .vertices[i * 3 + unwrapped_painted.start + 1] = points[i][1];
        self
          .vertices[i * 3 + unwrapped_painted.start + 2] = 0f32;
      }
      return unwrapped_painted;
    }
    self.create_triangles2d(points)
  }

  pub fn set_draw_triangles(&self) {
    unsafe {
      gl::DrawArrays(
        gl::TRIANGLES, // mode
        0,             // starting index in the enabled arrays
        self.vertices.len() as i32,      // number of indices to be rendered
      );
    }
  }

  pub fn publish(&self) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
        self.vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
        gl::DYNAMIC_DRAW,                                   // usage
      );
    }
  }
}

pub fn clear() {
  unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
  }
}

mod tests {
  use super::*;

  #[test]
  fn create_two_triangles2d() {
    let mut paint = Paint::new(&0);
    let result = paint.create_triangles2d(&[
      [-0.5f32, -0.5f32],
      [0.5f32, -0.5f32],
      [0.0f32, 0.5f32],
      [-0.5f32, -0.5f32],
      [0.5f32, -0.5f32],
      [0.0f32, 0.5f32],
    ]);
    assert_eq!(paint.vertices[0], -0.5f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.5f32);
    assert_eq!(paint.vertices[8], 0f32);
    assert_eq!(paint.vertices[9], -0.5f32);
    assert_eq!(paint.vertices[10], -0.5f32);
    assert_eq!(paint.vertices[11], -0f32);
    assert_eq!(paint.vertices[12], 0.5f32);
    assert_eq!(paint.vertices[13], -0.5f32);
    assert_eq!(paint.vertices[14], -0f32);
    assert_eq!(paint.vertices[15], 0f32);
    assert_eq!(paint.vertices[16], 0.5f32);
    assert_eq!(paint.vertices[17], 0f32);
    assert_eq!(result.start, 0);
    assert_eq!(result.length, 18);
  }
  #[test]
  fn create_triangles2d() {
    let mut paint = Paint::new(&0);
    let result =
      paint.create_triangles2d(&[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]]);
    assert_eq!(paint.vertices[0], -0.5f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.5f32);
    assert_eq!(paint.vertices[8], 0f32);
    assert_eq!(result.start, 0);
    assert_eq!(result.length, 9);
  }

  #[test]
  #[should_panic(expected = "expected 3 points for each triangle.")]
  fn create_triangles2d_panic() {
    let mut paint = Paint::new(&0);
    paint.create_triangles2d(&[[0.5f32, -0.5f32], [0.0f32, 0.5f32]]);
  }
  #[test]
  #[should_panic(expected = "expected 3 points for each triangle.")]
  fn draw_triangles2d_panic() {
    let mut paint = Paint::new(&0);
    paint.draw_triangles2d(&None, &[[0.5f32, -0.5f32], [0.0f32, 0.5f32]]);
  }

  #[test]
  fn create_triangle2d() {
    let mut paint = Paint::new(&0);
    let result =
      paint.create_triangle2d(&[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]]);
    assert_eq!(paint.vertices[0], -0.5f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.5f32);
    assert_eq!(paint.vertices[8], 0f32);
    assert_eq!(result.start, 0);
    assert_eq!(result.length, 9);
  }

  #[test]
  fn create_two_triangle2d() {
    let mut paint = Paint::new(&0);
    paint.create_triangle2d(&[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]]);
    let result =
      paint.create_triangle2d(&[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]]);
    for i in 0..1 {
      assert_eq!(paint.vertices[i + 0], -0.5f32);
      assert_eq!(paint.vertices[i + 1], -0.5f32);
      assert_eq!(paint.vertices[i + 2], -0f32);
      assert_eq!(paint.vertices[i + 3], 0.5f32);
      assert_eq!(paint.vertices[i + 4], -0.5f32);
      assert_eq!(paint.vertices[i + 5], -0f32);
      assert_eq!(paint.vertices[i + 6], 0f32);
      assert_eq!(paint.vertices[i + 7], 0.5f32);
      assert_eq!(paint.vertices[i + 8], 0f32);
    }
    assert_eq!(result.start, 9);
    assert_eq!(result.length, 9);
  }

  #[test]
  fn draw_new_triangle2d() {
    let mut paint = Paint::new(&0);
    let result = paint.draw_triangle2d(
      &None,
      &[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]],
    );
    assert_eq!(paint.vertices[0], -0.5f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.5f32);
    assert_eq!(paint.vertices[8], 0f32);
    assert_eq!(result.start, 0);
    assert_eq!(result.length, 9);
  }
  
  #[test]
  fn draw_two_existing_triangles2d() {
    let mut paint = Paint::new(&0);
    let painted = paint.create_triangles2d(&[
      [-0.5f32, -0.5f32],
      [0.5f32, -0.5f32],
      [0.0f32, 0.5f32],
      [-0.5f32, -0.5f32],
      [0.5f32, -0.5f32],
      [0.0f32, 0.5f32],
    ]);
    let result = paint.draw_triangles2d(
      &Some(painted), &[
      [-1.5f32, -0.5f32],
      [0.5f32, -0.5f32],
      [0.0f32, 0.5f32],
      [-0.5f32, -0.5f32],
      [0.5f32, -0.5f32],
      [0.0f32, 1.5f32],
    ]);
    assert_eq!(paint.vertices[0], -1.5f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.5f32);
    assert_eq!(paint.vertices[8], 0f32);
    assert_eq!(paint.vertices[9], -0.5f32);
    assert_eq!(paint.vertices[10], -0.5f32);
    assert_eq!(paint.vertices[11], -0f32);
    assert_eq!(paint.vertices[12], 0.5f32);
    assert_eq!(paint.vertices[13], -0.5f32);
    assert_eq!(paint.vertices[14], -0f32);
    assert_eq!(paint.vertices[15], 0f32);
    assert_eq!(paint.vertices[16], 1.5f32);
    assert_eq!(paint.vertices[17], 0f32);
    assert_eq!(result.start, 0);
    assert_eq!(result.length, 18);
  }
  
  #[test]
  fn draw_existing_triangles2d() {
    let mut paint = Paint::new(&0);
    let painted = paint.draw_triangles2d(
      &None,
      &[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]],
    );
    let result = paint.draw_triangles2d(
      &Some(painted),
      &[[-0.1f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.1f32]],
    );
    assert_eq!(paint.vertices[0], -0.1f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.1f32);
    assert_eq!(paint.vertices[8], 0f32);
    assert_eq!(result.start, 0);
    assert_eq!(result.length, 9);
  }

  #[test]
  fn draw_existing_triangle2d() {
    let mut paint = Paint::new(&0);
    let painted = paint.draw_triangle2d(
      &None,
      &[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]],
    );
    let result = paint.draw_triangle2d(
      &Some(painted),
      &[[-0.1f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.1f32]],
    );
    assert_eq!(paint.vertices[0], -0.1f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.1f32);
    assert_eq!(paint.vertices[8], 0f32);
    assert_eq!(result.start, 0);
    assert_eq!(result.length, 9);
  }
  
  #[test]
  fn draw_existing_second_triangle2d() {
    let mut paint = Paint::new(&0);
    paint.draw_triangle2d(
      &None,
      &[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]],
    );
    let painted = paint.draw_triangle2d(
      &None,
      &[[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]],
    );
    let result = paint.draw_triangle2d(
      &Some(painted),
      &[[-0.1f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.1f32]],
    );
    assert_eq!(paint.vertices[9], -0.1f32);
    assert_eq!(paint.vertices[10], -0.5f32);
    assert_eq!(paint.vertices[11], -0f32);
    assert_eq!(paint.vertices[12], 0.5f32);
    assert_eq!(paint.vertices[13], -0.5f32);
    assert_eq!(paint.vertices[14], -0f32);
    assert_eq!(paint.vertices[15], 0f32);
    assert_eq!(paint.vertices[16], 0.1f32);
    assert_eq!(paint.vertices[17], 0f32);
    assert_eq!(result.start, 9);
    assert_eq!(result.length, 9);
  }
}
