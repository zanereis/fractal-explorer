use crate::fractal::Fractal;

pub fn render_fractal(
    fractal: &(impl Fractal + ?Sized),
    buffer: &mut [u32],
    size: (usize, usize),
    center: (f32, f32),
    zoom: f32,
) {
    for x in 0..size.0 {
        for y in 0..size.1 {
            let px = (x as f32 - size.0 as f32 / 2.0) / zoom + center.0;
            let py = (y as f32 - size.1 as f32 / 2.0) / zoom + center.1;
            let color = fractal.render(px, py);
            buffer[y * size.0 + x] = color;
        }
    }
}
