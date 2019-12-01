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

type Dim = usize;

#[wasm_bindgen]
pub struct Scene {
  wx: Dim,
  wy: Dim,
  eye: Point,
  top_left: Point,
  horizontal: Point,
  vertical: Point,
  spheres: Vec<Sphere>,
  framebuffer: Vec<u8>,
  fg_color: Color,
}

#[wasm_bindgen]
impl Scene {
  pub fn new(wx: Dim, wy: Dim) -> Scene {
    let mut spheres = Vec::new();
    let eye = Point::origin();
    let top_left = Point::new(-2.0,-2.0,-2.0);
    let horizontal = Point::new(4.0, 0.0, 0.0);
    let vertical = Point::new(0.0, 4.0, 0.0);
    let framebuffer = vec![0; (wx*wy*4).into()];
    let fg_color = Color::new(255, 0, 0, 255);
    spheres.push(Sphere::new(Point::new(0.0, 0.0, -2.0), 0.5, Color::new(255,255,0,255)));
    spheres.push(Sphere::new(Point::new(1.0, 0.0, -2.0), 0.5, Color::new(0,255,255,255)));
    spheres.push(Sphere::new(Point::new(-1.0, 0.0, -2.0), 0.5, Color::new(255,0,255,255)));
    Scene {
      wx, wy,
      eye,
      top_left,
      horizontal,
      vertical,
      spheres,
      framebuffer,
      fg_color,
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
    for y in 0..self.wy {
      for x in 0..self.wx {
        let u = (x as f64) / (self.wx as f64);
        let v = (y as f64) / (self.wy as f64);
        let dir = self.top_left.add(
                  self.horizontal.mult(u)).add(
                  self.vertical.mult(v));
        let r = Ray::new(self.eye, dir);
        let col = match self.color_pixel(r) {
          Some(col) => col,
          None => Scene::bg_color(r),
        };
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
  //pub fn hit_sphere(&self) -> usize {
  //  let distances = self.spheres.iter().map(|s| s.intersect(ray));
  //}
  fn normal_to_color(n: Point) -> Color {
    let m = n.shift(1.0).mult(0.5).mult(255.99);
    Color::new(m.x() as u8, m.z() as u8, m.y() as u8, 255)
  }
  fn bg_color(ray: Ray) -> Color {
    let r = 255.99 * ((ray.direction().normalize().x() + 1.0) / 2.0);
    let b = 255.99 * ((ray.direction().normalize().y() + 1.0) / 2.0);
    let g = 255.99 * 0.2;
    Color::new(r as u8, g as u8, b as u8, 255)
  }
  pub fn color_pixel(&self, ray: Ray) -> Option<Color> {
    //distances to all spheres that intersect this ray
    let distances = self.spheres.iter().map(|s| s.intersect(ray));
    //if no spheres intersect the ray return the background color
    if distances.clone().all(|d| d.is_none()) {
      None
    } else {
      //get the closest sphere
      let closest = self.spheres.iter()
                                .min_by_key(|s| {
                                  match s.intersect(ray) {
                                    Some(val) => (val * 100.0) as u64,
                                    None => std::u64::MAX,
                                  }
                                }).unwrap();
      //get the ray parameter `t` at which the sphere and the ray intersect
      let t: f64 = closest.intersect(ray).unwrap();
      //get the Point at which the sphere and ray intersect
      let val = ray.value(t);
      //get the normal to the surface of the sphere
      let n = closest.normal(val);
      //pick colors based on the direction of this surface normal
      Some(Scene::normal_to_color(n))
    }
  }
  pub fn framebuffer(&self) -> *const u8 { self.framebuffer.as_ptr() }
  pub fn sphere(&self, idx: usize) -> Sphere { self.spheres[idx] }
  pub fn sphere_count(&self) -> usize {
    self.spheres.len()
  }
}
