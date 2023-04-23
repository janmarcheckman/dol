use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct USpan2
{
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
}

impl USpan2
{
    pub fn w(&self) -> usize
    {
        self.x1 - self.x0
    }

    pub fn h(&self) -> usize
    {
        self.y1 - self.y0
    }

    pub fn aspect_ratio(&self) -> f32
    {
        self.w() as  f32 / self.h() as f32
    }

    pub fn center_x(&self) -> usize { (self.x0 + self.x1) / 2 }

    pub fn center_y(&self) -> usize { (self.y0 + self.y1) / 2 }

    pub fn split_h(&self, w: f32) -> (USpan2, USpan2)
    {
        let m = (w * (self.y1 - self.y0) as f32) as usize + self.y0;
        (
            USpan2 { x0: self.x0, y0: self.y0, x1: self.x1, y1: m },
            USpan2 { x0: self.x0, y0: m, x1: self.x1, y1: self.y1 }
        )
    }

    pub fn split_v(&self, w: f32) -> (USpan2, USpan2)
    {
        let m = (w * (self.x1 - self.x0) as f32) as usize + self.x0;
        (
            USpan2 { x0: self.x0, y0: self.y0, x1: m, y1: self.y1 },
            USpan2 { x0: m, y0: self.y0, x1: self.x1, y1: self.y1 }
        )
    }
    
    pub fn is_inside(&self, p: Vec2) -> bool
    {
        p.x >= self.x0 as f32 && p.x < self.x1 as f32 && p.y >= self.y0 as f32 && p.y < self.y1 as f32
    }

    pub fn screen_flipped(&self, window_height: usize) -> USpan2
    {
        USpan2{ x0: self.x0, y0: window_height - self.y1, x1: self.x1, y1: window_height - self.y0 }
    }

    pub fn dim(&self) -> UDim2
    {
        UDim2 { x: self.w(), y: self.h() }
    }

    pub fn window_to_rect_clip(self, s: Vec2) -> Vec2
    {
        Vec2
        {
            x: (s.x - self.x0 as f32) / self.w() as f32 * 2.0 - 1.0,
            y: (s.y - self.y0 as f32) / self.h() as f32 * 2.0 - 1.0,
        }
    }

    pub fn rect_clip_to_window(self, v: Vec2) -> Vec2
    {
        Vec2
        {
            x: (v.x + 1.0) / 2.0 * self.w() as f32 + self.x0 as f32,
            y: (v.y + 1.0) / 2.0 * self.h() as f32 + self.y0 as f32,
        }
    }
}

impl From<UDim2> for USpan2
{
    fn from(value: UDim2) -> Self
    {
        USpan2 { x0: 0, y0: 0, x1: value.x, y1: value.y }
    }
}
