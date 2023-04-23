use std::ops;

use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Neg for Vec3
{
    type Output = Vec3;

    fn neg(self) -> Vec3
    {
        Vec3
        {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<Vec3> for Vec3
{
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<f32> for Vec3
{
    type Output = Vec3;

    fn add(self, rhs: f32) -> Vec3
    {
        Vec3
        {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::Sub<Vec3> for Vec3
{
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<f32> for Vec3
{
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Vec3
    {
        Vec3
        {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3
    {
        Vec3
        {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32
{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3
    {
        Vec3
        {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::MulAssign<f32> for Vec3
{
    fn mul_assign(&mut self, rhs: f32)
    {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::AddAssign<Vec3> for Vec3
{
    fn add_assign(&mut self, rhs: Vec3)
    {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::AddAssign<f32> for Vec3
{
    fn add_assign(&mut self, rhs: f32)
    {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl ops::SubAssign<Vec3> for Vec3
{
    fn sub_assign(&mut self, rhs: Vec3)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Vec3
{
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const E0: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const E1: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const E2: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
    pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3
    {
        Vec3 { x, y, z }
    }

    pub fn all(f: f32) -> Vec3
    {
        Vec3 { x: f, y: f, z: f }
    }

    pub fn lerp(a: Vec3, b: Vec3, w: f32) -> Vec3
    {
        let inv = 1.0 - w;
        Vec3
        {
            x: b.x * w + a.x * inv,
            y: b.y * w + a.y * inv,
            z: b.z * w + a.z * inv,
        }
    }

    pub fn dot(self, other: Vec3) -> f32
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn mul_comp(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn div_comp(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    pub fn floor_comp(self) -> Vec3
    {
        Vec3
        {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn fract_comp(self) -> Vec3
    {
        Vec3
        {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract(),
        }
    }

    pub fn sqr_mag(&self) -> f32
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn mag(&self) -> f32
    {
        f32::sqrt(self.sqr_mag())
    }

    pub fn dist(&self, other: Vec3) -> f32
    {
        (*self - other).mag()
    }

    pub fn normalize(&mut self)
    {
        let l = self.mag();
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }

    pub fn nor(&self) -> Vec3
    {
        let l = self.mag();
        Vec3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn angle(self, other: Vec3) -> f32
    {
        let mut a = Vec3::dot(self.nor(), other.nor());
        if a > 1.0 { a = 1.0 }
        f32::acos(a)
    }

    pub fn vec2(&self) -> Vec2
    {
        Vec2
        {
            x: self.x,
            y: self.y,
        }
    }

    pub fn abs(&self) -> Vec3
    {
        Vec3
        {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn min(&self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: f32::min(self.x, other.x),
            y: f32::min(self.y, other.y),
            z: f32::min(self.z, other.z),
        }
    }

    pub fn max(&self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: f32::max(self.x, other.x),
            y: f32::max(self.y, other.y),
            z: f32::max(self.z, other.z),
        }
    }

    pub fn maxcomp(&self) -> f32
    {
        f32::max(f32::max(self.x, self.y), self.z)
    }

    pub fn mincomp(&self) -> f32
    {
        f32::min(f32::min(self.x, self.y), self.z)
    }

    pub fn from_ivec3(v: IVec3) -> Vec3
    {
        Vec3
        {
            x: v.x as f32,
            y: v.y as f32,
            z: v.z as f32,
        }
    }
}

impl From<Vec2> for Vec3
{
    fn from(item: Vec2) -> Self
    {
        Vec3
        {
            x: item.x,
            y: item.y,
            z: 0.0,
        }
    }
}

impl From<Dim3> for Vec3
{
    fn from(item: Dim3) -> Self
    {
        Vec3
        {
            x: item.x,
            y: item.y,
            z: item.z,
        }
    }
}