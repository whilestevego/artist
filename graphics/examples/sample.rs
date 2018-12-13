use std::error::Error;

use graphics::*;
use image::{ImageBuffer, Rgba, RgbaImage};

/// This example generates the sample image shown in the readme.md.
fn main() -> Result<(), Box<Error>> {
    let image_buffer: &mut RgbaImage = &mut ImageBuffer::new(128, 128);
    // Draw white pixels on entire image buffer
    for (_x, _y, pixel) in image_buffer.enumerate_pixels_mut() {
        *pixel = Rgba([0, 0, 0, 255])
    }

    PolyLine::new(vec![
        (0.0, 0.0),
        (127.0, 0.0),
        (127.0, 127.0),
        (0.0, 127.0),
        (0.0, 0.0),
    ])
    .render_with(image_buffer, |_| Rgba([255, 0, 0, 255]));

    PolyLine::new(vec![
        (1.0, 1.0),
        (126.0, 1.0),
        (126.0, 126.0),
        (1.0, 126.0),
        (1.0, 1.0),
    ])
    .render_with(image_buffer, |_| Rgba([255, 0, 0, 255]));

    PolyLine::new(vec![
        (0.0, 63.0),
        (63.0, 0.0),
        (127.0, 63.0),
        (63.0, 127.0),
        (0.0, 63.0),
    ])
    .render_with(image_buffer, |_| Rgba([0, 255, 255, 255]));

    PolyLine::new(vec![
        (31.5, 31.5),
        (31.5 + 63.0, 31.5),
        (31.5 + 63.0, 31.5 + 63.0),
        (31.5, 31.5 + 63.0),
        (31.5, 31.5),
    ])
    .render_with(image_buffer, |_| Rgba([0, 0, 255, 255]));

    Circle::new((127.0 / 2.0, 127.0 / 2.0), 127.0 / 4.0)
        .render_with(image_buffer, |_| Rgba([255, 0, 255, 255]));

    Circle::new((127.0 * 3.0 / 4.0, 127.0 * 1.0 / 4.0), 127.0 / 8.0)
        .render_with(image_buffer, |_| Rgba([0, 255, 0, 255]));

    Circle::new((127.0 * 1.0 / 4.0, 127.0 * 1.0 / 4.0), 127.0 / 8.0)
        .render_with(image_buffer, |_| Rgba([0, 255, 0, 255]));

    image_buffer.save("../sample.png")?;

    Ok(())
}
