mod utils;
mod sphere;

extern crate wasm_bindgen;

use sphere::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

type dim = usize;

#[wasm_bindgen]
pub struct Scene {
  wx: dim,
  wy: dim,
  eye: Point,
  top_left: Point,
  horizontal: Point,
  vertical: Point,
  spheres: Vec<Sphere>,
  framebuffer: Vec<u8>,
  bgColor: Color,
}

#[wasm_bindgen]
impl Scene {
  pub fn new(wx: dim, wy: dim) -> Scene {
    let spheres = Vec::new();
    let eye = Point::origin();
    let top_left = Point::new(-2.0,-1.0,-2.0);
    let horizontal = Point::new(4.0, 0.0, 0.0);
    let vertical = Point::new(0.0, 4.0, 0.0);
    let framebuffer = vec![0; (wx*wy*4).into()];
    let bgColor = Color::new(0, 0, 255, 255);
    Scene {
      wx, wy,
      eye,
      top_left,
      horizontal,
      vertical,
      spheres,
      framebuffer,
      bgColor,
    }
  }
  fn set_framebuffer(&mut self, x: usize, y: usize, color: Color) {
    let idx = (x + (y * self.wx)) * 4;
    self.framebuffer[idx] = color.r();
    self.framebuffer[idx + 1] = color.g();
    self.framebuffer[idx + 2] = color.b();
    self.framebuffer[idx + 3] = color.a();
  }
  pub fn render(&mut self) {
    let red = Color::new(255, 0, 0, 255);
    let blue = Color::new(0, 0, 255, 255);
    for y in 0..self.wy {
      for x in 0..self.wx {
        let u = (x / self.wx) as f64;
        let v = (y / self.wy) as f64;
        let r = Ray::new(self.eye, self.top_left.add(
                                   self.horizontal.mult(u).add(
                                   self.vertical.mult(v))));
        let col = self.hit_sphere(r);
        self.set_framebuffer(x, y, col);
      }
    }
  }
  pub fn make_sphere(&mut self, x: f64, y: f64, z: f64, radius: f64) {
    self.spheres.push(Sphere::new(Point::new(x, y, z),
                                  radius,
                                  Color::new(255, 0, 0, 255)));
  }
  pub fn move_sphere(&mut self, idx: usize, x: f64, y: f64) {
    self.spheres[idx].set_center(x,y);
  }
  pub fn delete_sphere(&mut self, idx: usize) {
    self.spheres.remove(idx);
  }
  fn normal_to_color(n: Point) -> Color {
    let m = n.shift(1.0).mult(0.5).mult(255.99);
    alert(&m.x().to_string());
    Color::new(m.x() as u8, m.y() as u8, m.z() as u8, 255)
  }
  pub fn hit_sphere(&self, ray: Ray) -> Color {
    match self.spheres.iter().find_map(|&s| s.intersect(ray)) {
      Some(tup) => Scene::normal_to_color(tup.0.normal(ray.value(tup.1))),
      None => self.bgColor,
    }
  }
  pub fn framebuffer(&self) -> *const u8 { self.framebuffer.as_ptr() }
  pub fn sphere(&self, idx: usize) -> Sphere { self.spheres[idx] }
  pub fn sphere_count(&self) -> usize {
    self.spheres.len()
  }
}
