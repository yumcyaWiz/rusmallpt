use std::ops::{Add, Div, Mul, Sub};

use crate::types::Real;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
  elements: [Real; 3],
}

impl Vec3 {
  fn new(x: Real, y: Real, z: Real) -> Self {
    Vec3 {
      elements: [x, y, z],
    }
  }

  fn x(&self) -> Real {
    self.elements[0]
  }

  fn y(&self) -> Real {
    self.elements[1]
  }

  fn z(&self) -> Real {
    self.elements[2]
  }
}

fn dot(v1: &Vec3, v2: &Vec3) -> Real {
  let mut sum: Real = 0.0;
  for (k, _) in v1.elements.iter().enumerate() {
    sum += v1.elements[k] * v2.elements[k];
  }
  sum
}

fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
  Vec3 {
    elements: [
      v1.y() * v2.z() - v1.z() * v2.y(),
      v1.z() * v2.x() - v1.x() * v2.z(),
      v1.x() * v2.y() - v1.y() * v2.x(),
    ],
  }
}

fn length(v: &Vec3) -> Real {
  dot(v, v).sqrt()
}

fn length2(v: &Vec3) -> Real {
  dot(v, v)
}

// Vec3 + Vec3
impl Add for Vec3 {
  type Output = Self;

  fn add(self, rhs: Vec3) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v + rhs.elements[k];
    }
    ret
  }
}

// Vec3(ref) + Vec3(ref)
impl Add for &Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Self) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v + rhs.elements[k];
    }
    ret
  }
}

// Vec3 - Vec3
impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v - rhs.elements[k];
    }
    ret
  }
}

// Vec3(ref) - Vec3(ref)
impl Sub for &Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Self) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v - rhs.elements[k];
    }
    ret
  }
}

// Vec3 * Real
impl Mul<Real> for Vec3 {
  type Output = Self;

  fn mul(self, rhs: Real) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v * rhs;
    }
    ret
  }
}

// Vec3(ref) * Real
impl Mul<Real> for &Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Real) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v * rhs;
    }
    ret
  }
}

// Real * Vec3
impl Mul<Vec3> for Real {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in rhs.elements.iter().enumerate() {
      ret.elements[k] = self * v;
    }
    ret
  }
}

// Real * Vec3(ref)
impl Mul<&Vec3> for Real {
  type Output = Vec3;

  fn mul(self, rhs: &Vec3) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in rhs.elements.iter().enumerate() {
      ret.elements[k] = self * v;
    }
    ret
  }
}

// Vec3 * Vec3
impl Mul for Vec3 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v * rhs.elements[k];
    }
    ret
  }
}

// Vec3(ref) * Vec3(ref)
impl Mul for &Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Self) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v * rhs.elements[k];
    }
    ret
  }
}

// Vec3 / Real
impl Div<Real> for Vec3 {
  type Output = Self;

  fn div(self, rhs: Real) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v / rhs;
    }
    ret
  }
}

// Vec3(ref) / Real
impl Div<Real> for &Vec3 {
  type Output = Vec3;

  fn div(self, rhs: Real) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v / rhs;
    }
    ret
  }
}

// Real / Vec3
impl Div<Vec3> for Real {
  type Output = Vec3;

  fn div(self, rhs: Vec3) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in rhs.elements.iter().enumerate() {
      ret.elements[k] = self / v;
    }
    ret
  }
}

// Real / Vec3(ref)
impl Div<&Vec3> for Real {
  type Output = Vec3;

  fn div(self, rhs: &Vec3) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in rhs.elements.iter().enumerate() {
      ret.elements[k] = self / v;
    }
    ret
  }
}

// Vec3 / Vec3
impl Div for Vec3 {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    let mut ret = Self::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v / rhs.elements[k];
    }
    ret
  }
}

// Vec3(ref) / Vec3(ref)
impl Div for &Vec3 {
  type Output = Vec3;

  fn div(self, rhs: Self) -> Self::Output {
    let mut ret = Vec3::new(0.0, 0.0, 0.0);
    for (k, v) in self.elements.iter().enumerate() {
      ret.elements[k] = v / rhs.elements[k];
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use crate::vec3::*;

  #[test]
  fn vec3_add() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
  }

  #[test]
  fn vec3_ref_add() {
    let v1 = &Vec3::new(1.0, 2.0, 3.0);
    let v2 = &Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
  }

  #[test]
  fn vec3_sub() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
  }

  #[test]
  fn vec3_ref_sub() {
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
  fn vec3_mul_vec_vec() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
  }

  #[test]
  fn vec3_ref_mul_vec_vec() {
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
  fn vec3_div_vec_vec() {
    let v1 = Vec3::new(1.0, 2.0, 4.0);
    let v2 = Vec3::new(2.0, 4.0, 8.0);
    assert_eq!(v1 / v2, Vec3::new(0.5, 0.5, 0.5));
  }

  #[test]
  fn vec3_ref_div_vec_vec() {
    let v1 = &Vec3::new(1.0, 2.0, 4.0);
    let v2 = &Vec3::new(2.0, 4.0, 8.0);
    assert_eq!(v1 / v2, Vec3::new(0.5, 0.5, 0.5));
  }

  #[test]
  fn vec3_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(dot(&v1, &v2), 32.0);
  }

  #[test]
  fn vec3_length() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(length(&v), (14 as Real).sqrt());
  }

  #[test]
  fn vec3_length2() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(length2(&v), (14 as Real));
  }

  #[test]
  fn vec3_cross() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(cross(&v1, &v2), Vec3::new(0.0, 0.0, 1.0));
  }
}
