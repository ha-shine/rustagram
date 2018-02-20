extern crate image;

use image::{ImageBuffer, Rgba, RgbaImage};

pub fn over_with_one_rgba(mut base: &mut RgbaImage, target: [u8; 4]) -> Result<RgbaImage, &'static str>  {
    let mut buf = ImageBuffer::new(base.width(), base.height());
    for (_, _, pixel) in buf.enumerate_pixels_mut() {
        *pixel = Rgba(target)
    }
    over(&mut base, &mut buf)
}

pub fn over(base: &mut RgbaImage, target: &mut RgbaImage) -> Result<RgbaImage, &'static str>  {
    if base.width() != target.width() || base.height() != target.height() {
        return Err("The two image must have same dimension")
    }

    // Pre-multiply
    base.enumerate_pixels_mut().for_each(|(_,_,pixel)| *pixel = Rgba(pre_multiply(&pixel.data)));
    target.enumerate_pixels_mut().for_each(|(_,_,pixel)| *pixel = Rgba(pre_multiply(&pixel.data)));

    let mut buf = ImageBuffer::new(base.width(), base.height());
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let bpixel = base.get_pixel(x, y).data;
        let tpixel = target.get_pixel(x, y).data;
        *pixel = Rgba(calculate_over(&bpixel, &tpixel));
    }
    Ok(buf)
}

fn pre_multiply(input: &[u8; 4]) -> [u8; 4] {
    let mut out = [0; 4];
    out[0] = ((input[0] as f32) * (input[3] as f32) / 255.0) as u8;
    out[1] = ((input[1] as f32) * (input[3] as f32) / 255.0) as u8;
    out[2] = ((input[2] as f32) * (input[3] as f32) / 255.0) as u8;
    out[3] = input[3];
    out
}

fn calculate_over(background: &[u8; 4], foreground: &[u8; 4]) -> [u8; 4] {
    let mut final_pixel = [0; 4];
    let bg_alpha = background[3] as f32 / 255.0;
    let fg_alpha = foreground[3] as f32 / 255.0;
    let final_alpha = bg_alpha + fg_alpha * (1.0 - bg_alpha);
    for i in 0..3 {
        let bg_color = background[i] as f32 / 255.0;
        let fg_color = foreground[i] as f32 / 255.0;
        let out = bg_color + fg_color * (1.0 - bg_color);
        final_pixel[i] = (out*255.0) as u8;
    }
    final_pixel[3] = (final_alpha*255.0) as u8;
    final_pixel
}