use std::ops;

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct IVec3
{
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl IVec3
{
    pub const ZERO: IVec3 = IVec3 { x: 0, y: 0, z: 0 };
    
    pub fn new(x: isize, y: isize, z: isize) -> IVec3
    {
        IVec3 { x, y, z }
    }
    
    pub fn from_vec3(v: Vec3) -> IVec3
    {
        IVec3
        {
            x: f32::floor(v.x) as isize,
            y: f32::floor(v.y) as isize,
            z: f32::floor(v.z) as isize,
        }
    }
}

impl ops::Add<IVec3> for IVec3
{
    type Output = IVec3;

    fn add(self, _rhs: IVec3) -> IVec3
    {
        IVec3
        {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<IVec3> for IVec3
{
    type Output = IVec3;

    fn sub(self, _rhs: IVec3) -> IVec3
    {
        IVec3
        {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::AddAssign<IVec3> for IVec3
{
    fn add_assign(&mut self, rhs: IVec3)
    {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
