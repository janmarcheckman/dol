use super::element_state::ElementState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton
{
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
pub struct MouseButtonsInputState
{
    pub left: ElementState,
    pub right: ElementState,
    pub middle: ElementState,
}

impl MouseButtonsInputState
{
    pub fn new() -> MouseButtonsInputState
    {
        MouseButtonsInputState
        {
            left: ElementState::Released,
            right: ElementState::Released,
            middle: ElementState::Released,
        }
    }

    pub fn get(&self, button: MouseButton) -> ElementState
    {
        match button
        {
            MouseButton::Left => self.left,
            MouseButton::Right => self.right,
            MouseButton::Middle => self.middle,
        }
    }

    pub fn set(&mut self, button: MouseButton, state: ElementState)
    {
        match button
        {
            MouseButton::Left => self.left = state,
            MouseButton::Right => self.right = state,
            MouseButton::Middle => self.middle = state,
        }
    }
}