use crate::vector::*;
use crate::ray::*;
use crate::color::*;
use crate::material::*;

#[derive(Clone, Copy)]
pub struct Sphere {
  center: Vector,
  radius: f64,
  color: Color,
  material: Material,
}

impl Sphere {
  pub fn new(center: Vector, radius: f64, color: Color, material: Material) -> Self {
    Sphere {
      center,
      radius,
      color,
      material,
    }
  }
  pub fn material(&self) -> &Material {
    &self.material
  }
  pub fn color(&self) -> &Color {
    &self.color
  }
  pub fn set_position(&mut self, p: &Vector) {
    self.center = *p;
  }
  pub fn normal(&self, p: &Vector) -> Vector {
    let mut n = *p - self.center;
    n.normalize();
    n
  }
  pub fn intersect(&self, ray: &Ray) -> Option<f64> {
    let a = ray.direction() * ray.direction();
    let dr = ray.origin() - self.center;
    let b = 2.0 * (ray.direction() * dr);
    let c = (dr * dr) - self.radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;
    let ret = -(b + discriminant.sqrt()) / (a * 2.0);
    if discriminant > 0.0 && ret > 0.0 {
      Some(ret)
    } else {
      None
    }
  }
}
