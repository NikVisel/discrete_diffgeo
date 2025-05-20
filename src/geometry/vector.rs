/// 3D vector type and utilities
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Vector3 {
    /// Construct new Vector3
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    /// Zero vector
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
    /// Dot product
    pub fn dot(&self, other: &Self) -> f64 { self.x*other.x + self.y*other.y + self.z*other.z }
    /// Cross product
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
    /// Norm (length)
    pub fn norm(&self) -> f64 { self.dot(self).sqrt() }
    /// Normalize to unit length
    pub fn normalize(&self) -> Self {
        let n = self.norm(); if n == 0.0 { *self } else { *self / n }
    }
    /// Create from array [x, y, z]
    pub fn from_array(a: [f64;3]) -> Self { Self::new(a[0], a[1], a[2]) }
    /// Convert to array [x, y, z]
    pub fn to_array(&self) -> [f64;3] { [self.x, self.y, self.z] }
}

use std::ops::{Add, Sub, Mul, Div, Neg};

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z) }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z) }
}
impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y, -self.z) }
}
impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output { Self::new(self.x*s, self.y*s, self.z*s) }
}
impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, s: f64) -> Self::Output { Self::new(self.x/s, self.y/s, self.z/s) }
}

// Allow scalar * Vector3
impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self::Output { v * self }
}

impl From<[f64;3]> for Vector3 {
    fn from(a: [f64;3]) -> Self { Vector3::from_array(a) }
}
impl From<Vector3> for [f64;3] {
    fn from(v: Vector3) -> Self { v.to_array() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot_cross_norm() {
        let u = Vector3::new(1.0,0.0,0.0);
        let v = Vector3::new(0.0,1.0,0.0);
        assert_eq!(u.dot(&v), 0.0);
        assert_eq!(u.cross(&v), Vector3::new(0.0,0.0,1.0));
        assert_eq!(u.norm(), 1.0);
    }
    #[test]
    fn test_from_into_array() {
        let a = [1.1,2.2,3.3];
        let v: Vector3 = a.into();
        assert_eq!(v.to_array(), a);
    }
}