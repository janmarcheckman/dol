use crate::linalg::prelude::*;

use super::{convert::float_construct, hash::Hash, smoothstep::smoothstep};

pub fn value_noise(p: Vec3, seed: u32) -> f32
{
    let cell = IVec3::from_vec3(p);
    let w = p.fract_comp();

    let su = (seed ^ 73145542u32).hash();
    
    let x000 = float_construct((cell + IVec3::new(0,0,0)).hash() ^ su);
    let x001 = float_construct((cell + IVec3::new(0,0,1)).hash() ^ su);
    let x010 = float_construct((cell + IVec3::new(0,1,0)).hash() ^ su);
    let x011 = float_construct((cell + IVec3::new(0,1,1)).hash() ^ su);
    let x100 = float_construct((cell + IVec3::new(1,0,0)).hash() ^ su);
    let x101 = float_construct((cell + IVec3::new(1,0,1)).hash() ^ su);
    let x110 = float_construct((cell + IVec3::new(1,1,0)).hash() ^ su);
    let x111 = float_construct((cell + IVec3::new(1,1,1)).hash() ^ su);

    let wx = smoothstep(0.0, 1.0, w.x);
    let wy = smoothstep(0.0, 1.0, w.y);
    let wz = smoothstep(0.0, 1.0, w.z);

    let x00 = lerp(x000, x001, wz);
    let x01 = lerp(x010, x011, wz);
    let x10 = lerp(x100, x101, wz);
    let x11 = lerp(x110, x111, wz);
    
    let x0 = lerp(x00, x01, wy);
    let x1 = lerp(x10, x11, wy);

    let x = lerp(x0, x1, wx);
    
    return x;
}
