use std::ops;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct UVec2
{
    pub x: usize,
    pub y: usize,
}

impl UVec2
{
    pub fn all(u: usize) -> UVec2
    {
        UVec2 { x: u, y: u }
    }
}

impl ops::Add<UVec2> for UVec2
{
    type Output = UVec2;

    fn add(self, rhs: UVec2) -> UVec2
    {
        UVec2
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<UVec2> for UVec2
{
    type Output = UVec2;

    fn sub(self, rhs: UVec2) -> UVec2
    {
        UVec2
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::BitXor<UVec2> for UVec2
{
    type Output = UVec2;

    fn bitxor(self, rhs: UVec2) -> Self::Output
    {
        UVec2
        {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }
}
