extern crate image;
extern crate rustagram;
#[macro_use]
extern crate clap;

use std::process;

fn main() {
    let filter_arg = &format!("Filter name: {}", rustagram::available_filters().join(", "));
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Htet Aung Shine <h@shine.rocks>")
        (about: "Apply instagram filters to you photos")
        (@arg OUTPUT: -o --out "Output file name")
        (@arg INPUT: +required "Path to the input image file")
        (@arg FILTER: +required filter_arg)
    ).get_matches();

    let output = matches.value_of("OUTPUT").unwrap_or("output.jpg");
    let input = matches.value_of("INPUT").unwrap();
    let filter = matches.value_of("FILTER").unwrap();

    let filter_type = match rustagram::validate_filter_type(&filter) {
        Ok(item) => item,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };
}