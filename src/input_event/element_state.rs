use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub enum ElementState
{
    #[default]
    Pressed,
    Released,
}

impl ElementState
{
    pub fn is_pressed(self) -> bool
    {
        self == ElementState::Pressed
    }
}
