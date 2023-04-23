use std::ops;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Vec4
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl ops::Mul<f32> for Vec4
{
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Vec4
    {
        Vec4
        {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
