extern crate graphics;
extern crate image;
use std::error::Error;

use graphics::*;
use image::{ImageBuffer, Rgba, RgbaImage};

// TODO: Move me to examples
fn main() -> Result<(), Box<Error>> {
    let image_buffer: &mut RgbaImage = &mut ImageBuffer::new(128, 128);
    // Draw white pixels on entire image buffer
    for (_x, _y, pixel) in image_buffer.enumerate_pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255])
    }

    PolyLine::new(vec![
        (0.0, 0.0),
        (127.0, 0.0),
        (127.0, 127.0),
        (0.0, 127.0),
        (0.0, 0.0),
    ]).render_with(image_buffer, |_| Rgba([255, 0, 0, 255]));

    PolyLine::new(vec![
        (0.0, 63.0),
        (63.0, 0.0),
        (127.0, 63.0),
        (63.0, 127.0),
        (0.0, 63.0),
    ]).render_with(image_buffer, |_| Rgba([0, 255, 255, 255]));

    PolyLine::new(vec![
        (31.5, 31.5),
        (31.5 + 63.0, 31.5),
        (31.5 + 63.0, 31.5 + 63.0),
        (31.5, 31.5 + 63.0),
        (31.5, 31.5),
    ]).render_with(image_buffer, |_| Rgba([0, 0, 255, 255]));

    image_buffer.save("sample.png")?;

    Ok(())
}
