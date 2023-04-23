use std::path::Path;

use super::midi_song::{MidiSong, MidiTrack};

use super::event::*;

pub fn load_midi_file<P: AsRef<Path>>(path: P) -> Option<MidiSong>
{
    let bytes = std::fs::read(path).ok()?;
    read_midi_file(&bytes)
}

#[repr(C)]
pub struct MidiHeader {
    pub t: u32,
    pub ntracks: u32,
    pub pulses_per_quarter_note: u32,
}

pub fn read_midi_file(bytes: &[u8]) -> Option<MidiSong> {
    let mut cursor = 0;
    let header = read_mthd_chunck(bytes, &mut cursor);

    let mut midi_song = MidiSong {
        t: header.t,
        pulses_per_quarter_note: header.pulses_per_quarter_note,
        tracks: Vec::new(),
    };

    for _ in 0..header.ntracks {
        if let Some(track) = read_mtrk_chunck(bytes, &mut cursor) {
            midi_song.tracks.push(track);
        } else {
            return None;
        }
    }

    Some(midi_song)
}

pub fn read_mthd_chunck(bytes: &[u8], cursor: &mut usize) -> MidiHeader {
    let _mthd_header_bytes = raw_read::<[u8; 4]>(bytes, cursor);

    let _header_length = raw_read::<[u8; 4]>(bytes, cursor);

    let header_u8s = raw_read::<[u8; 6]>(bytes, cursor);

    MidiHeader {
        t: header_u8s[1] as u32 + ((header_u8s[0] as u32) << 8),
        ntracks: header_u8s[3] as u32 + ((header_u8s[2] as u32) << 8),
        pulses_per_quarter_note: header_u8s[5] as u32 + ((header_u8s[4] as u32) << 8),
    }
}

pub fn raw_read<T: Sized>(bytes: &[u8], cursor: &mut usize) -> T
where
    T: Copy,
{
    unsafe {
        let bs = &bytes[*cursor..*cursor + std::mem::size_of::<T>()];
        *cursor += std::mem::size_of::<T>();
        let ptr = bs as *const _ as *const T;
        let x = ptr.as_ref().unwrap();
        *x
    }
}

pub fn read_mtrk_chunck(bytes: &[u8], cursor: &mut usize) -> Option<MidiTrack> {
    let mtrk_header_u8s = raw_read::<[u8; 4]>(bytes, cursor);

    let chunck_length = raw_read::<[u8; 4]>(bytes, cursor);

    let length: usize = ((chunck_length[3] as usize) << 0)
        + ((chunck_length[2] as usize) << 8)
        + ((chunck_length[1] as usize) << 16)
        + ((chunck_length[0] as usize) << 24);

    let data = &bytes[*cursor..*cursor + length];
    *cursor += length;

    if mtrk_header_u8s != *b"MTrk" {
        println!("skipping non MTrk chunck: {:?}", mtrk_header_u8s);
        return None;
    }

    let mut track = MidiTrack {
        dts: Vec::new(),
        events: Vec::new(),
    };

    let mut cursor = 0;
    let mut status = 0;
    while cursor < data.len() {
        let dt = read_var_len(data, &mut cursor);
        track.dts.push(dt);
        if let Some(event) = read_midi_event(data, &mut cursor, &mut status) {
            track.events.push(event);
        } else {
            return None;
        }
    }

    Some(track)
}

pub fn read_var_len(bytes: &[u8], cursor: &mut usize) -> usize {
    let mut value: usize = 0;
    let mut is_last = false;
    while !is_last {
        value = value << 7;
        let b = bytes[*cursor];
        *cursor += 1;
        is_last = (b & 0x80) == 0;
        let d = b & 0x7F;
        value += d as usize;
    }
    return value;
}

pub fn read_midi_event(bs: &[u8], cursor: &mut usize, status: &mut u8) -> Option<MidiEvent> {
    if bs.len() < *cursor + 1 {
        return None;
    }
    let first_u8 = bs[*cursor];
    if first_u8 & 0x80 != 0 {
        *status = first_u8;
        *cursor += 1;
    }

    let a = *status >> 4;
    let b = *status & 0x0F;

    return match a {
        0x08 => {
            if bs.len() < *cursor + 2 {
                return None;
            }
            let r = Some(MidiEvent::NoteOff {
                channel: b,
                pitch: bs[*cursor],
                velocity: bs[*cursor + 1],
            });
            *cursor += 2;
            r
        }
        0x09 => {
            if bs.len() < *cursor + 2 {
                return None;
            }
            let r = Some(MidiEvent::NoteOn {
                channel: b,
                pitch: bs[*cursor],
                velocity: bs[*cursor + 1],
            });
            *cursor += 2;
            r
        }
        0x0B => {
            if bs.len() < *cursor + 2 {
                return None;
            }
            let r = Some(MidiEvent::ControllerChange {
                channel: b,
                controller: bs[*cursor],
                value: bs[*cursor + 1],
            });
            *cursor += 2;
            r
        }
        0x0C => {
            let preset = bs[*cursor];
            *cursor += 1;
            Some(MidiEvent::ProgramChange { channel: b, preset })
        }
        0x0D => {
            if bs.len() < *cursor + 1 {
                return None;
            }
            let r = Some(MidiEvent::ChannelPressure {
                channel: b,
                pressure: bs[*cursor],
            });
            *cursor += 1;
            r
        }
        0x0E => {
            if bs.len() < *cursor + 2 {
                return None;
            }
            let r = Some(MidiEvent::PitchBend {
                channel: b,
                bend_lsb: bs[*cursor],
                position_msb: bs[*cursor + 1],
            });
            *cursor += 2;
            r
        }
        0x0F => match b {
            0x0F => {
                let meta_type = bs[*cursor];
                *cursor += 1;
                let length = bs[*cursor];
                *cursor += 1;

                let meta_event = match meta_type {
                    0x01 => {
                        let xs = &bs[*cursor..*cursor + length as usize];
                        let text = String::from_utf8(xs.to_owned()).unwrap();
                        Some(MidiEvent::Text { text })
                    }
                    0x02 => {
                        let xs = &bs[*cursor..*cursor + length as usize];
                        let text = String::from_utf8(xs.to_owned()).unwrap();
                        Some(MidiEvent::Copyright  { text })
                    }
                    0x03 => {
                        let xs = &bs[*cursor..*cursor + length as usize];
                        let name = String::from_utf8(xs.to_owned()).unwrap();
                        Some(MidiEvent::TrackName { name })
                    }
                    0x05 => {
                        let xs = &bs[*cursor..*cursor + length as usize];
                        let text = String::from_utf8(xs.to_owned()).unwrap();
                        Some(MidiEvent::Lyrics { text })
                    }
                    0x06 => {
                        let xs = &bs[*cursor..*cursor + length as usize];
                        let text = String::from_utf8(xs.to_owned()).unwrap();
                        Some(MidiEvent::Marker { text })
                    }
                    0x2F => Some(MidiEvent::EndOfTrack),
                    0x51 => Some(MidiEvent::SetTempo {
                        microseconds_per_quarter_note: 
                              ((bs[*cursor + 0] as u32) << 16)
                            + ((bs[*cursor + 1] as u32) << 8)
                            + ((bs[*cursor + 2] as u32) << 0),
                    }),
                    0x58 => Some(MidiEvent::TimeSignature {
                        nn: bs[*cursor + 0],
                        dd: bs[*cursor + 1],
                        cc: bs[*cursor + 2],
                        bb: bs[*cursor + 3],
                    }),
                    0x59 => Some(MidiEvent::KeySignature {
                        sf: bs[*cursor + 0],
                        mi: bs[*cursor + 1],
                    }),
                    _ => {
                        println!("unparsed meta event: {:#X}", meta_type);

                        let data = bs[*cursor..*cursor + length as usize].to_owned();
                        Some(MidiEvent::UnknownMetaMessage { meta_type, data })
                    }
                };

                *cursor += length as usize;
                meta_event
            }
            _ => {
                println!("unparsed F message: {:#X}", b);
                None
            }
        },
        x => {
            println!("{:#X}: not not implemented midi value", x);
            None
        }
    };
}
