extern crate image;

use image::{RgbaImage};
use image::imageops;

mod rustaops;

pub enum FilterType {
    NineTeenSeventySeven,
    Gingham
}

pub trait RustagramFilter {
    fn apply_filter(&self, ft: FilterType) -> Self;
}

impl RustagramFilter for RgbaImage {
    fn apply_filter(&self, ft: FilterType) -> Self {
        match ft {
            FilterType::NineTeenSeventySeven => apply_1977(&self),
            FilterType::Gingham => apply_gingham(&self),
        }
    }
}

fn apply_1977(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, 10.0);
    let brightened = rustaops::brighten_by_percent(&contrasted, 10.0);
    let saturated = rustaops::saturate(&brightened, 30.0);
    let foreground = rustaops::pre_multiply(&saturated);
    let background = rustaops::fill_with_channels(width, height, &[243,106,188,76]);
    let out = rustaops::blend_screen(&foreground, &background);
    out
}

fn apply_gingham(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let brightened = rustaops::brighten_by_percent(img, 5.0);
    let foreground = imageops::huerotate(&brightened, -10);
    let background = rustaops::fill_with_channels(width, height, &[230, 230, 230, 255]);
    let out = rustaops::blend_soft_light(&foreground, &background);
    out
}