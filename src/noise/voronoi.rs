use crate::linalg::prelude::*;

use super::{rand::rand_vec3, hash::Hash};

pub fn voronoi(p: Vec3, seed: u32) -> Vec3
{
    let base = IVec3::from_vec3(p);
    
    let mut b: [IVec3; 27] = [IVec3::ZERO; 27];
    
    let mut i = 0;
    for x in -1..2
    {
        for y in -1..2
        {
            for z in -1..2
            {
                b[i] = base + IVec3::new(x, y, z);
                i += 1;
            }
        }
    }
    
    let mut r: [Vec3; 27] = [Vec3::ZERO; 27];
    
    for i in 0..27
    {
        r[i] = Vec3::from_ivec3(b[i]) + rand_vec3(b[i].hash() ^ seed);
    }
    
    let mut closesed_point = r[0];
    let mut closesed_dist = (p - r[0]).mag();
    
    for i in 1..27
    {
        let dist = (p - r[i]).mag();
        if dist < closesed_dist
        {
            closesed_point = r[i];
            closesed_dist = dist;
        }
    }
    
    return closesed_point;
}
