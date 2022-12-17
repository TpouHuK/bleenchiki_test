pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 { return (v, v, v) };
    let i = (h*6.0).trunc();
    let f = (h*6.0)-i;
    let (p, q, t) = (v*(1.0 - s), v*(1.0-s*f), v*(1.0-s*(1.0-f)));
    match i as i32 % 6 {
        0 => { (v, t, p) },
        1 => { (q, v, p) },
        2 => { (p, v, t) },
        3 => { (p, q, v) },
        4 => { (t, p, v) },
        5 => { (v, p, q) },
        _ => { unreachable!() },
    }
}
