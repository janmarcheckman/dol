use std::ops;

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct IVec2
{
    pub x: isize,
    pub y: isize,
}

impl IVec2
{
    pub fn new(x: isize, y: isize) -> IVec2
    {
        IVec2 { x, y }
    }

    pub fn from_vec2(v: Vec2) -> IVec2
    {
        IVec2
        {
            x: f32::floor(v.x) as isize,
            y: f32::floor(v.y) as isize,
        }
    }
}

impl ops::Add<IVec2> for IVec2
{
    type Output = IVec2;

    fn add(self, rhs: IVec2) -> IVec2
    {
        IVec2
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<IVec2> for IVec2
{
    type Output = IVec2;

    fn sub(self, rhs: IVec2) -> IVec2
    {
        IVec2
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign<IVec2> for IVec2
{
    fn add_assign(&mut self, rhs: IVec2)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
