use std::ops;

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct UVec3
{
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl UVec3
{
    pub fn new(x: usize, y: usize, z: usize) -> UVec3
    {
        UVec3 { x, y, z }
    }

    pub fn all(u: usize) -> UVec3
    {
        UVec3 { x: u, y: u, z: u }
    }

    pub fn from_vec3(v: Vec3) -> UVec3
    {
        UVec3
        {
            x: v.x as usize,
            y: v.y as usize,
            z: v.z as usize,
        }
    }
}

impl ops::Add<UVec3> for UVec3
{
    type Output = UVec3;

    fn add(self, rhs: UVec3) -> UVec3
    {
        UVec3
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<UVec3> for UVec3
{
    type Output = UVec3;

    fn sub(self, rhs: UVec3) -> UVec3
    {
        UVec3
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.y - rhs.z,
        }
    }
}

impl ops::BitXor<UVec3> for UVec3
{
    type Output = UVec3;

    fn bitxor(self, rhs: UVec3) -> Self::Output
    {
        UVec3
        {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
        }
    }
}
