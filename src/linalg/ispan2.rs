#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct ISpan2
{
    pub x0: isize,
    pub y0: isize,
    pub x1: isize,
    pub y1: isize,
}

impl ISpan2
{
    pub fn w(&self) -> isize
    {
        self.x1 - self.x0
    }

    pub fn h(&self) -> isize
    {
        self.y1 - self.y0
    }
}
