use super::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Dim3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Dim3
{
    pub const UNIT: Dim3 = Dim3 { x: 1.0, y: 1.0, z: 1.0 };
}

impl std::ops::Add<Dim3> for Dim3
{
    type Output = Dim3;

    fn add(self, rhs: Dim3) -> Self::Output
    {
        Dim3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl std::ops::Mul<f32> for Dim3
{
    type Output = Dim3;

    fn mul(self, rhs: f32) -> Self::Output
    {
        Dim3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl From<UDim3> for Dim3
{
    fn from(value: UDim3) -> Self
    {
        Dim3 { x: value.x as f32, y: value.y as f32, z: value.z as f32 }
    }
}

impl From<Vec3> for Dim3
{
    fn from(value: Vec3) -> Self
    {
        Dim3 { x: value.x as f32, y: value.y as f32, z: value.z as f32 }
    }
}
