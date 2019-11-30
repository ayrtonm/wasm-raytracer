extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Point {
  x: f64, y: f64, z: f64
}
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Ray {
  origin: Point,
  direction: Point,
}
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Color {
  r: u8, g: u8, b: u8, a: u8
}
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Sphere {
  center: Point,
  radius: f64,
  color: Color,
}

#[wasm_bindgen]
impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
  }
  pub fn origin() -> Point {
    Point::new(0.0, 0.0, 0.0)
  }
  pub fn x(&self) -> f64 { self.x }
  pub fn y(&self) -> f64 { self.y }
  pub fn z(&self) -> f64 { self.z }
  pub fn norm(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }
  pub fn add(&self, other: Point) -> Point {
    Point {
      x: self.x + other.x(),
      y: self.y + other.y(),
      z: self.z + other.z(),
    }
  }
  pub fn sub(&self, other: Point) -> Point {
    self.add(other.mult(-1.0))
  }
  pub fn shift(&self, t: f64) -> Point {
    Point {
      x: self.x + t,
      y: self.y + t,
      z: self.z + t,
    }
  }
  pub fn mult(&self, t: f64) -> Point {
    Point {
      x: self.x * t,
      y: self.y * t,
      z: self.z * t,
    }
  }
}

pub fn dot(a: Point, b: Point) -> f64 {
  (a.x() * b.x()) + (a.y() * b.y()) + (a.z() * b.z())
}

#[wasm_bindgen]
impl Ray {
  pub fn new(origin: Point, direction: Point) -> Ray {
    Ray { origin, direction }
  }
  pub fn origin(&self) -> Point { self.origin }
  pub fn direction(&self) -> Point { self.direction }
  pub fn value(&self, t: f64) -> Point {
    self.origin.add(self.direction.mult(t))
  }
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
impl Sphere {
  pub fn new(center: Point, radius: f64, color: Color) -> Sphere {
    Sphere { center, radius, color }
  }
  pub fn set_center(&mut self, x: f64, y: f64) {
    self.center = Point::new(x, y, self.center.z);
  }
  pub fn center(&self) -> Point { self.center }
  pub fn radius(&self) -> f64 { self.radius }
  pub fn color(&self) -> Color { self.color }
  pub fn intersect(&self, r: Ray) -> Option<(f64)> {
    let a: f64 = dot(r.direction(), r.direction());
    let dr: Point = r.origin().sub(self.center);
    let b: f64 = 2.0 * dot(r.direction(), dr);
    let c: f64 = dot(dr, dr) - self.radius.powi(2);
    let discriminant: f64 = b.powi(2) - 4.0*a*c;
    if discriminant > 0.0 {
      Some(-(b + discriminant.sqrt()) / (a * 2.0))
    } else {
      None
    }
  }
  pub fn normal(&self, p: Point) -> Point {
    let n = p.sub(self.center);
    n.mult(1.0/n.norm())
  }
}
