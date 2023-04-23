use serde::{Serialize, Deserialize};

use super::element_state::ElementState;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub enum MouseButton
{
    #[default]
    Left,
    Right,
    Middle,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
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