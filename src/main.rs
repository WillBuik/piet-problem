use std::path::Path;

use piet_common::{Device, kurbo::Rect, Color, RenderContext};

fn problem(width: u32, height: u32, output: &Path) {
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(width as usize, height as usize, 1.0).unwrap();
    let mut rc = bitmap.render_context();

    rc.fill(Rect{ x0: 0., y0: 0., x1: 100., y1: 100. }, &Color::BLUE);
    rc.finish().unwrap();
    drop(rc);

    bitmap
        .save_to_file(output)
        .expect("file save error");
}

fn main() {
    problem(1920, 1080, Path::new("good.png")); // Blue square is blue
    problem(1924, 1080, Path::new("bad.png"));  // Blue square is red, blue and red channels are swapped.
}
