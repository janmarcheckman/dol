use std::f32::consts::PI;

use crate::linalg::prelude::*;

use super::{convert::float_construct, hash::Hash};

pub fn rand_vec2(seed: u32) -> Vec2
{
    let ux = (seed ^ 85366318u32).hash();
    let uy = (ux   ^ 41105124u32).hash();
    Vec2::new(float_construct(ux), float_construct(uy))
}

pub fn rand_range(min: f32, max: f32, seed: u32) -> f32
{
    lerp(min, max, rand(seed))
}

pub fn rand_usize_under(max: usize, seed: u32) -> usize
{
    seed.hash() as usize % max
}

pub fn rand_usize_range(min: usize, max: usize, seed: u32) -> usize
{
    rand_usize_under(max - min, seed) + min
}

pub fn rand_vec2_dir(seed: u32) -> Vec2
{
    let angle = rand(seed) * PI;
    Vec2
    {
        x: angle.sin(),
        y: angle.cos(),
    }
}

pub fn rand_vec3(seed: u32) -> Vec3
{
    let ux = (seed ^ 85366318u32).hash();
    let uy = (ux   ^ 41105124u32).hash();
    let uz = (uy   ^ 63952259u32).hash();
    Vec3::new(float_construct(ux), float_construct(uy), float_construct(uz))
}

pub fn rand(seed: u32) -> f32
{
    float_construct((seed ^ 46316592u32).hash())
}

pub fn rand_point_in_box(span: Span3, seed: u32) -> Vec3
{
    Vec3
    {
        x: rand_range(span.x0, span.x1, seed ^ 82374),
        y: rand_range(span.y0, span.y1, seed ^ 92348),
        z: rand_range(span.z0, span.z1, seed ^ 23487),
    }
}

pub fn rand_point_in_rect(span: Span2, seed: u32) -> Vec2
{
    Vec2
    {
        x: rand_range(span.x0, span.x1, seed ^ 45674),
        y: rand_range(span.y0, span.y1, seed ^ 32567),
    }
}

pub fn pick_random<T>(seq: &mut Vec<T>, seed: u32) -> Option<T>
{
    let n = seq.len();
    if n == 0 { return None }
    let i = rand_usize_under(n, seed);
    Some(seq.remove(i))
}

pub fn insert_random<T>(seq: &mut Vec<T>, element: T, seed: u32)
{
    let i = rand_usize_under(seq.len() + 1, seed);
    seq.insert(i, element)
}

pub fn rand_bool(seed: u32) -> bool
{
    seed.hash() % 2 == 0
}
