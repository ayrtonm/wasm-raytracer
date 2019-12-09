use std::ops;
use crate::rng::*;

#[derive(Clone, Copy)]
pub struct Color {
  r: u8, g: u8, b: u8, a: u8,
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color { r, g, b, a }
  }
  pub fn rand_color() -> Self {
    Color::new(randu8(), randu8(), randu8(), 255)
  }
  pub fn red(&self) -> u8 { self.r }
  pub fn green(&self) -> u8 { self.g }
  pub fn blue(&self) -> u8 { self.b }
  pub fn alpha(&self) -> u8 { self.a }
}

impl ops::Add<Color> for Color {
  type Output = Color;
  fn add(self, other: Color) -> Color {
    Color::new(self.red() + other.red(), self.green() + other.green(), self.blue() + other.blue(), 255)
  }
}

impl ops::Mul<f64> for Color {
  type Output = Color;
  fn mul(self, other: f64) -> Color {
    Color::new((self.red() as f64 * other) as u8, (self.green() as f64 * other) as u8, (self.blue() as f64 * other) as u8, 255)
  }
}

impl ops::Mul<Color> for f64 {
  type Output = Color;
  fn mul(self, other: Color) -> Color {
    Color::new((other.red() as f64 * self) as u8, (other.green() as f64 * self) as u8, (other.blue() as f64 * self) as u8, 255)
  }
}
