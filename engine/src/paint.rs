use gl::types::GLuint;

use crate::graphics::Graphics;

pub struct Paint {
  vertices: Vec<f32>,
  vertex_buffer_object: GLuint,
}

impl Paint {
  pub fn new(vao: &GLuint) -> Self {
    Paint {
      vertices: vec![],
      vertex_buffer_object: *vao,
    }
  }
  pub fn create_triangle2D(&mut self, points: [[f32; 2]; 3]) {
    self.vertices.extend_from_slice(&points[0]);
    self.vertices.push(0f32);
    self.vertices.extend_from_slice(&points[1]);
    self.vertices.push(0f32);
    self.vertices.extend_from_slice(&points[2]);
    self.vertices.push(0f32);
  }
  pub fn draw_triangles(&self) {
    unsafe {
      gl::DrawArrays(
          gl::TRIANGLES, // mode
          0, // starting index in the enabled arrays
          3 as i32, // number of indices to be rendered
      );
    }
  }
  
  pub fn publish(&self) {
    println!("{} {}", self.vertices.len() / 3, self.vertices.len());
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
        self.vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
        gl::DYNAMIC_DRAW, // usage
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
  fn create_triangle2D() {
    let mut paint = Paint::new(&0);
    paint.create_triangle2D([[-0.5f32, -0.5f32], [0.5f32, -0.5f32], [0.0f32, 0.5f32]]);
    assert_eq!(paint.vertices[0], -0.5f32);
    assert_eq!(paint.vertices[1], -0.5f32);
    assert_eq!(paint.vertices[2], -0f32);
    assert_eq!(paint.vertices[3], 0.5f32);
    assert_eq!(paint.vertices[4], -0.5f32);
    assert_eq!(paint.vertices[5], -0f32);
    assert_eq!(paint.vertices[6], 0f32);
    assert_eq!(paint.vertices[7], 0.5f32);
    assert_eq!(paint.vertices[8], 0f32);
  }
}
