extern crate image;
extern crate rustagram;
#[macro_use]
extern crate clap;

use rustagram::filters::FilterType::*;
use std::process;

fn main() {
    let filter_strings = vec![
        "1977", "aden", "brannan", "brooklyn", "clarendon", "earlybird", "gingham", "hudson",
        "inkwell", "kelvin", "lark", "lofi", "maven", "mayfair", "moon", "nashville",
        "reyes", "rise", "slumber", "stinson", "toaster", "valencia", "walden"
    ];
    let filters = vec![
        NineTeenSeventySeven, Aden, Brannan, Brooklyn, Clarendon, Earlybird, Gingham, Hudson,
        Inkwell, Kelvin, Lark, Lofi, Maven, Mayfair, Moon, Nashville, Reyes, Rise, Slumber, Stinson,
        Toaster, Valencia, Walden
    ];

    let filter_arg = &format!("Filter name: {}", filter_strings.join(", "));
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

    let filter_type = match rustagram::validate_filter_type(filter, &filter_strings, &filters) {
        Ok(item) => item,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };
}