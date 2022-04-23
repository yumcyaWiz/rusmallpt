use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::types::Real;

// NOTE: y-up, (+x, +y, -z)

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

    pub fn max(&self) -> Real {
        self.x().max(self.y()).max(self.z())
    }

    pub fn max3(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.x().max(v.x()),
            self.y().max(v.y()),
            self.z().max(v.z()),
        )
    }

    pub fn world_to_local(&self, lx: Vec3, ly: Vec3, lz: Vec3) -> Vec3 {
        Vec3::new(self.dot(lx), self.dot(ly), self.dot(lz))
    }

    pub fn local_to_world(&self, lx: Vec3, ly: Vec3, lz: Vec3) -> Vec3 {
        self.x() * lx + self.y() * ly + self.z() * lz
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

macro_rules! impl_vec3_assign_operator  {
    ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
        impl $bound<$rhs> for $lhs {
            fn $func(&mut self, rhs: $rhs) {
                *self = Vec3::new(
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z(),
                )
            }
        }
    };
}

macro_rules! impl_vec3_scalar_assign_operator  {
    ($bound:ident, $func:ident, $lhs:ty, $op:tt, $rhs:ty) => {
        impl $bound<$rhs> for $lhs {
            fn $func(&mut self, rhs: $rhs) {
                *self = Vec3::new(
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs,
                )
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

impl_vec3_assign_operator!(AddAssign, add_assign, Vec3, +, Vec3);
impl_vec3_assign_operator!(SubAssign, sub_assign, Vec3, -, Vec3);
impl_vec3_assign_operator!(MulAssign, mul_assign, Vec3, *, Vec3);
impl_vec3_assign_operator!(DivAssign, div_assign, Vec3, /, Vec3);

impl_vec3_scalar_assign_operator!(AddAssign, add_assign, Vec3, +, Real);
impl_vec3_scalar_assign_operator!(SubAssign, sub_assign, Vec3, -, Real);
impl_vec3_scalar_assign_operator!(MulAssign, mul_assign, Vec3, *, Real);
impl_vec3_scalar_assign_operator!(DivAssign, div_assign, Vec3, /, Real);

pub fn build_orthonormal_basis(v: Vec3) -> (Vec3, Vec3, Vec3) {
    #[allow(unused_assignments)]
    let mut lx = Vec3::new(1.0, 0.0, 0.0);
    let ly = v;
    #[allow(unused_assignments)]
    let mut lz = Vec3::new(0.0, 0.0, 1.0);

    if ly.y().abs() < 0.9 {
        lx = -ly.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
    } else {
        lx = -ly.cross(Vec3::new(0.0, 0.0, -1.0)).normalize();
    }
    lz = lx.cross(ly).normalize();

    (lx, ly, lz)
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
        assert_eq!(v.length(), 14_f32.sqrt());
    }

    #[test]
    fn vec3_length2() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length2(), 14_f32);
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

    #[test]
    fn vec3_world_to_local() {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let lx = Vec3::new(1.0, 0.0, 0.0);
        let ly = Vec3::new(0.0, 1.0, 0.0);
        let lz = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(v.world_to_local(lx, ly, lz), Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_local_to_world() {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let lx = Vec3::new(1.0, 0.0, 0.0);
        let ly = Vec3::new(0.0, 1.0, 0.0);
        let lz = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(v.local_to_world(lx, ly, lz), Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_orthonormal_basis() {
        let v = Vec3::new(0.0, 1.0, 0.0);
        let (lx, ly, lz) = build_orthonormal_basis(v);
        assert_eq!(lx, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ly, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(lz, Vec3::new(0.0, 0.0, 1.0));
    }
}
