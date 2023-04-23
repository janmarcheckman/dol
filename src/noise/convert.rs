use std::mem::transmute;

pub fn f32_bits_to_u32(f: f32) -> u32
{
    unsafe { transmute::<f32, u32>(f) }
}

pub fn u32_bits_to_f32(u: u32) -> f32
{
    unsafe { transmute::<u32, f32>(u) }
}

pub fn i32_bits_to_u32(i: i32) -> u32
{
    unsafe { transmute::<i32, u32>(i) }
}

pub fn float_construct(mut m: u32) -> f32
{
    let ieee_mantissa = 0x007FFFFFu32;
    let ieee_one      = 0x3F800000u32;
    m &= ieee_mantissa;
    m |= ieee_one;
    let  f = u32_bits_to_f32(m);
    f - 1.0
}
