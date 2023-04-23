use serde::{Serialize, Deserialize};

use super::event::MidiEvent;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct MidiSong
{
    pub t: u32,
    pub pulses_per_quarter_note: u32,
    pub tracks: Vec<MidiTrack>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct MidiTrack
{
    pub dts: Vec<usize>,
    pub events: Vec<MidiEvent>,
}
