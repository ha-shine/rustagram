extern crate image;

use image::{RgbaImage};
use image::imageops;

mod rustaops;

pub enum FilterType {
    NineTeenSeventySeven
}

pub trait RustagramFilter {
    fn apply_filter(&self, ft: FilterType) -> Self;
}

impl RustagramFilter for RgbaImage {
    fn apply_filter(&self, ft: FilterType) -> Self {
        match ft {
            FilterType::NineTeenSeventySeven => apply_1977(&self),
            _ => apply_1977(&self)
        }
    }
}

fn apply_1977(img: &RgbaImage) -> RgbaImage {
    let contrasted = imageops::contrast(img, 10.0);
    let brightened = rustaops::brighten_by_percent(&contrasted, 10.0);
    let saturated = rustaops::saturate(&brightened, 30.0);
    saturated
}

//pub fn over_with_one_rgba(mut base: &mut RgbaImage, target: [u8; 4]) -> Result<RgbaImage, &'static str>  {
//    let mut buf = ImageBuffer::new(base.width(), base.height());
//    for (_, _, pixel) in buf.enumerate_pixels_mut() {
//        *pixel = Rgba(target)
//    }
//    over(&mut base, &mut buf)
//}
//
//pub fn over(background: &mut RgbaImage, foreground: &mut RgbaImage) -> Result<RgbaImage, &'static str>  {
//    if background.width() != foreground.width() || background.height() != foreground.height() {
//        return Err("The two image must have same dimension")
//    }
//
//    // Pre-multiply
//    background.enumerate_pixels_mut().for_each(|(_,_,pixel)| *pixel = Rgba(pre_multiply(&pixel.data)));
//    foreground.enumerate_pixels_mut().for_each(|(_,_,pixel)| *pixel = Rgba(pre_multiply(&pixel.data)));
//
//    let mut buf = ImageBuffer::new(background.width(), background.height());
//    for (x, y, pixel) in buf.enumerate_pixels_mut() {
//        let bg_color = background.get_pixel(x, y).data;
//        let fg_color = foreground.get_pixel(x, y).data;
//        *pixel = Rgba(calculate_over(&bg_color, &fg_color));
//    }
//
//    Ok(buf)
//}
//
//fn pre_multiply(input: &[u8; 4]) -> [u8; 4] {
//    let mut out = [0; 4];
//    out[0] = ((input[0] as f32) * (input[3] as f32) / 255.0) as u8;
//    out[1] = ((input[1] as f32) * (input[3] as f32) / 255.0) as u8;
//    out[2] = ((input[2] as f32) * (input[3] as f32) / 255.0) as u8;
//    out[3] = input[3];
//    out
//}
//
//fn calculate_over(background: &[u8; 4], foreground: &[u8; 4]) -> [u8; 4] {
//    let mut final_pixel = [0; 4];
//    let bg_alpha = background[3] as f32 / 255.0;
//    let fg_alpha = foreground[3] as f32 / 255.0;
//    let final_alpha = bg_alpha + fg_alpha * (1.0 - bg_alpha);
//    for i in 0..3 {
//        let bg_color = background[i] as f32 / 255.0;
//        let fg_color = foreground[i] as f32 / 255.0;
//        let out = bg_color + fg_color * (1.0 - bg_color);
//        final_pixel[i] = (out*255.0) as u8;
//    }
//    final_pixel[3] = (final_alpha*255.0) as u8;
//    final_pixel
//}