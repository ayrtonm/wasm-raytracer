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
    spheres.push(Sphere::new(Point::new(0.0, 0.0, -2.0), 0.5, Color::new(255,0,0,255)));
    spheres.push(Sphere::new(Point::new(1.0, 0.0, -2.0), 0.5, Color::new(0,255,0,255)));
    spheres.push(Sphere::new(Point::new(-1.0, 0.0, -2.0), 0.5, Color::new(0,0,255,255)));
    Scene {
      wx, wy,
      eye,
      top_left,
      horizontal,
      vertical,
      spheres,
      framebuffer,
    }
  }
  pub fn render(&mut self) {
    for y in 0..self.wy {
      for x in 0..self.wx {
        let r = self.xy_to_ray(x as u64, y as u64);
        let col = match self.color_pixel(&r) {
          Some(col) => col,
          None => Scene::bg_color(&r),
        };
        self.set_framebuffer(x, y, &col);
      }
    }
  }
  pub fn make_sphere(&mut self, x: f64, y: f64, radius: f64, r: u8, g: u8, b: u8) {
    self.spheres.push(Sphere::new(Point::new(x, y, -2.0),
                                  radius,
                                  Color::new(r, g, b, 255)));
  }
  //move the sphere idx to x, y while keeping z constant
  //this function is mostly commented out because it's too slow
  //it would be great to find a more efficient way to do these things
  pub fn move_sphere(&mut self, idx: usize, x: f64, y: f64) {
    //this keeps the distance to the point we grab constant
    //let r = self.xy_to_ray(x,y);
    //let t = self.spheres[idx].intersect(&r);
    //let d = r.value(t).norm();
    //let z = (d.powi(2) - x.powi(2) - y.powi(2)).sqrt();

    //this keeps the distance to the center of the sphere constant and is much more efficient
    //let z = (self.spheres[idx].distancesq() - x.powi(2) - y.powi(2)).sqrt();

    //this checks for collisions with another sphere, but currently seems too inefficient
    //let newpos = Point::new(x,y,-z);
    //let collision = self.spheres.iter()
    //                            .skip(idx)
    //                            .any(|&s| {
    //                              if s.center().sub(newpos).norm() < (s.radius() + self.spheres[idx].radius()) {
    //                                true
    //                              } else {
    //                                false
    //                              }
    //                            });
    //if !collision {
      //keep z constant for now
      let z = 2.0;
      self.spheres[idx].set_center(x,y,-z);
    //}
  }
  pub fn delete_sphere(&mut self, idx: usize) {
    self.spheres.remove(idx);
  }
  pub fn hit_sphere(&self, x: usize, y: usize) -> usize {
    let ray = self.xy_to_ray(x as u64, y as u64);
    let mut distances = self.spheres.iter().map(|&s| s.intersect(&ray));
    if distances.all(|d| d.is_none()) {
      self.spheres.len() + 1
    } else {
      //get the index of the closest sphere
      self.spheres.iter()
                  .enumerate()
                  .min_by_key(|&(_, s)| {
                    match s.intersect(&ray) {
                      Some(val) => (val * 100.0) as u64,
                      None => std::u64::MAX,
                    }
                  }).unwrap().0
    }
  }
  pub fn framebuffer(&self) -> *const u8 { self.framebuffer.as_ptr() }
  pub fn sphere_count(&self) -> usize {
    self.spheres.len()
  }
}

impl Scene {
  fn set_framebuffer(&mut self, x: usize, y: usize, color: &Color) {
    let idx = (x + (y * self.wx)) * 4;
    self.framebuffer[idx] = color.r();
    self.framebuffer[idx + 1] = color.g();
    self.framebuffer[idx + 2] = color.b();
    self.framebuffer[idx + 3] = color.a();
  }
  fn xy_to_frac(&self, x: u64, y: u64) -> (f64, f64) {
    let u = (x as f64) / (self.wx as f64);
    let v = (y as f64) / (self.wy as f64);
    (u, v)
  }
  fn xy_to_ray(&self, x: u64, y: u64) -> Ray {
    let (u, v) = self.xy_to_frac(x, y);
    let dir = self.top_left.add(
              self.horizontal.mult(u)).add(
              self.vertical.mult(v));
    Ray::new(self.eye, dir)
  }
  fn normal_to_color(c: &Color, n: &Point) -> Color {
    let v = Point::new(-1.0, -1.0, 1.0).normalize();
    c.shade(dot(n, &v))
  }
  fn bg_color(ray: &Ray) -> Color {
    let u = ray.direction().normalize();
    let t = 0.5*(1.0 - u.y() - u.z());
    let r = 255.99 * (1.0 - (0.5 * t));
    let g = 255.99 * (1.0 - (0.3 * t));
    let b = 255.99 * 1.0;
    Color::new(r as u8, g as u8, b as u8, 255)
  }
  pub fn color_pixel(&self, ray: &Ray) -> Option<Color> {
    //if no spheres intersect the ray return the background color
    let no_intersect = self.spheres.iter()
                           .all(|&s| {
                             match s.intersect(&ray) {
                               Some(_) => false,
                               None => true,
                             }
                           });
    if no_intersect {
      None
    } else {
      //get the closest sphere
      let closest = self.spheres.iter()
                                .min_by_key(|&s| {
                                  match s.intersect(&ray) {
                                    Some(val) => (val * 100.0) as u64,
                                    None => std::u64::MAX,
                                  }
                                }).unwrap();
      //get the ray parameter `t` at which the sphere and the ray intersect
      let t: f64 = closest.intersect(&ray).unwrap();
      //get the Point at which the sphere and ray intersect
      let val = ray.value(t);
      //get the normal to the surface of the sphere
      let n = closest.normal(&val);
      //pick colors based on the direction of this surface normal
      Some(Scene::normal_to_color(&closest.color(), &n))
    }
  }
}
