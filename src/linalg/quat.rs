use std::{f32::consts::PI, ops};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Quat
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl ops::Add<Quat> for Quat
{
    type Output = Quat;

    fn add(self, r: Quat) -> Quat
    {
        Quat
        {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
            w: self.w + r.w,
        }
    }
}

impl ops::Sub<Quat> for Quat
{
    type Output = Quat;

    fn sub(self, r: Quat) -> Quat
    {
        Quat
        {
            x: self.x - r.x,
            y: self.y - r.y,
            z: self.z - r.z,
            w: self.w - r.w,
        }
    }
}

impl ops::Mul<Quat> for Quat
{
    type Output = Quat;

    fn mul(self, r: Quat) -> Quat
    {
        Quat
        {
            x: self.x * r.w + self.y * r.z - self.z * r.y + self.w * r.x,
            y: -self.x * r.z + self.y * r.w + self.z * r.x + self.w * r.y,
            z: self.x * r.y - self.y * r.x + self.z * r.w + self.w * r.z,
            w: -self.x * r.x - self.y * r.y - self.z * r.z + self.w * r.w,
        }
    }
}

impl ops::Mul<f32> for Quat
{
    type Output = Quat;

    fn mul(self, f: f32) -> Quat
    {
        Quat
        {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
            w: self.w * f,
        }
    }
}

impl ops::Mul<Vec3> for Quat
{
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3
    {
        let xyz = Vec3
        {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        let t = Vec3::cross(xyz, v) * 2.0;
        v + t * self.w + Vec3::cross(xyz, t)
    }
}

impl Quat
{
    pub const IDENTITY: Quat = { Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } };

    pub fn euler(x: f32, y: f32, z: f32) -> Quat
    {
        let cz = f32::cos(z * 0.5);
        let sz = f32::sin(z * 0.5);
        let cy = f32::cos(y * 0.5);
        let sy = f32::sin(y * 0.5);
        let cx = f32::cos(x * 0.5);
        let sx = f32::sin(x * 0.5);

        Quat
        {
            x: cz * cy * sx - sz * sy * cx,
            y: sz * cy * sx + cz * sy * cx,
            z: sz * cy * cx - cz * sy * sx,
            w: cz * cy * cx + sz * sy * sx,
        }
    }

    pub fn to_euler(self) -> (f32, f32, f32)
    {
        let sinr_cosp = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = f32::atan2(sinr_cosp, cosr_cosp);

        let sinp = 2.0 * (self.w * self.y - self.z * self.x);
        let pitch = if f32::abs(sinp) >= 1.0 {
            f32::copysign(PI / 2.0, sinp)
        }
        else
        {
            f32::asin(sinp)
        };

        let siny_cosp = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy_cosp = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = f32::atan2(siny_cosp, cosy_cosp);

        (roll, pitch, yaw)
    }

    pub fn angle_axis(angle: f32, mut axis: Vec3) -> Quat
    {
        axis.normalize();
        let s = f32::sin(angle / 2.0);
        Quat {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: f32::cos(angle / 2.0),
        }
    }

    pub fn swing(from: Vec3, to: Vec3) -> Quat
    {
        let angle = Vec3::angle(from, to);
        let mut axis = Vec3::cross(from, to).nor();
        if axis.x.is_nan() || axis.y.is_nan() || axis.z.is_nan()
        {
            axis = Vec3::E2;
        }
        Quat::angle_axis(angle, axis)
    }
    
    pub fn swing_y(to: Vec3) -> Quat
    {
        let from = Vec3::E1;
        let angle = Vec3::angle(from, to);
        let mut axis = Vec3::cross(from, to).nor();
        if axis.x.is_nan() || axis.y.is_nan() || axis.z.is_nan()
        {
            axis = Vec3::E2;
        }
        Quat::angle_axis(angle, axis)
    }
    
    pub fn inverse(self) -> Quat
    {
        let num2 = (((self.x * self.x) + (self.y * self.y)) + (self.z * self.z)) + (self.w * self.w);
        let num = 1.0 / num2;
        Quat
        {
            x: -self.x * num,
            y: -self.y * num,
            z: -self.z * num,
            w:  self.w * num,
        }
    }

    pub fn abs(self) -> Quat
    {
        Quat { x: self.x.abs(), y: self.y.abs(), z: self.z.abs(), w: self.w.abs() }
    }

    pub fn nor(self) -> Quat
    {
        let length_sqr = self.x * self.x
                            + self.y * self.y
                            + self.z * self.z
                            + self.w * self.w;

        let length = length_sqr.sqrt();

        let inv_length = 1.0 / length;

        Quat { x: self.x * inv_length, y: self.y * inv_length, z: self.z * inv_length, w: self.w * inv_length }  
    }

    pub fn mag(self) -> f32
    {
        let length_sqr = self.x * self.x
                            + self.y * self.y
                            + self.z * self.z
                            + self.w * self.w;

        length_sqr.sqrt()
    }

    pub fn look_rotation(mut look_dir: Vec3) -> Quat
    {
        look_dir.normalize();

        let pitch = f32::asin(look_dir.z);

        let flat = Vec3 { x: look_dir.x, y: look_dir.y, z: 0.0 }.nor();
        let mut yaw = f32::acos(flat.y);
        if flat.x > 0.0
        {
            yaw = -yaw;
        }

        let yaw_rot = Quat::euler(0.0, 0.0, yaw);
        let pitch_rot = Quat::euler(-pitch, 0.0, 0.0);

        yaw_rot * pitch_rot
    }

    pub fn lerp(a: Quat, b: Quat, w: f32) -> Quat
    {
        Quat
        {
            x: lerp(a.x, b.x, w),
            y: lerp(a.y, b.y, w),
            z: lerp(a.z, b.z, w),
            w: lerp(a.w, b.w, w),
        }
    }
}
