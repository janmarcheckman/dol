use serde::{Serialize, Deserialize};

use crate::linalg::prelude::*;

use super::{prelude::{KeyboardInputState, ModifierState}, mouse::MouseButtonsInputState, input_event::InputEvent};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct InputState
{
    pub cursor_pos: Vec2,
    pub keyboard: KeyboardInputState,
    pub mouse: MouseButtonsInputState,
}

impl InputState
{
    pub fn new() -> InputState
    {
        InputState
        {
            cursor_pos: Vec2::ZERO,
            keyboard: KeyboardInputState::new(),
            mouse: MouseButtonsInputState::new(),
        }
    }

    pub fn update(&mut self, event: &InputEvent)
    {
        match event
        {
            InputEvent::CursorPos (cursor_pos) => self.cursor_pos = *cursor_pos,
            InputEvent::Key { key, state } => self.keyboard.set(*key, *state),
            InputEvent::MouseButton { button, state } => self.mouse.set(*button, *state),
            _ => {}
        }
    }

    pub fn get_mods(&self) -> ModifierState
    {
        self.keyboard.get_mods()
    }

    pub fn shift(&self) -> bool
    {
        self.keyboard.left_shift.is_pressed() | self.keyboard.right_shift.is_pressed()
    }
}
