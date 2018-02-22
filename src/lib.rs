extern crate image;

use image::{RgbaImage, ConvertBuffer};
use image::imageops;

mod rustaops;

pub enum FilterType {
    NineTeenSeventySeven,
    Aden,
    Brannan,
    Brooklyn,
    Clarendon,
    Earlybird,
    Gingham,
    Hudson,
    Inkwell,
    Kelvin,
    Lark,
    Lofi,
    Maven,
    Mayfair,
    Moon,
    Nashville,
    Test
}

pub trait RustagramFilter {
    fn apply_filter(&self, ft: FilterType) -> Self;
}

impl RustagramFilter for RgbaImage {
    fn apply_filter(&self, ft: FilterType) -> Self {
        match ft {
            FilterType::NineTeenSeventySeven => apply_1977(&self),
            FilterType::Aden => apply_aden(&self),
            FilterType::Brannan => apply_brannan(&self),
            FilterType::Brooklyn => apply_brooklyn(&self),
            FilterType::Clarendon => apply_clarendon(&self),
            FilterType::Earlybird => apply_earlybird(&self),
            FilterType::Gingham => apply_gingham(&self),
            FilterType::Hudson => apply_hudson(&self),
            FilterType::Inkwell => apply_inkwell(&self),
            FilterType::Kelvin => apply_kelvin(&self),
            FilterType::Lark => apply_lark(&self),
            FilterType::Lofi => apply_lofi(&self),
            FilterType::Maven => apply_maven(&self),
            FilterType::Mayfair => apply_mayfair(&self),
            FilterType::Moon => apply_moon(&self),
            FilterType::Nashville => apply_nashville(&self),
            FilterType::Test => apply_test(&self),
        }
    }
}

fn apply_1977(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, 10.0);
    let brightened = rustaops::brighten_by_percent(&contrasted, 10.0);
    let saturated = rustaops::saturate(&brightened, 30.0);
    let foreground = rustaops::fill_with_channels(width, height, &[243,106,188,76]);
    let out = rustaops::blend_screen(&foreground, &saturated);
    out
}

fn apply_aden(img: &RgbaImage) -> RgbaImage {
    let huerotated = imageops::huerotate(img, -20);
    let contrasted = imageops::contrast(&huerotated, -10.0);
    let saturated = rustaops::saturate(&contrasted, -20.0);
    let brightened = rustaops::brighten_by_percent(&saturated, 20.0);
    let out = rustaops::restore_transparency(&brightened);
    out
}

fn apply_brannan(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let with_sepia = rustaops::sepia(img, 20.0);
    let contrasted = imageops::contrast(&with_sepia, 20.0);
    let foreground = rustaops::fill_with_channels(width, height, &[161,44,199,59]);
    let out = rustaops::blend_lighten(&foreground, &contrasted);
    out
}

fn apply_brooklyn(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, -10.0);
    let brightened = rustaops::brighten_by_percent(&contrasted, 10.0);
    let foreground = rustaops::fill_with_channels(width, height, &[168,223,193,150]);
    let background = rustaops::restore_transparency(&brightened);
    let out = rustaops::blend_overlay(&foreground, &background);
    out
}

fn apply_clarendon(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, 20.0);
    let saturated = rustaops::saturate(&contrasted, 35.0);
    let foreground = rustaops::fill_with_channels(width, height, &[127,187,227,101]);
    let out = rustaops::blend_overlay(&foreground, &saturated);
    out
}

fn apply_earlybird(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, -10.0);
    let with_sepia = rustaops::sepia(&contrasted, 5.0);
    let foreground = rustaops::fill_with_channels(width, height, &[208,186,142,150]);
    let out = rustaops::blend_overlay(&with_sepia, &foreground);
    let out = rustaops::restore_transparency(&out);
    out
}

fn apply_gingham(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let brightened = rustaops::brighten_by_percent(img, 5.0);
    let background = imageops::huerotate(&brightened, -10);
    let foreground = rustaops::fill_with_channels(width, height, &[230, 230, 230, 255]);
    let out = rustaops::blend_soft_light(&foreground, &background);
    out
}

fn apply_hudson(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let brightened = rustaops::brighten_by_percent(img, 50.0);
    let constrasted = imageops::contrast(&brightened, -10.0);
    let saturated = rustaops::saturate(&constrasted, 10.0);
    let foreground = rustaops::fill_with_channels(width, height, &[166, 177, 255, 208]);
    let blended = rustaops::blend_multiply(&foreground, &saturated);
    let out = rustaops::restore_transparency(&blended);
    out
}

fn apply_inkwell(img: &RgbaImage) -> RgbaImage {
    let with_sepia = rustaops::sepia(img, 30.0);
    let contrasted = imageops::contrast(&with_sepia, 10.0);
    let brightened = rustaops::brighten_by_percent(&contrasted, 10.0);
    let out = imageops::grayscale(&brightened);
    ConvertBuffer::convert(&out)
}

fn apply_kelvin(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let foreground = rustaops::fill_with_channels(width, height, &[56, 44, 52, 255]);
    let color_dodged = rustaops::blend_color_dodge(img, &foreground);
    let foreground = rustaops::fill_with_channels(width, height, &[183, 125, 33, 255]);
    let out = rustaops::blend_overlay(&foreground, &color_dodged);
    out
}

fn apply_lark(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, -10.0);
    let foreground = rustaops::fill_with_channels(width, height, &[34, 37, 63, 255]);
    let color_dodged = rustaops::blend_color_dodge(&contrasted, &foreground);
    let foreground = rustaops::fill_with_channels(width, height, &[242, 242, 242, 204]);
    let out = rustaops::blend_darken(&foreground, &color_dodged);
    out
}

fn apply_lofi(img: &RgbaImage) -> RgbaImage {
    let saturated = rustaops::saturate(img, 10.0);
    let out = imageops::contrast(&saturated, 50.0);
    out
}

fn apply_maven(img: &RgbaImage) -> RgbaImage {
    let with_sepia = rustaops::sepia(img, 25.0);
    let brightened = rustaops::brighten_by_percent(&with_sepia, -0.05);
    let contrasted = imageops::contrast(&brightened, -0.05);
    let out = rustaops::saturate(&contrasted, 50.0);
    out
}

fn apply_mayfair(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, 10.0);
    let saturated = rustaops::saturate(&contrasted, 10.0);
    let foreground = rustaops::fill_with_channels(width, height, &[255,200,200,153]);
    let out = rustaops::blend_overlay(&foreground, &saturated);
    out
}

fn apply_moon(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let contrasted = imageops::contrast(img, 10.0);
    let brightened = rustaops::brighten_by_percent(&contrasted, 10.0);
    let foreground = rustaops::fill_with_channels(width, height, &[160,160,160,255]);
    let soft_light = rustaops::blend_soft_light(&foreground, &brightened);
    let foreground = rustaops::fill_with_channels(width, height, &[56,56,56,255]);
    let lighten = rustaops::blend_lighten(&foreground, &soft_light);
    let out = imageops::grayscale(&lighten);
    ConvertBuffer::convert(&out)
}

fn apply_nashville(img: &RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let with_sepia = rustaops::sepia(img, 2.0);
    let contrasted = imageops::contrast(&with_sepia, 20.0);
    let brightened = rustaops::brighten_by_percent(&contrasted, 5.0);
    let saturated = rustaops::saturate(&brightened, 20.0);
    let foreground = rustaops::fill_with_channels(width, height, &[247,176,153,243]);
    let darkened = rustaops::blend_darken(&foreground, &saturated);
    let foreground = rustaops::fill_with_channels(width, height, &[0,70,150,230]);
    let out = rustaops::blend_lighten(&foreground, &darkened);
    out
}

fn apply_test(img: &RgbaImage) -> RgbaImage {
    rustaops::saturate(img, 50.0)
}