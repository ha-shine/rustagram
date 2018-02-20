extern crate image;
extern crate rustagram;

fn main() {
    let img = image::open("test.jpg").unwrap();
    let out = rustagram::over_with_one_rgba(&mut img.to_rgba(), [243,106,188,76]).unwrap();

    out.save("output.png").unwrap();
}