mod utils;
mod sphere;

extern crate rand;
extern crate web_sys;
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

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  Ok(())
}

#[wasm_bindgen]
pub struct Scene {
  canvas: web_sys::HtmlCanvasElement,
  spheres: Vec<Sphere>,
  bg: Color,
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
  pub fn new() -> Scene {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas = document.get_element_by_id("scene")
                         .expect("document should havea canvas named scene")
                         .dyn_into::<web_sys::HtmlCanvasElement>()
                         .expect("");
    let mut spheres = Vec::new();
    spheres.push(Sphere::new(Point::new(1.0, 1.0, 0.0),
                             5.0,
                             Color::new(255, 0, 0, 0)));
    let bg = Color::new(0,255,255,0);
    Scene {
      canvas,
      spheres,
      bg,
    }
  }
  pub fn render(&self) {
    let context = self.canvas.get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();
    let fill_style = JsValue::from_str("#0000FF");
    let width = self.canvas.width();
    let height = self.canvas.height();
    context.set_fill_style(&fill_style);
    context.fill_rect(0.0, 0.0, width.into(), height.into());

    for s in &self.spheres {
      let color = JsValue::from_str("#FF0000");
      context.begin_path();
      context.arc(s.center().x(), s.center().y(), s.radius(), 0.0, 2.0 * std::f64::consts::PI).unwrap();
      context.set_fill_style(&color);
      context.fill();
      context.set_line_width(1.0);
      context.set_stroke_style(&color);
      context.stroke();
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
  pub fn sphere(&self, idx: usize) -> Sphere { self.spheres[idx] }
  pub fn sphere_count(&self) -> usize { self.spheres.len() - 1 }
}
