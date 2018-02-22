extern crate image;
extern crate rustagram;

use rustagram::{RustagramFilter};
use rustagram::FilterType::*;

fn main() {
    let img = image::open("test.jpg").unwrap();
    let out = img.to_rgba().apply_filter(Walden);

    out.save("output.jpg").unwrap();
}