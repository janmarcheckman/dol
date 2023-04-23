use crate::linalg::prelude::*;

use super::{hash::Hash, rand::{rand_vec2, rand_vec3}};

pub fn gradient_noise_2d(p: Vec2, seed: u32) -> f32
{
    let i = IVec2::from_vec2(p);
    let f = p - Vec2::from_ivec2(i);

    let u = f*f*(Vec2::ONE * 3.0 - f * 2.0);
    
    let r00 = rand_vec2((i + IVec2::new(0,0)).hash() ^ seed) * 2.0 - Vec2::new(1.0, 1.0);
    let r01 = rand_vec2((i + IVec2::new(1,0)).hash() ^ seed) * 2.0 - Vec2::new(1.0, 1.0);
    let r10 = rand_vec2((i + IVec2::new(0,1)).hash() ^ seed) * 2.0 - Vec2::new(1.0, 1.0);
    let r11 = rand_vec2((i + IVec2::new(1,1)).hash() ^ seed) * 2.0 - Vec2::new(1.0, 1.0);
    
    let x00 = Vec2::dot(r00, f - Vec2::new(0.0,0.0));
    let x01 = Vec2::dot(r01, f - Vec2::new(1.0,0.0));
    let x10 = Vec2::dot(r10, f - Vec2::new(0.0,1.0));
    let x11 = Vec2::dot(r11, f - Vec2::new(1.0,1.0));
    
    let x0 = lerp(x00, x01, u.x);
    let x1 = lerp(x10, x11, u.x);

    lerp(x0, x1, u.y)
}

pub fn gradient_noise_3d(p: Vec3, seed: u32) -> f32
{
    let i = IVec3::from_vec3(p);
    let f = p - Vec3::from_ivec3(i);

    let u = f*f*(Vec3::ONE * 3.0 - f * 2.0);
    
    let r000 = rand_vec3((i + IVec3::new(0,0,0)).hash() ^ seed) * 2.0 - 1.0;
    let r010 = rand_vec3((i + IVec3::new(1,0,0)).hash() ^ seed) * 2.0 - 1.0;
    let r100 = rand_vec3((i + IVec3::new(0,1,0)).hash() ^ seed) * 2.0 - 1.0;
    let r110 = rand_vec3((i + IVec3::new(1,1,0)).hash() ^ seed) * 2.0 - 1.0;
    let r001 = rand_vec3((i + IVec3::new(0,0,1)).hash() ^ seed) * 2.0 - 1.0;
    let r011 = rand_vec3((i + IVec3::new(1,0,1)).hash() ^ seed) * 2.0 - 1.0;
    let r101 = rand_vec3((i + IVec3::new(0,1,1)).hash() ^ seed) * 2.0 - 1.0;
    let r111 = rand_vec3((i + IVec3::new(1,1,1)).hash() ^ seed) * 2.0 - 1.0;
    
    let x000 = Vec3::dot(r000, f - Vec3::new(0.0,0.0,0.0));
    let x010 = Vec3::dot(r010, f - Vec3::new(1.0,0.0,0.0));
    let x100 = Vec3::dot(r100, f - Vec3::new(0.0,1.0,0.0));
    let x110 = Vec3::dot(r110, f - Vec3::new(1.0,1.0,0.0));
    let x001 = Vec3::dot(r001, f - Vec3::new(0.0,0.0,1.0));
    let x011 = Vec3::dot(r011, f - Vec3::new(1.0,0.0,1.0));
    let x101 = Vec3::dot(r101, f - Vec3::new(0.0,1.0,1.0));
    let x111 = Vec3::dot(r111, f - Vec3::new(1.0,1.0,1.0));
    
    let x00 = lerp(x000, x001, u.z);
    let x01 = lerp(x010, x011, u.z);
    let x10 = lerp(x100, x101, u.z);
    let x11 = lerp(x110, x111, u.z);

    let x0 = lerp(x00, x01, u.x);
    let x1 = lerp(x10, x11, u.x);

    lerp(x0, x1, u.y)
}
