mod vector;
mod ray;
mod color;
mod material;
mod sphere;
mod rng;

extern crate wasm_bindgen;

use rng::*;
use vector::*;
use ray::*;
use color::*;
use material::*;
use sphere::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Scene {
  spheres: Vec<Sphere>,
  eye: Vector,
  top_left: Vector,
  horizontal: Vector,
  vertical: Vector,
  light_source: Vector,
  wx: usize,
  wy: usize,
  framebuffer: Vec<u8>,
}

const BYTES_PER_PIXEL: usize = 4;

#[wasm_bindgen]
impl Scene {
  pub fn new(wx: usize, wy: usize) -> Self {
    let spheres = vec![Sphere::new(Vector::new(0.0,0.0,-2.0), 0.5, Color::new(255,0,0,255),Material::Metal),
                       Sphere::new(Vector::new(1.25,0.0,-2.0), 0.5, Color::new(0,255,0,255),Material::Metal),
                       Sphere::new(Vector::new(-1.25,0.0,-2.0), 0.5, Color::new(0,0,255,255),Material::Metal)];
    let eye = Vector::origin();
    let top_left = Vector::new(-2.0, -2.0, -2.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 4.0, 0.0);
    let mut light_source = Vector::new(-1.0, -1.0, 3.0);
    light_source.normalize();
    let framebuffer = vec![0; (wx * wy * BYTES_PER_PIXEL).into()];
    Scene { spheres, eye, top_left, horizontal, vertical, light_source, wx, wy, framebuffer, }
  }
  pub fn make_sphere(&mut self, x: f64, y: f64) {
    let rand_mat = if randbool() {
      Material::Metal
    } else {
      Material::Matte
    };
    self.spheres.push(Sphere::new(self.frac_to_vector(x, y),
                                  (randf64() % 0.45) + 0.15,
                                  Color::rand_color(),
                                  rand_mat));
  }
  pub fn move_sphere(&mut self, idx: usize, x: f64, y: f64) {
    let p = self.frac_to_vector(x,y);
    self.spheres[idx].set_position(&p);
  }
  pub fn delete_sphere(&mut self, idx: usize) {
    self.spheres.remove(idx);
  }
  pub fn hit_sphere(&mut self, x: f64, y: f64) -> usize {
    let r = self.frac_to_ray(x, y);
    match self.spheres
              .iter()
              .map(|s| s.intersect(&r))
              .enumerate()
              .filter(|(_,y)| y.is_some())
              .map(|(i,y)| (i,y.unwrap()))
              .min_by_key(|(_,t)| (t * 100.0) as u64) {
                Some((i,_)) => i,
                None => self.sphere_count(),
              }
  }
  pub fn framebuffer(&self) -> *const u8 { self.framebuffer.as_ptr() }
  pub fn sphere_count(&self) -> usize { self.spheres.len() }
  pub fn render(&mut self) {
    for y in 0..self.wy {
      for x in 0..self.wx {
        let ray = self.xy_to_ray(x,y);
        let col = self.color_pixel(&ray);
        self.set_framebuffer(x, y, &col);
      }
    }
  }
}

impl Scene {
  fn set_framebuffer(&mut self, x: usize, y: usize, col: &Color) {
    let idx = (x + (y * self.wx)) * BYTES_PER_PIXEL;
    self.framebuffer[idx] = col.red();
    self.framebuffer[idx + 1] = col.green();
    self.framebuffer[idx + 2] = col.blue();
    self.framebuffer[idx + 3] = col.alpha();
  }
  pub fn xy_to_frac(&self, x: usize, y: usize) -> (f64, f64) {
    ((x as f64) / (self.wx as f64),
     (y as f64) / (self.wy as f64))
  }
  pub fn xy_to_ray(&self, x: usize, y: usize) -> Ray {
    let (u,v) = self.xy_to_frac(x,y);
    self.frac_to_ray(u,v)
  }
  pub fn frac_to_vector(&self, x: f64, y: f64) -> Vector {
    self.top_left + (x * self.horizontal) + (y * self.vertical)
  }
  pub fn frac_to_ray(&self, x: f64, y: f64) -> Ray {
    Ray::new(self.eye, self.frac_to_vector(x, y))
  }
  fn bg_color() -> Color {
    Color::new(0,0,0,0)
  }
  fn color_pixel(&self, ray: &Ray) -> Color {
    match self.color_pixel_aux(ray, 0) {
      Some(color) => color,
      None => Scene::bg_color(),
    }
  }
  fn color_pixel_aux(&self, ray: &Ray, depth: u64) -> Option<Color> {
    self.spheres
        .iter()
        .map(|s| (s, s.intersect(&ray)))
        .filter(|(_,y)| y.is_some())
        .map(|(s,y)| (s, y.unwrap()))
        .min_by_key(|(_,t)| (t * 100.0) as u64)
        .map(|(s,t)| {
          if t < 10.0 && depth < 3 {
            //point where the ray hits the sphere
            let intersection_point = ray.value(t);
            //unit vector from this point perpendicular to sphere's surface
            let n_hat = s.normal(&intersection_point);
            let mix = n_hat * self.light_source;
            match s.material() {
              Material::Metal => {
                //compute direction of reflected ray
                let new_dir = ray.direction() - (2.0 * n_hat * (ray.direction() * n_hat));
                let new_ray = Ray::new(intersection_point, new_dir);
                //get Option<Color> from reflected ray
                (mix * *s.color()) + match self.color_pixel_aux(&new_ray, depth + 1) {
                  Some(reflected_color) => reflected_color,
                  None => Scene::bg_color(),
                }
              },
              Material::Dielectric => {
                //random direction
                let new_dir = n_hat + Vector::random_unit();
                let new_ray = Ray::new(intersection_point, new_dir);
                match self.color_pixel_aux(&new_ray, depth + 1) {
                  Some(reflected_color) => 0.5 * reflected_color,
                  None => 0.5 * *s.color(),
                }
              }
              Material::Matte => {
                mix * *s.color()
              }
            }
          } else {
            Scene::bg_color()
          }
        })
  }
}
