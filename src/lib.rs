mod utils;
mod sphere;

extern crate rand;
extern crate wasm_bindgen;

use sphere::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::Rng;

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
  spheres: Vec<Sphere>,
  bg: Color,
  framebuffer: Vec<u8>,
}

//fn rand_color() -> Color {
//  let mut rng = rand::thread_rng();
//  let r: u8 = rng.gen();
//  let g: u8 = rng.gen();
//  let b: u8 = rng.gen();
//  let a: u8 = rng.gen();
//  Color { r, g, b, a }
//}

#[wasm_bindgen]
impl Scene {
  pub fn new(wx: dim, wy: dim) -> Scene {
    let mut spheres = Vec::new();
    spheres.push(Sphere::new(Point::new(1.0, 1.0, 0.0),
                             5.0,
                             Color::new(255, 0, 0, 0)));
    let bg = Color::new(0,255,255,0);
    let framebuffer = vec![0; (wx*wy*4).into()];
    Scene {
      wx, wy,
      spheres,
      bg,
      framebuffer,
    }
  }
  pub fn render(&mut self) {
    for y in 0..self.wy {
      for x in 0..self.wx {
        match self.hit_sphere(x as f64,y as f64) {
          Some(idx) => {
            //set red
            let idx = (x + (y * self.wx)) * 4;
            self.framebuffer[idx as usize] = 0xff;
            self.framebuffer[(idx + 1) as usize] = 0x00;
            self.framebuffer[(idx + 2) as usize] = 0x00;
            self.framebuffer[(idx + 3) as usize] = 0x00;
          }
          None => {
            //set blue
            let idx = (x + (y * self.wx)) * 4;
            self.framebuffer[idx as usize] = 0x00;
            self.framebuffer[(idx + 1) as usize] = 0x00;
            self.framebuffer[(idx + 2) as usize] = 0xff;
            self.framebuffer[(idx + 3) as usize] = 0x00;
          }
        }
      }
    }
  }
  pub fn make_sphere(&mut self, x: f64, y: f64, z: f64, radius: f64) {
    self.spheres.push(Sphere::new(Point::new(x, y, z),
                                  radius,
                                  Color::new(255, 0, 0, 0)));
  }
  pub fn move_sphere(&mut self, idx: usize, x: f64, y: f64) {
    self.spheres[idx].set_center(x,y);
  }
  pub fn delete_sphere(&mut self, idx: usize) {
    self.spheres.remove(idx);
  }
  pub fn hit_sphere(&self, px: f64, py: f64) -> Option<usize> {
    self.spheres.iter().position(|&s| {
      let cx = s.center().x();
      let cy = s.center().y();
      let rsq = s.radius().powi(2);
      let dx = cx - px;
      let dy = cy - py;
      dx.powi(2) + dy.powi(2) < rsq
    })
  }
  pub fn framebuffer(&self) -> *const u8 { self.framebuffer.as_ptr() }
  pub fn sphere(&self, idx: usize) -> Sphere { self.spheres[idx] }
  pub fn sphere_count(&self) -> usize { self.spheres.len() - 1 }
}
