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

pub fn saturate<I>(image: &I, value: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where I: GenericImage<Pixel=Rgba<u8>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    let percent = value / 100.0;
    for y in 0..height {
        for x in 0..width {
            let channels = image.get_pixel(x, y).channels4();
            let hsv = rgb_to_hsv(&[channels.0, channels.1, channels.2, channels.3]);
            let hsv = saturate_hsv(&hsv, percent);
            let rgb = hsv_to_rgb(&hsv);
            out.put_pixel(x, y, *Rgba::from_slice(&rgb));
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

// https://www.pocketmagic.net/enhance-saturation-in-images-programatically/
fn saturate_hsv(hsv: &[f32; 4], percent: f32) -> [f32; 4] {
    let (_, mut s, _) = (hsv[0], hsv[1], hsv[2]);
    if percent >= 0.0 {
        let interval = 1.0 - s;
        s = s + percent * interval * s;
    } else {
        s = s + percent * s;
    }
    [hsv[0], s, hsv[2], hsv[3]]
}

// https://stackoverflow.com/questions/13806483/increase-or-decrease-color-saturation
fn rgb_to_hsv(rgba: &[u8; 4]) -> [f32; 4] {
    let mut h;
    let s;
    let r = rgba[0] as f32 / 255.0;
    let g = rgba[1] as f32 / 255.0;
    let b = rgba[2] as f32 / 255.0;
    let a = rgba[3] as f32;
    let min = float_min(r, g, b);
    let max = float_max(r, g, b);
    let delta = max - min;
    let v = max;
    if max != 0.0 {
        s = delta / max;
    } else {
        s = 0.0;
        h = -1.0;
        return [h, s, 0.0, a];
    }
    match max {
        x if x == r => h = (g - b) / delta,
        x if x == g => h = 2.0 + (b - r) / delta,
        _ => h = 4.0 + (r - g) / delta
    }
    h *= 60.0;
    if h < 0.0 {
        h += 360.0;
    }
    [h, s, v, a]
}

fn hsv_to_rgb(hsv: &[f32; 4]) -> [u8; 4] {
    let mut r;
    let mut g;
    let mut b;
    let a = hsv[3];
    let mut h = hsv[0];
    let s = hsv[1];
    let v = hsv[2];
    if s == 0.0 {
        r = v * 255.0;
        g = r;
        b = g;
        return [r as u8, g as u8, b as u8, a as u8];
    }
    h = h / 60.0;
    let i = h.floor();
    let f = h - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    match i {
        x if x == 0.0 => {
            r = v;
            g = t;
            b = p;
        },
        x if x == 1.0 => {
            r = q;
            g = v;
            b = p;
        },
        x if x == 2.0 => {
            r = p;
            g = v;
            b = t;
        },
        x if x == 3.0 => {
            r = p;
            g = q;
            b = v;
        },
        x if x == 4.0 => {
            r = t;
            g = p;
            b = v;
        },
        _ => {
            r = v;
            g = p;
            b = q;
        }
    }
    r = r * 255.0;
    g = g * 255.0;
    b = b * 255.0;
    return [r as u8, g as u8, b as u8, a as u8]
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