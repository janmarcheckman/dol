use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct UDim2
{
    pub x: usize,
    pub y: usize,
}

impl UDim2
{
    pub const ZERO: UDim2 = UDim2 { x: 0, y: 0 };

    pub fn n(&self) -> usize
    {
        self.x * self.y
    }

    pub fn aspect_ratio(&self) -> f32
    {
        self.x as f32 / self.y as f32
    }
}

impl std::ops::Add<UDim2> for UDim2
{
    type Output = UDim2;

    fn add(self, rhs: UDim2) -> Self::Output
    {
        UDim2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Mul<UDim2> for UDim2
{
    type Output = UDim2;

    fn mul(self, rhs: UDim2) -> Self::Output
    {
        UDim2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl std::ops::Div<usize> for UDim2
{
    type Output = UDim2;

    fn div(self, rhs: usize) -> Self::Output
    {
        UDim2 { x: self.x / rhs, y: self.y / rhs }
    }
}

impl From<Dim2> for UDim2
{
    fn from(value: Dim2) -> Self
    {
        UDim2 { x: value.x as usize, y: value.y as usize }
    }
}
