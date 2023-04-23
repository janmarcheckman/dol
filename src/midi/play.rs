use serde::{Serialize, Deserialize};

use crate::pipe::Pipe;

use super::{event::MidiEvent, midi_song::MidiSong};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct MidiPlayer
{
    cursors: Vec<MidiTrackCursor>,
    pulse_duration: f64,
}

impl MidiPlayer
{
    pub fn new(song: &MidiSong) -> MidiPlayer
    {
        let quarter_notes_per_second = 2.0;

        MidiPlayer
        {
            cursors: vec![MidiTrackCursor{ elapsed_seconds: 0.0, index: 0 }; song.tracks.len()],
            pulse_duration: 1.0 / (song.pulses_per_quarter_note as f64 * quarter_notes_per_second),
        }
    }

    pub fn reset(&mut self, song: &MidiSong)
    {
        *self = Self::new(song);
    }
    
    pub fn update<P: Pipe<MidiEvent>>(&mut self, dt: f64, song: &MidiSong, midi_pipe: &mut P)
    {
        for (i, cursor) in self.cursors.iter_mut().enumerate()
        {
            cursor.elapsed_seconds += dt;
            let track = &song.tracks[i];

            while cursor.index < track.events.len()
            {
                let pulses_till_next_event = track.dts[cursor.index] as f64;
                let seconds_till_next_event = pulses_till_next_event * self.pulse_duration;
                if seconds_till_next_event > cursor.elapsed_seconds
                {
                    break;
                }

                let midi_event = track.events[cursor.index].clone();
                cursor.elapsed_seconds -= seconds_till_next_event;
                cursor.index += 1;
                
                if let MidiEvent::SetTempo { microseconds_per_quarter_note } = midi_event
                {
                    self.pulse_duration = microseconds_per_quarter_note as f64 / (song.pulses_per_quarter_note as f64 * 1000000.0);
                }
                else
                {
                    midi_pipe.send(midi_event);
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
struct MidiTrackCursor
{
    elapsed_seconds: f64,
    index: usize,
}
