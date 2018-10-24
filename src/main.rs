extern crate image;
extern crate turtle;

use image::{ImageBuffer, Rgba, RgbaImage};
use turtle::*;

fn main() {
    let mut img: RgbaImage = ImageBuffer::new(255, 255);

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255])
    }

    let mut turtle = Turtle::default();

    turtle
        .turn(90.0)
        .forward(10.0)
        .turn(-90.0)
        .forward(50.0)
        .turn(90.0)
        .forward(50.0)
        .turn(-43.0)
        .forward(50.0)
        .points()
        .for_each(|Point { x, y }| img.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 255])));

    img.save("test.png").unwrap();
}
