use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::types::Real;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
  elements: [Real; 3],
}

impl Vec3 {
  pub fn new(x: Real, y: Real, z: Real) -> Self {
    Vec3 {
      elements: [x, y, z],
    }
  }

  pub fn x(&self) -> Real {
    self.elements[0]
  }

  pub fn y(&self) -> Real {
    self.elements[1]
  }

  pub fn z(&self) -> Real {
    self.elements[2]
  }

  pub fn dot(&self, v: Vec3) -> Real {
    let mut sum: Real = 0.0;
    for (k, _) in self.elements.iter().enumerate() {
      sum += self.elements[k] * v.elements[k];
    }
    sum
  }

  pub fn cross(&self, v: Vec3) -> Vec3 {
    Vec3 {
      elements: [
        self.y() * v.z() - self.z() * v.y(),
        self.z() * v.x() - self.x() * v.z(),
        self.x() * v.y() - self.y() * v.x(),
      ],
    }
  }

  pub fn length(&self) -> Real {
    (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
  }

  pub fn length2(&self) -> Real {
    self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
  }

  pub fn normalize(&self) -> Vec3 {
    self / self.length()
  }
}

macro_rules! impl_vec3_operator {
  ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
    impl $bound<$rhs> for $lhs {
      type Output = Vec3;

      fn $func(self, rhs: $rhs) -> Self::Output {
        Vec3{
          elements: [
            self.elements[0] $op rhs.elements[0],
            self.elements[1] $op rhs.elements[1],
            self.elements[2] $op rhs.elements[2],
          ]
        }
      }
    }
  };
}

macro_rules! impl_vec3_scalar_operator {
  ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
    impl $bound<$rhs> for $lhs {
      type Output = Vec3;

      fn $func(self, rhs: $rhs) -> Self::Output {
        Vec3{
          elements: [
            self.elements[0] $op rhs,
            self.elements[1] $op rhs,
            self.elements[2] $op rhs,
          ]
        }
      }
    }
  };
}

macro_rules! impl_scalar_vec3_operator {
  ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
    impl $bound<$rhs> for $lhs {
      type Output = Vec3;

      fn $func(self, rhs: $rhs) -> Self::Output {
        Vec3{
          elements: [
            self $op rhs.elements[0],
            self $op rhs.elements[1],
            self $op rhs.elements[2],
          ]
        }
      }
    }
  };
}

impl_vec3_operator!(Add, add, Vec3, +, Vec3);
impl_vec3_operator!(Add, add, &Vec3, +, Vec3);
impl_vec3_operator!(Add, add, Vec3, +, &Vec3);
impl_vec3_operator!(Add, add, &Vec3, +, &Vec3);

impl_vec3_operator!(Sub, sub, Vec3, -, Vec3);
impl_vec3_operator!(Sub, sub, &Vec3, -, Vec3);
impl_vec3_operator!(Sub, sub, Vec3, -, &Vec3);
impl_vec3_operator!(Sub, sub, &Vec3, -, &Vec3);

impl_vec3_operator!(Mul, mul, Vec3, *, Vec3);
impl_vec3_operator!(Mul, mul, &Vec3, *, Vec3);
impl_vec3_operator!(Mul, mul, Vec3, *, &Vec3);
impl_vec3_operator!(Mul, mul, &Vec3, *, &Vec3);

impl_vec3_operator!(Div, div, Vec3, /, Vec3);
impl_vec3_operator!(Div, div, &Vec3, /, Vec3);
impl_vec3_operator!(Div, div, Vec3, /, &Vec3);
impl_vec3_operator!(Div, div, &Vec3, /, &Vec3);

impl_vec3_scalar_operator!(Mul, mul, Vec3, *, Real);
impl_vec3_scalar_operator!(Mul, mul, &Vec3, *, Real);
impl_scalar_vec3_operator!(Mul, mul, Real, *, Vec3);
impl_scalar_vec3_operator!(Mul, mul, Real, *, &Vec3);

impl_vec3_scalar_operator!(Div, div, Vec3, /, Real);
impl_vec3_scalar_operator!(Div, div, &Vec3, /, Real);
impl_scalar_vec3_operator!(Div, div, Real, /, Vec3);
impl_scalar_vec3_operator!(Div, div, Real, /, &Vec3);

impl Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
    Vec3 {
      elements: [-self.elements[0], -self.elements[1], -self.elements[2]],
    }
  }
}

impl Neg for &Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
    Vec3 {
      elements: [-self.elements[0], -self.elements[1], -self.elements[2]],
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::vec3::*;

  #[test]
  fn vec3_val_val_add() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
  }

  #[test]
  fn vec3_ref_val_add() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
  }

  #[test]
  fn vec3_val_ref_add() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = &Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
  }

  #[test]
  fn vec3_ref_ref_add() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = &Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
  }

  #[test]
  fn vec3_val_val_sub() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
  }

  #[test]
  fn vec3_ref_val_sub() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
  }

  #[test]
  fn vec3_val_ref_sub() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = &Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
  }

  #[test]
  fn vec3_ref_ref_sub() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = &Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
  }

  #[test]
  fn vec3_mul_vec_scalar() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = 3.0;
    assert_eq!(v1 * v2, Vec3::new(3.0, 6.0, 9.0));
  }

  #[test]
  fn vec3_ref_mul_vec_scalar() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = 3.0;
    assert_eq!(v1 * v2, Vec3::new(3.0, 6.0, 9.0));
  }

  #[test]
  fn vec3_mul_scalar_vec() {
    let v1 = 3.0;
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v1 * v2, Vec3::new(3.0, 6.0, 9.0));
  }

  #[test]
  fn vec3_ref_mul_scalar_vec() {
    let v1 = 3.0;
    let v2 = &Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v1 * v2, Vec3::new(3.0, 6.0, 9.0));
  }

  #[test]
  fn vec3_val_val_mul() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
  }

  #[test]
  fn vec3_ref_val_mul() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
  }

  #[test]
  fn vec3_val_ref_mul() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = &Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
  }

  #[test]
  fn vec3_ref_ref_mul() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = &Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
  }

  #[test]
  fn vec3_div_vec_scalar() {
    let v1 = Vec3::new(1.0, 2.0, 4.0);
    let v2 = 2.0;
    assert_eq!(v1 / v2, Vec3::new(0.5, 1.0, 2.0));
  }

  #[test]
  fn vec3_ref_div_vec_scalar() {
    let v1 = &Vec3::new(1.0, 2.0, 4.0);
    let v2 = 2.0;
    assert_eq!(v1 / v2, Vec3::new(0.5, 1.0, 2.0));
  }

  #[test]
  fn vec3_div_scalar_vec() {
    let v1 = 2.0;
    let v2 = Vec3::new(1.0, 2.0, 4.0);
    assert_eq!(v1 / v2, Vec3::new(2.0, 1.0, 0.5));
  }

  #[test]
  fn vec3_ref_div_scalar_vec() {
    let v1 = 2.0;
    let v2 = &Vec3::new(1.0, 2.0, 4.0);
    assert_eq!(v1 / v2, Vec3::new(2.0, 1.0, 0.5));
  }

  #[test]
  fn vec3_val_val_div() {
    let v1 = Vec3::new(1.0, 2.0, 4.0);
    let v2 = Vec3::new(2.0, 4.0, 8.0);
    assert_eq!(v1 / v2, Vec3::new(0.5, 0.5, 0.5));
  }

  #[test]
  fn vec3_ref_val_div() {
    let v1 = &Vec3::new(1.0, 2.0, 4.0);
    let v2 = Vec3::new(2.0, 4.0, 8.0);
    assert_eq!(v1 / v2, Vec3::new(0.5, 0.5, 0.5));
  }

  #[test]
  fn vec3_val_ref_div() {
    let v1 = Vec3::new(1.0, 2.0, 4.0);
    let v2 = &Vec3::new(2.0, 4.0, 8.0);
    assert_eq!(v1 / v2, Vec3::new(0.5, 0.5, 0.5));
  }

  #[test]
  fn vec3_ref_ref_div() {
    let v1 = &Vec3::new(1.0, 2.0, 4.0);
    let v2 = &Vec3::new(2.0, 4.0, 8.0);
    assert_eq!(v1 / v2, Vec3::new(0.5, 0.5, 0.5));
  }

  #[test]
  fn vec3_neg() {
    let v = Vec3::new(1.0, 2.0, 4.0);
    assert_eq!(-v, Vec3::new(-1.0, -2.0, -4.0));
  }

  #[test]
  fn vec3_ref_neg() {
    let v = &Vec3::new(1.0, 2.0, 4.0);
    assert_eq!(-v, Vec3::new(-1.0, -2.0, -4.0));
  }

  #[test]
  fn vec3_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1.dot(v2), 32.0);
  }

  #[test]
  fn vec3_length() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.length(), (14 as Real).sqrt());
  }

  #[test]
  fn vec3_length2() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.length2(), (14 as Real));
  }

  #[test]
  fn vec3_normalize() {
    let v = Vec3::new(2.0, 0.0, 0.0);
    assert_eq!(v.normalize(), Vec3::new(1.0, 0.0, 0.0));
  }

  #[test]
  fn vec3_cross() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(v1.cross(v2), Vec3::new(0.0, 0.0, 1.0));
  }
}
