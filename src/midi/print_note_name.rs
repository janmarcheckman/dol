
pub fn print_note_name(semitone: isize)
{
    let octave = semitone / 12;
    let note = semitone - octave * 12;
    let note_str = match note
    {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => panic!()
    };
    println!("{}{}", note_str, octave);
}
