mod utils;

extern crate rand;
extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//import functions from scene.js
#[link(wasm_import_module = "scene")]
extern {
    fn colorToString(r: u8, g: u8, b: u8) -> String;
}
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  Ok(())
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Color {
  r: u8, g: u8, b: u8
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Point {
  x: f64, y: f64, z: f64
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Sphere {
  center: Point,
  radius: f64,
  color: Color,
}

#[wasm_bindgen]
pub struct Scene {
  canvas: web_sys::HtmlCanvasElement,
  spheres: Vec<Sphere>,
  //bg: Color,
}

fn rand_color() -> Color {
  let mut rng = rand::thread_rng();
  let r: u8 = rng.gen();
  let g: u8 = rng.gen();
  let b: u8 = rng.gen();
  Color { r, g, b }
}

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
    spheres.push(Sphere {
      center: Point {
        x: 1.0, y: 1.0, z: 0.0
      },
      radius: 5.0,
      color: Color {
        r: 255, g: 0, b: 0
      }
    });
    Scene {
      canvas,
      spheres,
    }
  }
  pub fn render(&self) {
    let context = self.canvas.get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();
    let fill_style = JsValue::from_str(&colorToString(0,0,0xff));
    let width = self.canvas.width();
    let height = self.canvas.height();
    context.set_fill_style(&fill_style);
    context.fill_rect(0.0, 0.0, width.into(), height.into());

    for s in &self.spheres {
      alert(&self.spheres.len().to_string());
      let color = JsValue::from_str("#FF0000");
      context.begin_path();
      context.arc(s.center.x, s.center.y, s.radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
      context.set_fill_style(&color);
      context.fill();
      context.set_line_width(1.0);
      context.set_stroke_style(&color);
      context.stroke();
    }
  }
  //pub fn makeSphere(&mut self, center: &Point) {
  //  let mut rng = rand::thread_rng();
  //  let radius: f64 = rng.gen();
  //  let color = rand_color();
  //  self.spheres.push(Sphere { center: *center, radius, color } );
  //  self.render();
  //}
  //pub fn deleteSphere(&mut self, idx: usize) {
  //  self.spheres.remove(idx);
  //  self.render();
  //}
  //pub fn moveSphere(&mut self, idx: usize, center: &Point) {
  //  self.spheres[idx].center = *center;
  //  self.render();
  //}
  //pub fn hitSphere(&self, pos: &Point) -> usize {
  //  0
  //  //for s in &self.spheres {
  //  //  if s.x <= pos.x + s.radius && s.x >= pos.x - s.radius &&
  //  //     s.y <= pos.y + s.radius && s.y >= pos.y - s.radius {
  //  //       return i;
  //  //}
  //}
}
