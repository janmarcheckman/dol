use super::event::MidiEvent;

pub struct MidiSong
{
    pub t: u32,
    pub pulses_per_quarter_note: u32,
    pub tracks: Vec<MidiTrack>,
}

pub struct MidiTrack
{
    pub dts: Vec<usize>,
    pub events: Vec<MidiEvent>,
}
