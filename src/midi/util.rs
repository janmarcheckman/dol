pub fn mtof(m: f32) -> f32 {
    f32::powf(2.0, (m - 69.0) / 12.0) * 440.0
}
