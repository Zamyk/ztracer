use crate::bbox::BBox;
use crate::color::ColorRgb;
use crate::flt::FloatP;
use crate::hit::THit;
use crate::point::{Point2, Point3};
use crate::ray::Ray;

pub trait Texture2d<T, C>: Sync {
    fn get(&self, uv: &Point2<C>) -> T;
}

pub struct ColorTexture {
    pub color: ColorRgb
}

impl Texture2d<ColorRgb, f64> for ColorTexture {
    fn get(&self, uv: &Point2<f64>) -> ColorRgb {
        self.color
    }
}

pub struct ProceduralTexture2d<F: Fn(Point2<f64>) -> ColorRgb> {
    pub f: F
}

impl<F: Fn(Point2<f64>) -> ColorRgb + Sync> Texture2d<ColorRgb, f64> for ProceduralTexture2d<F> {
    fn get(&self, uv: &Point2<f64>) -> ColorRgb {
        (self.f)(*uv)
    }
}

pub enum ImageTextureWrapMode {
    Repeat, Clamp
}

pub struct ImageTexture {
    pub pixels: Vec<ColorRgb>,
    pub width: u32,
    pub height: u32,
    pub wrap_mode: ImageTextureWrapMode,
}

impl ImageTexture {

    pub fn get(&self, uv: &Point2<f64>) -> ColorRgb {
        let x = uv.x * (self.width - 1) as f64;
        let y = uv.y * (self.height - 1) as f64;
        let x = x as i64;
        let y = y as i64;

        let (x, y) = match self.wrap_mode {
            ImageTextureWrapMode::Repeat => (x.rem_euclid(self.width as i64), y.rem_euclid(self.height as i64)),
            ImageTextureWrapMode::Clamp => (num::clamp(x, 0, self.width as i64 - 1), num::clamp(y, 0, self.height as i64 - 1)),
        };

        self.pixels[x as usize + y as usize * self.width as usize]
    }

    pub fn bilinear(&self, uv: &Point2<f64>) -> ColorRgb {
        let x1 = uv.x;
        let x2 = uv.x + 1.0 / (self.width - 1) as f64;
        let y1 = uv.y;
        let y2 = uv.y + 1.0 / (self.height - 1) as f64;

        let c11 = self.get(&Point2{x: x1, y: y1});
        let c21 = self.get(&Point2{x: x2, y: y1});
        let c12 = self.get(&Point2{x: x1, y: y2});
        let c22 = self.get(&Point2{x: x2, y: y2});

        let w11 = (1.0 - uv.x.fract()) * (1.0 - uv.y.fract());
        let w12 = uv.x.fract() * (1.0 - uv.y.fract());
        let w21 = (1.0 - uv.x.fract()) * uv.y.fract();
        let w22 = uv.x.fract() * uv.y.fract();

        c11 * w11 + c21 * w21 + c12 * w12 + c22 * w22
    }
}

impl Texture2d<ColorRgb, f64> for ImageTexture {

    fn get(&self, uv: &Point2<f64>) -> ColorRgb {
        self.bilinear(uv)
    }

}