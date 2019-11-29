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
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");

  let canvas = document.get_element_by_id("scene").expect("document should havea canvas named scene");
  let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("");

  let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
  let fill_style = JsValue::from_str("#000000");
  context.set_fill_style(&fill_style);
  context.fill_rect(0.0,0.0,10.0,10.0);

  //let val = document.create_element("p")?;
  //val.set_inner_html("hello from rust");

  //body.append_child(&val)?;

  Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
  a + b
}

#[wasm_bindgen]
pub fn print_sphere(idx: i32) {
  alert(&idx.to_string());
}
