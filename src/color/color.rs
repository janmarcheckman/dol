use std::ops;

use serde::{Serialize, Deserialize};

use super::{prelude::*, srgb::srgb_to_linear_value};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Color
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color
{
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };

    pub fn gray_tone(tone: f32) -> Color { Color { r: tone, g: tone, b: tone, a: 1.0 } }

    pub fn scaled(self, factor: f32) -> Color
    {
        Color
        {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
            a: self.a * factor,
        }
    }

    pub fn lerp(a: Color, b: Color, w: f32) -> Color
    {
        b.scaled(w) + a.scaled(1.0 - w)
    }

    pub fn srgb_to_linear(&self) -> Color
    {
        Color
        {
            r: srgb_to_linear_value((self.r * 255.0) as u8),
            g: srgb_to_linear_value((self.g * 255.0) as u8),
            b: srgb_to_linear_value((self.b * 255.0) as u8),
            a: self.a,
        }
    }
}

impl From<Col32> for Color
{
    fn from(col_32: Col32) -> Self
    {
        Color
        {
            r: srgb_to_linear_value(col_32.r),
            g: srgb_to_linear_value(col_32.g),
            b: srgb_to_linear_value(col_32.b),
            a: col_32.a as f32 / 255.0,
        }
    }
}

impl From<u32> for Color
{
    fn from(x: u32) -> Self
    {
        Color::from(Col32::from(x))
    }
}

impl ops::Add for Color
{
    type Output = Color;
    fn add(self, other: Color) -> Color
    {
        Color
        {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl ops::AddAssign for Color
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.r += rhs.r;
        self.g += rhs.b;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl ops::Mul<f32> for Color
{
    type Output = Color;

    fn mul(self, rhs: f32) -> Color
    {
        Color
        {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}
