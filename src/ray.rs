use crate::vector::*;

#[derive(Clone, Copy)]
pub struct Ray {
  origin: Vector,
  direction: Vector,
}

impl Ray {
  pub fn new(origin: Vector, direction: Vector) -> Self {
    Ray { origin, direction }
  }
  pub fn origin(&self) -> Vector { self.origin }
  pub fn direction(&self) -> Vector { self.direction }
  pub fn value(&self, t: f64) -> Vector {
    self.origin + (self.direction * t)
  }
}

