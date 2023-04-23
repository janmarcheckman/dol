use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::linalg::prelude::*;

use super::{keyboard::KeyboardKey, mouse::MouseButton, element_state::ElementState};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub enum InputEvent
{
    #[default]
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
