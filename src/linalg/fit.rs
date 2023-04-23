pub fn fit(x: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32
{
    let w = (x - old_min) / (old_max - old_min);
    new_min + (new_max - new_min) * w
}
