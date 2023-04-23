use crate::linalg::prelude::*;

use std::path::PathBuf;

use super::{keyboard::KeyboardKey, mouse::MouseButton, element_state::ElementState};

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent
{
    Close,
    Key { key: KeyboardKey, state: ElementState },
    CursorPos (Vec2),
    MouseButton { button: MouseButton, state: ElementState },
    Scroll { x: f32, y: f32 },
    Size (UDim2),
    FileDrop (PathBuf),
    Focused (bool),
    CursorEntered,
    CursorLeft,
}
