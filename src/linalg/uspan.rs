use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct USpan
{
    pub x0: usize,
    pub x1: usize,
}

impl USpan
{
    pub fn len(self) -> usize
    {
        self.x1 - self.x0
    }

    pub fn substring(self, s: &str) -> &str
    {
        let a = &s.as_bytes()[self.x0..self.x1];
        std::str::from_utf8(a).unwrap()
    }
}
