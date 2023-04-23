use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Span3
{
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
}

impl Span3
{
    pub const UNIT: Span3 = Span3 { x0: 0.0, y0: 0.0, z0: 0.0, x1: 1.0, y1: 1.0, z1: 1.0 };
    pub const R1: Span3 = Span3 { x0: -1.0, y0: -1.0, z0: -1.0, x1: 1.0, y1: 1.0, z1: 1.0 };
    pub const S1: Span3 = Span3 { x0: -0.5, y0: -0.5, z0: -0.5, x1: 0.5, y1: 0.5, z1: 1.0 };

    pub fn w(&self) -> f32
    {
        self.x1 - self.x0
    }

    pub fn h(&self) -> f32
    {
        self.y1 - self.y0
    }

    pub fn d(&self) -> f32
    {
        self.z1 - self.z0
    }

    pub fn center(&self) -> Vec3
    {
        Vec3
        {
            x: (self.x0 + self.x1) * 0.5,
            y: (self.y0 + self.y1) * 0.5,
            z: (self.z0 + self.z1) * 0.5,
        }
    }

    pub fn inside(&self, p: Vec3) -> bool
    {
        p.x >= self.x0 && p.x <= self.x1 &&
        p.y >= self.y0 && p.y <= self.y1 &&
        p.z >= self.z0 && p.z <= self.z1
    }

    pub fn dim(&self) -> Dim3
    {
        Dim3 { x: self.x1 - self.x0, y: self.y1 - self.y0, z: self.z1 - self.z0 }
    }

    pub fn grow_to_contain_point(&mut self, p: Vec3)
    {
        self.x0 = self.x0.min(p.x);
        self.y0 = self.y0.min(p.y);
        self.z0 = self.z0.min(p.z);
        self.x1 = self.x1.max(p.x);
        self.y1 = self.y1.max(p.y);
        self.z1 = self.z1.max(p.z);
    }

    pub fn sd(&self, p: Vec3) -> f32
    {
        let pos = self.center();
        let r = Vec3::from(self.dim()) * 0.5;
        let q = (p - pos).abs() - r;
        q.max(Vec3::ZERO).mag() + q.maxcomp().min(0.0)
    } 
}
