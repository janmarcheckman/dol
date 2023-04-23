use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub enum MidiEvent
{
    NoteOn { channel: u8, pitch: u8, velocity: u8, },
    NoteOff{ channel: u8, pitch: u8, velocity: u8, },
    ControllerChange{ channel: u8, controller: u8, value: u8, },
    ProgramChange{ channel: u8, preset: u8, },
    ChannelPressure{ channel: u8, pressure: u8, },
    PitchBend{ channel: u8, bend_lsb: u8, position_msb: u8, },
    SysEx{ data: Vec::<u8>, },
    SongPosition{ position_lsb: u8, position_msb: u8, },
    SongSelect{ song_number: u8, },
    BusSelect{ bus_number: u8, },
    TuneRequest,
    TimingTick,
    StartSong,
    ContinueSong,
    StopSong,
    ActiveSensing,
    Text { text: String, },
    Copyright { text: String, },
    Lyrics { text: String, },
    Marker { text: String, },
    TrackName { name: String, },
    InstrumentName { name: String, },
    DeviceName { name: String, },
    MidiPort { port: u8, },
    EndOfTrack,
    SetTempo { microseconds_per_quarter_note: u32, },
    SMTPEOffset { hh: u8, mm: u8, ss: u8, fr: u8, ff: u8, },
    TimeSignature { nn: u8, dd: u8, cc: u8, bb: u8, },
    KeySignature { sf: u8, mi: u8, },
    UnknownMetaMessage { meta_type: u8, data: Vec::<u8> },
}

impl Default for MidiEvent
{
    fn default() -> Self {
        MidiEvent::NoteOn { channel: 0, pitch: 0, velocity: 0 }
    }
}
