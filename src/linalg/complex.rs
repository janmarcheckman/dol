use std::ops;

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Complex
{
    pub r: f32,
    pub i: f32,
}

impl Complex
{
    pub const ONE: Complex = Complex { r: 1.0, i: 0.0 };
    pub const I: Complex = Complex { r: 0.0, i: 1.0 };

    pub fn phase(theta: f32) -> Complex
    {
        Complex
        {
            r: f32::cos(theta),
            i: f32::sin(theta),
        }
    }

    pub fn i_inv(self) -> Complex
    {
        Complex { r: self.r, i: -self.i }
    }
}

impl ops::MulAssign<Complex> for Complex
{
    fn mul_assign(&mut self, rhs: Complex)
    {
        let r = self.r * rhs.r - self.i * rhs.i;
        let i = self.r * rhs.i + self.i * rhs.r;
        self.r = r;
        self.i = i;
    }
}

impl ops::Mul<Complex> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex
    {
        Complex
        {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl ops::Mul<f32> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: f32) -> Complex
    {
        Complex
        {
            r: self.r * rhs,
            i: self.i * rhs,
        }
    }
}

impl ops::AddAssign<Complex> for Complex
{
    fn add_assign(&mut self, rhs: Complex)
    {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl ops::Mul<Vec2> for Complex
{
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2
    {
        Vec2
        {
            x: rhs.x * self.r - rhs.y * self.i,
            y: rhs.y * self.r + rhs.x * self.i,
        }
    }
}

impl From<Vec2> for Complex
{
    fn from(value: Vec2) -> Self {
        Complex { r: value.x, i: value.y }
    }
}