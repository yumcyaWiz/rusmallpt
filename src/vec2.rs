use std::ops::{Add, Div, Mul, Sub};

use crate::types::Real;

#[derive(Debug, PartialEq)]
pub struct Vec2 {
  elements: [Real; 2],
}

impl Vec2 {
  pub fn new(x: Real, y: Real) -> Self {
    Vec2 { elements: [x, y] }
  }

  pub fn x(&self) -> Real {
    self.elements[0]
  }

  pub fn y(&self) -> Real {
    self.elements[1]
  }

  pub fn dot(&self, v: &Vec2) -> Real {
    let mut sum: Real = 0.0;
    for (k, _) in self.elements.iter().enumerate() {
      sum += self.elements[k] * v.elements[k];
    }
    sum
  }

  pub fn length(&self) -> Real {
    self.dot(self).sqrt()
  }

  pub fn length2(&self) -> Real {
    self.dot(self)
  }

  pub fn normalize(&self) -> Vec2 {
    self / self.length()
  }
}

macro_rules! impl_vec2_operator {
  ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
    impl $bound<$rhs> for $lhs {
      type Output = Vec2;

      fn $func(self, rhs: $rhs) -> Self::Output {
        Vec2{
          elements: [
            self.elements[0] $op rhs.elements[0],
            self.elements[1] $op rhs.elements[1],
          ]
        }
      }
    }
  };
}

macro_rules! impl_vec2_scalar_operator {
  ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
    impl $bound<$rhs> for $lhs {
      type Output = Vec2;

      fn $func(self, rhs: $rhs) -> Self::Output {
        Vec2{
          elements: [
            self.elements[0] $op rhs,
            self.elements[1] $op rhs,
          ]
        }
      }
    }
  };
}

macro_rules! impl_scalar_vec2_operator {
  ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
    impl $bound<$rhs> for $lhs {
      type Output = Vec2;

      fn $func(self, rhs: $rhs) -> Self::Output {
        Vec2{
          elements: [
            self $op rhs.elements[0],
            self $op rhs.elements[1],
          ]
        }
      }
    }
  };
}

impl_vec2_operator!(Add, add, Vec2, +, Vec2);
impl_vec2_operator!(Add, add, &Vec2, +, Vec2);
impl_vec2_operator!(Add, add, Vec2, +, &Vec2);
impl_vec2_operator!(Add, add, &Vec2, +, &Vec2);

impl_vec2_operator!(Sub, sub, Vec2, -, Vec2);
impl_vec2_operator!(Sub, sub, &Vec2, -, Vec2);
impl_vec2_operator!(Sub, sub, Vec2, -, &Vec2);
impl_vec2_operator!(Sub, sub, &Vec2, -, &Vec2);

impl_vec2_operator!(Mul, mul, Vec2, *, Vec2);
impl_vec2_operator!(Mul, mul, &Vec2, *, Vec2);
impl_vec2_operator!(Mul, mul, Vec2, *, &Vec2);
impl_vec2_operator!(Mul, mul, &Vec2, *, &Vec2);

impl_vec2_scalar_operator!(Mul, mul, Vec2, *, Real);
impl_vec2_scalar_operator!(Mul, mul, &Vec2, *, Real);
impl_scalar_vec2_operator!(Mul, mul, Real, *, Vec2);
impl_scalar_vec2_operator!(Mul, mul, Real, *, &Vec2);

impl_vec2_operator!(Div, div, Vec2, /, Vec2);
impl_vec2_operator!(Div, div, &Vec2, /, Vec2);
impl_vec2_operator!(Div, div, Vec2, /, &Vec2);
impl_vec2_operator!(Div, div, &Vec2, /, &Vec2);

impl_vec2_scalar_operator!(Div, div, Vec2, /, Real);
impl_vec2_scalar_operator!(Div, div, &Vec2, /, Real);
impl_scalar_vec2_operator!(Div, div, Real, /, Vec2);
impl_scalar_vec2_operator!(Div, div, Real, /, &Vec2);

#[cfg(test)]
mod tests {
  use crate::vec2::*;

  #[test]
  fn vec2_val_val_add() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
  }

  #[test]
  fn vec2_val_ref_add() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(3.0, 4.0);
    assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
  }

  #[test]
  fn vec2_ref_val_add() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
  }

  #[test]
  fn vec2_ref_ref_add() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(3.0, 4.0);
    assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
  }

  #[test]
  fn vec2_val_val_sub() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 - v2, Vec2::new(-2.0, -2.0));
  }

  #[test]
  fn vec2_val_ref_sub() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(3.0, 4.0);
    assert_eq!(v1 - v2, Vec2::new(-2.0, -2.0));
  }

  #[test]
  fn vec2_ref_val_sub() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 - v2, Vec2::new(-2.0, -2.0));
  }

  #[test]
  fn vec2_ref_ref_sub() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(3.0, 4.0);
    assert_eq!(v1 - v2, Vec2::new(-2.0, -2.0));
  }

  #[test]
  fn vec2_valv_vals_mul() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = 3.0;
    assert_eq!(v1 * v2, Vec2::new(3.0, 6.0));
  }

  #[test]
  fn vec2_refv_vals_mul() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = 3.0;
    assert_eq!(v1 * v2, Vec2::new(3.0, 6.0));
  }

  #[test]
  fn vec2_vals_valv_mul() {
    let v1 = 3.0;
    let v2 = Vec2::new(1.0, 2.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 6.0));
  }

  #[test]
  fn vec2_vals_refv_mul() {
    let v1 = 3.0;
    let v2 = &Vec2::new(1.0, 2.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 6.0));
  }

  #[test]
  fn vec2_val_val_mul() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 8.0));
  }

  #[test]
  fn vec2_ref_val_mul() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 8.0));
  }

  #[test]
  fn vec2_val_ref_mul() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(3.0, 4.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 8.0));
  }

  #[test]
  fn vec2_ref_ref_mul() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(3.0, 4.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 8.0));
  }

  #[test]
  fn vec2_valv_vals_div() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = 2.0;
    assert_eq!(v1 / v2, Vec2::new(0.5, 1.0));
  }

  #[test]
  fn vec2_refv_vals_div() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = 2.0;
    assert_eq!(v1 / v2, Vec2::new(0.5, 1.0));
  }

  #[test]
  fn vec2_vals_valv_div() {
    let v1 = 2.0;
    let v2 = Vec2::new(1.0, 2.0);
    assert_eq!(v1 / v2, Vec2::new(2.0, 1.0));
  }

  #[test]
  fn vec2_vals_refv_div() {
    let v1 = 2.0;
    let v2 = &Vec2::new(1.0, 2.0);
    assert_eq!(v1 / v2, Vec2::new(2.0, 1.0));
  }

  #[test]
  fn vec2_val_val_div() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(2.0, 4.0);
    assert_eq!(v1 / v2, Vec2::new(0.5, 0.5));
  }

  #[test]
  fn vec2_ref_val_div() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(2.0, 4.0);
    assert_eq!(v1 / v2, Vec2::new(0.5, 0.5));
  }

  #[test]
  fn vec2_val_ref_div() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(2.0, 4.0);
    assert_eq!(v1 / v2, Vec2::new(0.5, 0.5));
  }

  #[test]
  fn vec2_ref_ref_div() {
    let v1 = &Vec2::new(1.0, 2.0);
    let v2 = &Vec2::new(2.0, 4.0);
    assert_eq!(v1 / v2, Vec2::new(0.5, 0.5));
  }

  #[test]
  fn vec2_dot() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1.dot(&v2), 11.0);
  }

  #[test]
  fn vec2_length() {
    let v = Vec2::new(1.0, 2.0);
    assert_eq!(v.length(), (5 as Real).sqrt());
  }

  #[test]
  fn vec2_length2() {
    let v = Vec2::new(1.0, 2.0);
    assert_eq!(v.length2(), (5 as Real));
  }

  #[test]
  fn vec2_normalize() {
    let v = Vec2::new(2.0, 0.0);
    assert_eq!(v.normalize(), Vec2::new(1.0, 0.0));
  }
}
