pub fn srgb_to_linear_value(u: u8) -> f32
{
    let s = u as f32 / 255.0;
    if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
}

pub fn linear_to_srgb_value(l: f32) -> u8
{
    let s = if l < 0.0031308 { l * 12.92 } else { 1.055 * l.powf(1.0 / 2.4) };
    (s * 255.0) as u8
}
