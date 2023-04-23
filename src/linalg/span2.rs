use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Span2
{
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Span2
{
    pub const UNIT: Span2 = Span2 { x0: 0.0, y0: 0.0, x1: 1.0, y1: 1.0 };
    pub const R1: Span2 = Span2 { x0: -1.0, y0: -1.0, x1: 1.0, y1: 1.0 };
    pub const S1: Span2 = Span2 { x0: -0.5, y0: -0.5, x1: 0.5, y1: 0.5 };

    pub fn w(&self) -> f32
    {
        self.x1 - self.x0
    }

    pub fn h(&self) -> f32
    {
        self.y1 - self.y0
    }

    pub fn center(&self) -> Vec2
    {
        Vec2
        {
            x: (self.x0 + self.x1) * 0.5,
            y: (self.y0 + self.y1) * 0.5,
        }
    }

    pub fn contains(&self, p: Vec2) -> bool
    {
        p.x >= self.x0 && p.x <= self.x1 && p.y >= self.y0 && p.y <= self.y1
    }

    pub fn dim(&self) -> Dim2
    {
        Dim2 { x: self.w(), y: self.h() }
    }

    pub fn from_pos_dim(pos: Vec2, dim: Dim2) -> Span2
    {
        Span2 { x0: pos.x, y0: pos.y, x1: pos.x + dim.x, y1: pos.y + dim.y }
    }

    pub fn min(self) -> Vec2
    {
        Vec2 { x: self.x0, y: self.y0 }
    }

    pub fn max(self) -> Vec2
    {
        Vec2 { x: self.x1, y: self.y1 }
    }

    pub fn area(self) -> f32
    {
        self.dim().area()
    }
}

impl From<USpan2> for Span2
{
    fn from(value: USpan2) -> Self
    {
        Span2 { x0: value.x0 as f32, y0: value.y0 as f32, x1: value.x1 as f32, y1: value.y1 as f32 }
    }
}

impl From<Dim2> for Span2
{
    fn from(value: Dim2) -> Self
    {
        Span2 { x0: 0.0, y0: 0.0, x1: value.x, y1: value.y }
    }
}

impl From<UDim2> for Span2
{
    fn from(value: UDim2) -> Self
    {
        Span2 { x0: 0.0, y0: 0.0, x1: value.x as f32, y1: value.y as f32 }
    }
}
