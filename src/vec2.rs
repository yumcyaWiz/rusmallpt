use std::ops::{Add, Div, Mul, Sub};

use crate::types::Real;

#[derive(Debug, PartialEq)]
struct Vec2 {
  elements: [Real; 2],
}

impl Vec2 {
  fn new(x: Real, y: Real) -> Self {
    Vec2 { elements: [x, y] }
  }

  fn x(&self) -> Real {
    self.elements[0]
  }

  fn y(&self) -> Real {
    self.elements[1]
  }
}

fn dot(v1: &Vec2, v2: &Vec2) -> Real {
  let mut sum: Real = 0.0;
  for (k, _) in v1.elements.iter().enumerate() {
    sum += v1.elements[k] * v2.elements[k];
  }
  sum
}

fn length(v: &Vec2) -> Real {
  dot(v, v).sqrt()
}

fn length2(v: &Vec2) -> Real {
  dot(v, v)
}

// Vec2 + Vec2
impl Add for Vec2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v + rhs.elements[k];
    }
    ret
  }
}

// Vec2 - Vec2
impl Sub for Vec2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v - rhs.elements[k];
    }
    ret
  }
}

// Vec2 * Real
impl Mul<Real> for Vec2 {
  type Output = Self;

  fn mul(self, rhs: Real) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v * rhs;
    }
    ret
  }
}

// Real * Vec2
impl Mul<Vec2> for Real {
  type Output = Vec2;

  fn mul(self, rhs: Vec2) -> Self::Output {
    let mut ret = Vec2::new(0.0, 0.0);
    for (k, v) in rhs.elements.iter().enumerate() {
      ret.elements[k] = self * v;
    }
    ret
  }
}

// Vec2 * Vec2
impl Mul for Vec2 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v * rhs.elements[k];
    }
    ret
  }
}

// Vec2 / Real
impl Div<Real> for Vec2 {
  type Output = Self;

  fn div(self, rhs: Real) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v / rhs;
    }
    ret
  }
}

// Real / Vec2
impl Div<Vec2> for Real {
  type Output = Vec2;

  fn div(self, rhs: Vec2) -> Self::Output {
    let mut ret = Vec2::new(0.0, 0.0);
    for (k, v) in rhs.elements.iter().enumerate() {
      ret.elements[k] = self / v;
    }
    ret
  }
}

// Vec2 / Vec2
impl Div for Vec2 {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v / rhs.elements[k];
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use crate::vec2::*;

  #[test]
  fn vec2_add() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
  }

  #[test]
  fn vec2_sub() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 - v2, Vec2::new(-2.0, -2.0));
  }

  #[test]
  fn vec2_mul_vec_scalar() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = 3.0;
    assert_eq!(v1 * v2, Vec2::new(3.0, 6.0));
  }

  #[test]
  fn vec2_mul_scalar_vec() {
    let v1 = 3.0;
    let v2 = Vec2::new(1.0, 2.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 6.0));
  }

  #[test]
  fn vec2_mul_vec_vec() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 * v2, Vec2::new(3.0, 8.0));
  }

  #[test]
  fn vec2_div_vec_scalar() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = 2.0;
    assert_eq!(v1 / v2, Vec2::new(0.5, 1.0));
  }

  #[test]
  fn vec2_div_scalar_vec() {
    let v1 = 2.0;
    let v2 = Vec2::new(1.0, 2.0);
    assert_eq!(v1 / v2, Vec2::new(2.0, 1.0));
  }

  #[test]
  fn vec2_div_vec_vec() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(2.0, 4.0);
    assert_eq!(v1 / v2, Vec2::new(0.5, 0.5));
  }

  #[test]
  fn vec2_dot() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(dot(&v1, &v2), 11.0);
  }

  #[test]
  fn vec2_length() {
    let v = Vec2::new(1.0, 2.0);
    assert_eq!(length(&v), (5 as Real).sqrt());
  }

  #[test]
  fn vec2_length2() {
    let v = Vec2::new(1.0, 2.0);
    assert_eq!(length2(&v), (5 as Real));
  }
}
