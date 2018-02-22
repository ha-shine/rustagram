extern crate image;
extern crate rustagram;

use rustagram::filters::{RustagramFilter};
use rustagram::filters::FilterType::*;

fn main() {
    let img = image::open("test.jpg").unwrap();

    let out = img.to_rgba().apply_filter(NineTeenSeventySeven);
    out.save("output/NineTeenSeventySeven.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Aden);
    out.save("output/Aden.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Brannan);
    out.save("output/Brannan.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Brooklyn);
    out.save("output/Brooklyn.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Clarendon);
    out.save("output/Clarendon.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Earlybird);
    out.save("output/Earlybird.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Gingham);
    out.save("output/Gingham.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Hudson);
    out.save("output/Hudson.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Inkwell);
    out.save("output/Inkwell.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Kelvin);
    out.save("output/Kelvin.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Lark);
    out.save("output/Lark.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Lofi);
    out.save("output/Lofi.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Maven);
    out.save("output/Maven.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Mayfair);
    out.save("output/Mayfair.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Moon);
    out.save("output/Moon.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Nashville);
    out.save("output/Nashville.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Reyes);
    out.save("output/Reyes.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Rise);
    out.save("output/Rise.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Slumber);
    out.save("output/Slumber.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Stinson);
    out.save("output/Stinson.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Toaster);
    out.save("output/Toaster.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Valencia);
    out.save("output/Valencia.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Walden);
    out.save("output/Walden.jpg").unwrap();
}