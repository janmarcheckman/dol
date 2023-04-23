use std::ops;

use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32,
}

impl ops::Neg for Vec2
{
    type Output = Vec2;

    fn neg(self) -> Vec2
    {
        Vec2
        {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add<Vec2> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<f32> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: f32) -> Vec2
    {
        Vec2
        {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::AddAssign<Vec2> for Vec2
{
    fn add_assign(&mut self, rhs: Vec2)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign<Vec2> for Vec2
{
    fn sub_assign(&mut self, rhs: Vec2)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<Vec2> for Vec2
{
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vec2
{
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2
    {
        Vec2
        {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vec2
{
    fn mul_assign(&mut self, rhs: f32)
    {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2
{
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vec2
{
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Vec2
    {
        Vec2 { x, y }
    }

    pub fn dot(a: Vec2, b: Vec2) -> f32
    {
        a.x * b.x + a.y * b.y
    }

    pub fn hadamard(a: Vec2, b: Vec2) -> Vec2
    {
        Vec2
        {
            x: a.x * b.x,
            y: a.y * b.y,
        }
    }

    pub fn mag(&self) -> f32
    {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn sqr_dist(self, other: Vec2) -> f32
    {
        (self - other).sqr_mag()
    }

    pub fn dist(self, other: Vec2) -> f32
    {
        (self - other).mag()
    }

    pub fn nor_mag(self) -> (Vec2, f32)
    {
        let mag = self.mag();
        let nor = self / mag;
        (nor, mag)
    }

    pub fn sqr_mag(&self) -> f32
    {
        self.x * self.x + self.y * self.y
    }
    
    pub fn nor(&self) -> Vec2
    {
        let l = self.mag();
        Vec2
        {
            x: self.x / l,
            y: self.y / l,
        }
    }

    pub fn all(f: f32) -> Vec2
    {
        Vec2 { x: f, y: f }
    }

    pub fn abs(self) -> Vec2
    {
        Vec2
        {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn max(self, other: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn lerp(a: Vec2, b: Vec2, w: f32) -> Vec2
    {
        let inv = 1.0 - w;
        Vec2
        {
            x: b.x * w + a.x * inv,
            y: b.y * w + a.y * inv,
        }
    }

    pub fn fract_comp(self) -> Vec2
    {
        Vec2
        {
            x: self.x.fract(),
            y: self.y.fract(),
        }
    }

    pub fn from_ivec2(v: IVec2) -> Vec2
    {
        Vec2
        {
            x: v.x as f32,
            y: v.y as f32,
        }
    }

    pub fn respan(self, a: Span2, b: Span2) -> Vec2
    {
        let w = (self - a.min()) / (a.max() - a.min());
        b.min() + w * (b.max() - b.min())
    }

    pub fn from_to_rot(from: Vec2, to: Vec2) -> Complex
    {
        Complex::from(from.nor()).i_inv() * Complex::from(to.nor())
    }

    pub fn constrain_to_rect(&mut self, rect: Span2)
    {
        if self.x < rect.x0 { self.x = rect.x0 }
        if self.y < rect.y0 { self.y = rect.y0 }
        if self.x > rect.x1 { self.x = rect.x1 }
        if self.y > rect.y1 { self.y = rect.y1 }
    }
}

impl From<Dim2> for Vec2
{
    fn from(value: Dim2) -> Self
    {
        Vec2 { x: value.x, y: value.y }
    }
}
