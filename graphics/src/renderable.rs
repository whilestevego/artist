use crate::*;
use image::{Rgba, RgbaImage};

// TODO: Support any pixel type
pub trait Renderable
where
    Self: Sized + Plotable,
{
    fn render(self, image_buffer: &mut RgbaImage) {
        self.plot().for_each(|Point(x, y)| {
            let (x, y) = (x as u32, y as u32);

            if x < image_buffer.width() && y < image_buffer.height() {
                image_buffer.put_pixel(x, y, Rgba([0, 0, 0, 255]))
            }
        })
    }

    fn render_with(self, image_buffer: &mut RgbaImage, draw_fn: impl Fn((u32, u32)) -> Rgba<u8>) {
        self.plot().for_each(|Point(x, y)| {
            let (x, y) = (x as u32, y as u32);

            if x < image_buffer.width() && y < image_buffer.height() {
                image_buffer.put_pixel(x, y, draw_fn((x, y)))
            }
        })
    }
}

impl Renderable for Line {}
impl Renderable for PolyLine {}
impl Renderable for Circle {}
