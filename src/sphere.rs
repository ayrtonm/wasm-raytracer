extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Color {
  r: u8, g: u8, b: u8, a: u8
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
impl Color {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a}
  }
  pub fn r(&self) -> u8 { self.r }
  pub fn g(&self) -> u8 { self.g }
  pub fn b(&self) -> u8 { self.b }
  pub fn a(&self) -> u8 { self.a }
}

#[wasm_bindgen]
impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
  }
  pub fn x(&self) -> f64 { self.x }
  pub fn y(&self) -> f64 { self.y }
  pub fn z(&self) -> f64 { self.z }
}

#[wasm_bindgen]
impl Sphere {
  pub fn new(center: Point, radius: f64, color: Color) -> Sphere {
    Sphere { center, radius, color }
  }
  pub fn set_center(&mut self, x: f64, y: f64) {
    self.center = Point::new(x, y, 0.0);
  }
  pub fn center(&self) -> Point { self.center }
  pub fn radius(&self) -> f64 { self.radius }
  pub fn color(&self) -> Color { self.color }
}
