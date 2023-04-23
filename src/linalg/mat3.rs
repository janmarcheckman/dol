use std::ops;

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Mat3
{
    pub v00: f32,
    pub v01: f32,
    pub v02: f32,

    pub v10: f32,
    pub v11: f32,
    pub v12: f32,

    pub v20: f32,
    pub v21: f32,
    pub v22: f32,
}

impl ops::Mul<Mat3> for Mat3
{
    type Output = Mat3;

    fn mul(self, r: Mat3) -> Mat3
    {
        Mat3
        {
            v00: self.v00 * r.v00 + self.v01 * r.v10 + self.v02 * r.v20,
            v01: self.v00 * r.v01 + self.v01 * r.v11 + self.v02 * r.v21,
            v02: self.v00 * r.v02 + self.v01 * r.v12 + self.v02 * r.v22,

            v10: self.v10 * r.v00 + self.v11 * r.v10 + self.v12 * r.v20,
            v11: self.v10 * r.v01 + self.v11 * r.v11 + self.v12 * r.v21,
            v12: self.v10 * r.v02 + self.v11 * r.v12 + self.v12 * r.v22,

            v20: self.v20 * r.v00 + self.v21 * r.v10 + self.v22 * r.v20,
            v21: self.v20 * r.v01 + self.v21 * r.v11 + self.v22 * r.v21,
            v22: self.v20 * r.v02 + self.v21 * r.v12 + self.v22 * r.v22,
        }
    }
}

impl ops::Mul<Vec3> for Mat3
{
    type Output = Vec3;

    fn mul(self, r: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.v00 * r.x + self.v01 * r.y + self.v02 * r.z,
            y: self.v10 * r.x + self.v11 * r.y + self.v12 * r.z,
            z: self.v20 * r.x + self.v21 * r.y + self.v22 * r.z,
        }
    }
}

impl Mat3
{
    pub const IDENTITY: Mat3 = Mat3
    {
        v00: 1.0, v01: 0.0, v02: 0.0,
        v10: 0.0, v11: 1.0, v12: 0.0,
        v20: 0.0, v21: 0.0, v22: 1.0,
    };

    pub fn rotation(q: Quat) -> Mat3
    {
        Mat3::axis
        (
            q * Vec3 { x: 1.0, y: 0.0, z: 0.0 },
            q * Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            q * Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        )
    }

    pub fn angle_axis(angle: f32, axis: Vec3) -> Mat3
    {
        Mat3::rotation(Quat::angle_axis(angle, axis))
    }
    
    pub fn swing(from: Vec3, to: Vec3) -> Mat3
    {
        let q = Quat::swing(from, to);
        Mat3::rotation(q)
    }

    pub fn swing_y(to: Vec3) -> Mat3
    {
        Mat3::swing(Vec3::E1, to)
    }
    
    pub fn euler(x: f32, y: f32, z: f32) -> Mat3
    {
        Mat3::rotation(Quat::euler(x, y, z))
    }

    pub fn scale(x: f32) -> Mat3
    {
        Mat3
        {
            v00: x,   v01: 0.0, v02: 0.0,
            v10: 0.0, v11: x,   v12: 0.0,
            v20: 0.0, v21: 0.0, v22: x,
        }
    }

    pub fn scale_dim3(dim: Dim3) -> Mat3
    {
        Mat3
        {
            v00: dim.x,   v01: 0.0, v02: 0.0,
            v10: 0.0, v11: dim.y,   v12: 0.0,
            v20: 0.0, v21: 0.0, v22: dim.z,
        }
    }

    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Mat3
    {
        Mat3
        {
            v00: x,   v01: 0.0, v02: 0.0,
            v10: 0.0, v11: y,   v12: 0.0,
            v20: 0.0, v21: 0.0, v22: z,
        }
    }

    pub fn scale_v(v: Vec3) -> Mat3
    {
        Mat3
        {
            v00: v.x, v01: 0.0, v02: 0.0,
            v10: 0.0, v11: v.y, v12: 0.0,
            v20: 0.0, v21: 0.0, v22: v.z,
        }
    }

    pub fn axis(i: Vec3, j: Vec3, k: Vec3) -> Mat3
    {
        return Mat3
        {
            v00: i.x, v01: j.x, v02: k.x,
            v10: i.y, v11: j.y, v12: k.y,
            v20: i.z, v21: j.z, v22: k.z,
        };
    }

    pub fn scale_vec3(v: Vec3) -> Mat3
    {
        Mat3
        {
            v00: v.x, v01: 0.0, v02: 0.0,
            v10: 0.0, v11: v.y, v12: 0.0,
            v20: 0.0, v21: 0.0, v22: v.z,
        }
    }
    
    pub fn transposed(&self) -> Mat3
    {
        Mat3
        {
            v00: self.v00, v01: self.v10, v02: self.v20,
            v10: self.v01, v11: self.v11, v12: self.v21,
            v20: self.v02, v21: self.v12, v22: self.v22,
        }
    }

    pub fn to_array(&self) -> [f32; 9]
    {
        [
            self.v00, self.v01, self.v02,
            self.v10, self.v11, self.v12,
            self.v20, self.v21, self.v22,
        ]
    }

    pub fn inverse(self) -> Mat3
    {
        let det = self.v00 * (self.v11 * self.v22 - self.v21 * self.v12) -
        self.v01 * (self.v10 * self.v22 - self.v12 * self.v20) +
        self.v02 * (self.v10 * self.v21 - self.v11 * self.v20);

        let invdet = 1.0 / det;

        Mat3
        {
            v00: (self.v11 * self.v22 - self.v21 * self.v12) * invdet,
            v01: (self.v02 * self.v21 - self.v01 * self.v22) * invdet,
            v02: (self.v01 * self.v12 - self.v02 * self.v11) * invdet,
            v10: (self.v12 * self.v20 - self.v10 * self.v22) * invdet,
            v11: (self.v00 * self.v22 - self.v02 * self.v20) * invdet,
            v12: (self.v10 * self.v02 - self.v00 * self.v12) * invdet,
            v20: (self.v10 * self.v21 - self.v20 * self.v11) * invdet,
            v21: (self.v20 * self.v01 - self.v00 * self.v21) * invdet,
            v22: (self.v00 * self.v11 - self.v10 * self.v01) * invdet,
        }
    }

    pub fn lerp(a: Mat3, b: Mat3, w: f32) -> Mat3
    {
        Mat3
        {
            v00: lerp(a.v00, b.v00, w),
            v01: lerp(a.v01, b.v01, w),
            v02: lerp(a.v02, b.v02, w),
            v10: lerp(a.v10, b.v10, w),
            v11: lerp(a.v11, b.v11, w),
            v12: lerp(a.v12, b.v12, w),
            v20: lerp(a.v20, b.v20, w),
            v21: lerp(a.v21, b.v21, w),
            v22: lerp(a.v22, b.v22, w),
        }
    }

    pub fn nor_axis(&self) -> Mat3
    {
        let i = Vec3 { x: self.v00, y: self.v01, z: self.v02 }.nor();
        let j = Vec3 { x: self.v10, y: self.v11, z: self.v12 }.nor();
        let k = Vec3 { x: self.v20, y: self.v21, z: self.v22 }.nor();
        Mat3::axis(i, j, k)
    }
}
