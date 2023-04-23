use std::ops;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct UVec4
{
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
}

impl UVec4
{
    pub fn all(u: usize) -> UVec4
    {
        UVec4 { x: u, y: u, z: u, w: u }
    }
}

impl ops::Add<UVec4> for UVec4
{
    type Output = UVec4;

    fn add(self, rhs: UVec4) -> UVec4
    {
        UVec4
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<UVec4> for UVec4
{
    type Output = UVec4;

    fn sub(self, rhs: UVec4) -> UVec4
    {
        UVec4
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.y - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::BitXor<UVec4> for UVec4
{
    type Output = UVec4;

    fn bitxor(self, rhs: UVec4) -> Self::Output
    {
        UVec4
        {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
            w: self.w ^ rhs.w,
        }
    }
}
