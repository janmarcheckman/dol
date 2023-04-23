use super::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Dim2
{
    pub x: f32,
    pub y: f32,
}

impl Dim2
{
    pub const UNIT: Dim2 = Dim2 { x: 1.0, y: 1.0 };

    pub fn area(self) -> f32
    {
        self.x * self.y
    }

    pub fn contains(self, p: Vec2) -> bool
    {
        p.x >= 0.0 && p.y >= 0.0 && p.x < self.x && p.y < self.y
    }
}

impl std::ops::Add<Dim2> for Dim2
{
    type Output = Dim2;

    fn add(self, rhs: Dim2) -> Self::Output
    {
        Dim2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Mul<f32> for Dim2
{
    type Output = Dim2;

    fn mul(self, rhs: f32) -> Self::Output
    {
        Dim2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl From<UDim2> for Dim2
{
    fn from(value: UDim2) -> Self
    {
        Dim2 { x: value.x as f32, y: value.y as f32 }
    }
}
