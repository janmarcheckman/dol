use std::ops;

use serde::{Serialize, Deserialize};

use super::{prelude::*, srgb::linear_to_srgb_value};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Col32
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ops::Add<Col32> for Col32
{
    type Output = Col32;

    fn add(self, rhs: Col32) -> Col32
    {
        Col32::from(Color::from(self) + Color::from(rhs))
    }
}

impl ops::Mul<f32> for Col32
{
    type Output = Col32;

    fn mul(self, rhs: f32) -> Col32
    {
        Col32::from(Color::from(self) * rhs)
    }
}

impl Col32
{
    pub const BLACK:   Col32 = Col32 { r: 0x00, g: 0x00, b: 0x00, a: 0xFF };
    pub const WHITE:   Col32 = Col32 { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF };
    pub const RED:     Col32 = Col32 { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF };
    pub const GREEN:   Col32 = Col32 { r: 0x00, g: 0xFF, b: 0x00, a: 0xFF };
    pub const BLUE:    Col32 = Col32 { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF };
    pub const CYAN:    Col32 = Col32 { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF };
    pub const YELLOW:  Col32 = Col32 { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF };
    pub const MAGENTA: Col32 = Col32 { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF };

    pub fn lerp(a: Col32, b: Col32, w: f32) -> Col32
    {
        Col32::from(Color::lerp(a.into(), b.into(), w))
    }

    pub fn gray_tone(x: u8) -> Col32
    {
        Col32 { r: x, g: x, b: x, a: 0xFF }
    }

    pub fn gray_tone_linear(l: f32) -> Col32
    {
        let x = linear_to_srgb_value(l);
        Col32 { r: x, g: x, b: x, a: 0xFF }
    }

    pub fn to_rgb_hex_string(&self) -> String
    {
        let r = (self.r as u32) << 16;
        let g = (self.g as u32) << 8;
        let b = self.b as u32;
        let x = r + g + b;
        format!("{:#08x}", x)
    }

    pub fn as_u32(self) -> u32
    {
        let r = (self.r as u32) << 24;
        let g = (self.g as u32) << 16;
        let b = (self.b as u32) << 8;
        let a = (self.a as u32) << 0;

        let u = r | g | b | a;
        u
    }
}

impl From<u32> for Col32
{
    fn from(value: u32) -> Self
    {
        let r = ((value & 0xff000000) >> 24) as u8;
        let g = ((value & 0x00ff0000) >> 16) as u8;
        let b = ((value & 0x0000ff00) >>  8) as u8;
        let a = ((value & 0x000000ff) >>  0) as u8;
        Col32 { r, g, b, a}
    }
}

impl From<Col32> for u32
{
    fn from(value: Col32) -> Self
    {
        let r = (value.r as u32) << 24;
        let g = (value.g as u32) << 16;
        let b = (value.b as u32) << 8;
        let a = (value.a as u32) << 0;

        let u = r | g | b | a;
        u
    }
}

impl From<Color> for Col32
{
    fn from(value: Color) -> Self
    {
        Col32
        {
            r: linear_to_srgb_value(value.r.clamp(0.0, 1.0)),
            g: linear_to_srgb_value(value.g.clamp(0.0, 1.0)),
            b: linear_to_srgb_value(value.b.clamp(0.0, 1.0)),
            a: (value.a.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::color::col32::Col32;

    #[test]
    fn test_to_rgb_hex_string()
    {
        assert_eq!(Col32::RED.to_rgb_hex_string(),   "0xff0000");
        assert_eq!(Col32::GREEN.to_rgb_hex_string(), "0x00ff00");
        assert_eq!(Col32::BLUE.to_rgb_hex_string(),  "0x0000ff");
    }

    #[test]
    fn test_from_u32()
    {
        assert_eq!(Col32::from(0xff0000ff), Col32::RED);
        assert_eq!(Col32::from(0x00ff00ff), Col32::GREEN);
        assert_eq!(Col32::from(0x0000ffff), Col32::BLUE);
    }

    #[test]
    fn test_as_u32()
    {
        assert_eq!(Col32::RED.as_u32(), 0xff0000ff);
        assert_eq!(Col32::GREEN.as_u32(), 0x00ff00ff);
        assert_eq!(Col32::BLUE.as_u32(), 0x0000ffff);
    }
}