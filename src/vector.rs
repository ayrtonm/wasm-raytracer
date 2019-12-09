use std::ops;
use crate::rng::*;

#[derive(Clone, Copy)]
pub struct Vector {
  x: f64, y: f64, z: f64
}

impl Vector {
  fn x(&self) -> f64 { self.x }
  fn y(&self) -> f64 { self.y }
  fn z(&self) -> f64 { self.z }
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Vector { x, y, z }
  }
  pub fn origin() -> Self {
    Vector::new(0.0, 0.0, 0.0)
  }
  pub fn random_unit() -> Self {
    let mut v = Vector::new(randf64(), randf64(), randf64());
    v.normalize();
    v
  }
  pub fn norm(&self) -> f64 {
    (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
  }
  pub fn normalize(&mut self) {
    self.scale(1.0/self.norm());
  }
  pub fn shift(&mut self, other: &Vector) {
    *self = *self + *other;
  }
  pub fn scale(&mut self, other: f64) {
    *self = *self * other;
  }
}

impl ops::Add<Vector> for Vector {
  type Output = Vector;
  fn add(self, other: Vector) -> Vector {
    Vector::new(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
  }
}

impl ops::Sub<Vector> for Vector {
  type Output = Vector;
  fn sub(self, other: Vector) -> Vector {
    Vector::new(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
  }
}

impl ops::Mul<Vector> for Vector {
  type Output = f64;
  fn mul(self, other: Vector) -> f64 {
    (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
  }
}

impl ops::Mul<f64> for Vector {
  type Output = Vector;
  fn mul(self, other: f64) -> Vector {
    Vector::new(self.x() * other, self.y() * other, self.z() * other)
  }
}

impl ops::Mul<Vector> for f64 {
  type Output = Vector;
  fn mul(self, other: Vector) -> Vector {
    Vector::new(self * other.x(), self * other.y(), self * other.z())
  }
}

#[cfg(test)]
mod test {
  use super::Vector;
  #[test]
  fn basics() {
    let o = Vector::origin;
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);
    assert_eq!(x.x(), 1.0);
    assert_eq!(x.y(), 0.0);
    assert_eq!(x.z(), 0.0);
    assert_eq!(x * y, 0.0);
  }
  #[test]
  fn rng() {
    let v = Vector::random_unit();
    assert_eq!(v.norm(), 1.0);
  }
}
