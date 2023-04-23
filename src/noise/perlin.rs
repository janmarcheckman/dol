use crate::linalg::prelude::*;

use super::gradient::{gradient_noise_3d, gradient_noise_2d};

pub fn perlin_2d(mut p: Vec2, mut octaves: u32, s: u32) -> f32
{
    let mut acc = 0.0;
    let mut contrib = 0.5;
    
    while octaves > 0
    {
        acc += gradient_noise_2d(p, s) * contrib;
        contrib *= 0.5;
        p *= 2.0;
        octaves -= 1;
    }
    
    return acc;
}

pub fn perlin_3d(mut p: Vec3, mut octaves: u32, s: u32) -> f32
{
    let mut acc = 0.0;
    let mut contrib = 0.5;
    
    while octaves > 0
    {
        acc += gradient_noise_3d(p, s) * contrib;
        contrib *= 0.5;
        p *= 2.0;
        octaves -= 1;
    }
    
    return acc;
}
