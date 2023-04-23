use crate::linalg::prelude::*;

use super::convert::{f32_bits_to_u32, i32_bits_to_u32};

pub trait Hash
{
    fn hash(self) -> u32;
}

impl Hash for u32
{
    fn hash(mut self) -> u32
    {
        self = self.wrapping_add(self << 10u32);
        self = self ^ (self >>  6u32);
        self = self.wrapping_add(self <<  3u32);
        self = self ^ (self >> 11u32);
        self = self.wrapping_add(self << 15u32);
        self
    }
}

impl Hash for f32
{
    fn hash(self) -> u32
    {
        f32_bits_to_u32(self).hash()
    }
}

impl Hash for i32
{
    fn hash(self) -> u32
    {
        i32_bits_to_u32(self).hash()
    }
}

impl Hash for IVec2
{
    fn hash(self) -> u32
    {
        (i32_bits_to_u32(self.x as i32) ^ (self.y as i32).hash()).hash()
    }
}

impl Hash for IVec3
{
    fn hash(self) -> u32
    {
        (i32_bits_to_u32(self.x as i32) ^ (self.y as i32).hash() ^ (self.z as i32).hash()).hash()
    }
}

impl Hash for Vec3
{
    fn hash(self) -> u32
    {
        (f32_bits_to_u32(self.x) ^ self.y.hash() ^ self.z.hash()).hash()
    }
}
