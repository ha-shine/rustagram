extern crate image;

pub mod filters;
mod rustaops;

use filters::{FilterType};
use filters::FilterType::*;

pub fn available_filters<'a>() -> Vec<&'a str> {
    vec![
        "1977", "aden", "brannan", "brooklyn", "clarendon", "earlybird", "gingham", "hudson",
        "inkwell", "kelvin", "lark", "lofi", "maven", "mayfair", "moon", "nashville",
        "reyes", "rise", "slumber", "stinson", "toaster", "valencia", "walden"
    ]
}

pub fn validate_filter_type(filter: &str) -> Result<FilterType, &'static str> {
    let filters_strings = vec![
        "1977", "aden", "brannan", "brooklyn", "clarendon", "earlybird", "gingham", "hudson",
        "inkwell", "kelvin", "lark", "lofi", "maven", "mayfair", "moon", "nashville",
        "reyes", "rise", "slumber", "stinson", "toaster", "valencia", "walden"
    ];
    let filters = vec![
        NineTeenSeventySeven, Aden, Brannan, Brooklyn, Clarendon, Earlybird, Gingham, Hudson,
        Inkwell, Kelvin, Lark, Lofi, Maven, Mayfair, Moon, Nashville, Reyes, Rise, Slumber, Stinson,
        Toaster, Valencia, Walden
    ];

    let search_result = filters_strings.iter().enumerate().find(|f| &filter == f.1);
    match search_result {
        Some((i,_)) => Ok(filters[i].clone()),
        None => Err("Invalid filter type")
    }
}