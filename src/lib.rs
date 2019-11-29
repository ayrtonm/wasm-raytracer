mod utils;

extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
  //let scene = Scene::new();
  //scene.render();
  Ok(())
}

//#[wasm_bindgen]
//struct Color {
//  r: u8, g: u8, b: u8
//}
//
//#[wasm_bindgen]
//struct Point {
//  x: f64, y: f64, z: f64
//}
//
//pub struct Sphere {
//  center: Point,
//  radius: f64,
//  color: Color,
//}

#[wasm_bindgen]
pub fn add() {
}

#[wasm_bindgen]
pub struct Scene {
  canvas: web_sys::HtmlCanvasElement,
  //spheres: Vec<Sphere>,
  //bg: color,
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
    //let spheres = Vec::new();
    Scene {
      canvas,
      //spheres,
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
  }
}
