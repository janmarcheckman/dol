use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct UDim3
{
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl UDim3
{
    pub const ZERO: UDim3 = UDim3 { x: 0, y: 0, z: 0 };

    pub fn n(&self) -> usize
    {
        self.x * self.y
    }
}

impl std::ops::Add<UDim3> for UDim3
{
    type Output = UDim3;

    fn add(self, rhs: UDim3) -> Self::Output
    {
        UDim3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl std::ops::Mul<UDim3> for UDim3
{
    type Output = UDim3;

    fn mul(self, rhs: UDim3) -> Self::Output
    {
        UDim3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl std::ops::Div<usize> for UDim3
{
    type Output = UDim3;

    fn div(self, rhs: usize) -> Self::Output
    {
        UDim3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
