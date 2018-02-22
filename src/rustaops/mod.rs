extern crate image;

use image::{GenericImage, ImageBuffer, Pixel, Rgba};
use image::math::utils::clamp;

mod blend;

pub fn brighten_by_percent<I, P>(image: &I, value: f32) -> ImageBuffer<P, Vec<u8>>
    where I: GenericImage<Pixel=P>,
          P: Pixel<Subpixel=u8> + 'static {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    let max = u8::max_value();
    let max: f32 =  max as f32;
    let percent = (value + 100.0) / 100.0;

    for y in 0..height {
        for x in 0..width {
            let e = image.get_pixel(x, y).map_with_alpha(|b| {
                let c: f32 = b as f32;
                let d = clamp(c * percent, 0.0, max);

                d as u8
            }, |alpha| alpha);

            out.put_pixel(x, y, e);
        }
    }

    out
}

pub fn sepia<I>(image: &I, intensity: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>> {
    let (width, height) = image.dimensions();
    let depth = 20;
    let mut out = ImageBuffer::new(width, height);

    let percent = intensity / 100.0;
    for (x, y, pixel) in out.enumerate_pixels_mut() {
        let channels = image.get_pixel(x, y).data;
        let mut r = channels[0] as u16;
        let mut g = channels[1] as u16;
        let mut b = channels[2] as u16;
        let gray = (r + g + b) / 3;

        r = r + (depth * 2);
        g = g + depth;
        b = gray;

        if r > 255 {
            r = 255
        }
        if g > 255 {
            g = 255
        }
        if b > 255 {
            b = 255
        }

        let f = b as f32;
        b = (f - (f * percent)) as u16;

        if b > 255 {
            b = 255
        }
        *pixel = Rgba([r as u8, g as u8, b as u8, channels[3]]);
    }

    out
}

pub fn fill_with_channels(width: u32, height: u32, channels: &[u8; 4]) -> ImageBuffer<Rgba<u8>, Vec<u8>>
{
    let a = channels[3] as f32;
    let r = ((channels[0] as f32) * a / 255.0) as u8;
    let g = ((channels[1] as f32) * a / 255.0) as u8;
    let b = ((channels[2] as f32) * a / 255.0) as u8;
    let fill = [r, g, b, channels[3]];

    let mut out = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            out.put_pixel(x, y, *Rgba::from_slice(&fill));
        }
    }

    out
}

pub fn restore_transparency<I>(image: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut e = image.get_pixel(x, y).data;
            e[3] = 255;

            out.put_pixel(x, y, *Rgba::from_slice(&e));
        }
    }

    out
}

pub fn over<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    let (width, height) = foreground.dimensions();
    let mut out = ImageBuffer::new(width, height);
    for (x, y, pixel) in out.enumerate_pixels_mut() {
        let fg_data = foreground.get_pixel(x, y).data;
        let bg_data = background.get_pixel(x, y).data;
        let final_alpha = blend::compute_final_alpha(&fg_data, &bg_data);
        let mut final_data = [0; 4];
        final_data[3] = final_alpha;
        for i in 0..3 {
            let fg_c = fg_data[i] as f32 / 255.0;
            let bg_c = bg_data[i] as f32 / 255.0;
            let final_c = fg_c + bg_c * (1.0 - fg_c);
            final_data[i] = (final_c * 255.0) as u8;
        }
        *pixel = Rgba(final_data);
    }

    out
}

#[allow(dead_code)]
pub fn blend_screen<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_screen)
}

#[allow(dead_code)]
pub fn blend_soft_light<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_soft_light)
}

#[allow(dead_code)]
pub fn blend_overlay<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_overlay)
}

#[allow(dead_code)]
pub fn blend_color_dodge<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_color_dodge)
}

#[allow(dead_code)]
pub fn blend_darken<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_darken)
}

#[allow(dead_code)]
pub fn blend_lighten<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_lighten)
}

#[allow(dead_code)]
pub fn blend_multiply<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_multiply)
}

#[allow(dead_code)]
pub fn blend_color_burn<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_color_burn)
}

#[allow(dead_code)]
pub fn blend_linear_burn<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_linear_burn)
}

#[allow(dead_code)]
pub fn blend_linear_dodge<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_linear_dodge)
}

#[allow(dead_code)]
pub fn blend_hard_light<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_hard_light)
}

#[allow(dead_code)]
pub fn blend_vivid_light<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_vivid_light)
}

#[allow(dead_code)]
pub fn blend_linear_light<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_linear_light)
}

#[allow(dead_code)]
pub fn blend_pin_light<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_pin_light)
}

#[allow(dead_code)]
pub fn blend_difference<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_difference)
}

#[allow(dead_code)]
pub fn blend_exclusion<I>(foreground: &I, background: &I) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    process_blend(foreground, background, &blend::blend_exclusion)
}

fn process_blend<I>(foreground: &I, background: &I, f: &Fn(u8, u8) -> u8) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>>
{
    let (width, height) = foreground.dimensions();
    let mut out = ImageBuffer::new(width, height);
    for (x, y, pixel) in out.enumerate_pixels_mut() {
        let fg_data = foreground.get_pixel(x, y).data;
        let bg_data = background.get_pixel(x, y).data;
        let final_r = f(fg_data[0], bg_data[0]);
        let final_g = f(fg_data[1], bg_data[1]);
        let final_b = f(fg_data[2], bg_data[2]);
        let final_alpha = blend::compute_final_alpha(&fg_data, &bg_data);
        *pixel = Rgba([final_r, final_g, final_b, final_alpha]);
    }

    out
}

fn saturate_value(s: f32, percent: f32) -> f32 {
    let mut s = s;
    if percent >= 0.0 {
        let interval = 1.0 - s;
        s = s + percent * interval * s;
    } else {
        s = s + percent * s;
    }
    s
}

fn rgb_to_hls(rgba: &[u8; 4]) -> [f32; 3] {
    let r = rgba[0] as f32 / 255.0;
    let g = rgba[1] as f32 / 255.0;
    let b = rgba[2] as f32 / 255.0;

    let max = float_max(r, g, b);
    let min = float_min(r, g, b);

    let mut hue = 0.0;
    let mut saturation = 0.0;
    let lumination = (max + min) / 2.0;

    if max == min {
        return [hue, lumination, saturation]
    }

    let delta = max - min;
    if lumination < 0.5 {
        saturation = delta / (max + min);
    } else {
        saturation = delta / (2.0 - max - min);
    }

    if r == max {
        hue = (g - b) / delta;
    } else if g == max {
        hue = 2.0 + (b - r) / delta;
    } else {
        hue = 4.0 + (r - g) / delta;
    }

    hue /= 6.0;
    if hue < 0.0 {
        hue += 1.0;
    }

    return [hue, lumination, saturation]
}

pub fn saturate<I>(image: &I, value: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    let percent = value / 100.0;
    for (x, y, pixel) in out.enumerate_pixels_mut() {
        let data = image.get_pixel(x, y).data;
        let mut hls = rgb_to_hls(&data);
        hls[2] = saturate_value(hls[2], percent);
        let rgb = hls_to_rgb(&hls, data[3]);

        *pixel = Rgba(rgb);
    }

    out
}

fn hls_to_rgb(hsl: &[f32; 3], alpha: u8) -> [u8; 4] {
    let (r,g,b,m1,m2);
    let hue = hsl[0];
    let lumination = hsl[1];
    let saturation = hsl[2];
    if saturation == 0.0 {
        r = lumination;
        g = lumination;
        b = lumination;
    } else {
        if lumination <= 0.5 {
            m2 = lumination * (1.0 + saturation);
        } else {
            m2 = lumination + saturation - lumination * saturation;
        }
        m1 = 2.0 * lumination - m2;
        r = hue_to_rgb(m1, m2, hue + (1.0/3.0));
        g = hue_to_rgb(m1, m2, hue);
        b = hue_to_rgb(m1, m2, hue - (1.0/3.0));
    }

    let red = (r * 255.0) as u8;
    let green = (g * 255.0) as u8;
    let blue = (b * 255.0) as u8;
    [red, green, blue, alpha]
}

fn hue_to_rgb(m1: f32, m2: f32, hue: f32) -> f32 {
    let mut hue = hue;
    if hue < 0.0 {
        hue += 1.0;
    } else if hue > 1.0 {
        hue -= 1.0;
    }

    if (6.0 * hue) < 1.0 {
        return m1 + (m2 - m1) * hue * 6.0
    } else if (2.0 * hue) < 1.0 {
        return m2
    } else if (3.0 * hue) < 2.0 {
        return m1 + (m2 - m1) * ((2.0/3.0) - hue) * 6.0
    } else {
        return m1
    }
}

fn float_max(a: f32, b: f32, c: f32) -> f32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

fn float_min(a: f32, b: f32, c: f32) -> f32 {
    if a <= b && a <= c {
        a
    } else if b <= a && b <= c {
        b
    } else {
        c
    }
}