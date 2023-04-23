#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElementState
{
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
