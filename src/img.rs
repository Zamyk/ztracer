use crate::texture::{ImageTexture, ImageTextureWrapMode};
use image::{GenericImageView, ImageReader};
use std::io;
use crate::color::ColorRgb;

pub fn load_texture(path: &str) -> Option<ImageTexture> {
    let file = ImageReader::open(path);
    if let Ok(data) = file {
        if let Ok(img) = data.decode() {
            let mut pixels = vec![ ColorRgb{r: 0.0, g: 0.0, b: 0.0} ; (img.width() * img.height()) as usize];
            for y in 0..img.height() {
                for x in 0..img.width() {
                    let color = img.get_pixel(x, y).0;
                    let convert = |i: u8| -> f64 { i as f64 / 255.0 };

                    let r = convert(color[0]);
                    let g = convert(color[1]);
                    let b = convert(color[2]);
                    pixels[ (y * img.width() + x) as usize ] = ColorRgb{r, g, b};
                }
            }
            return Some(ImageTexture{pixels, wrap_mode: ImageTextureWrapMode::Repeat, width: img.width(), height: img.height()});
        }
    }
    return None;
}