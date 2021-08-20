use lazy_static::*;
use phf::phf_map;
use regex::{Regex, Captures, CaptureMatches};
use std::iter::Map;
use bigdecimal::BigDecimal;
use std::str::FromStr;

static KEYWORDS: phf::Map<&'static str, i32> = phf_map!{"a" => 7, "c" => 6, "h" => 1, "l" => 2, "m" => 2, "q" => 4, "s" => 4, "t" => 2, "v" => 1, "z" => 0};

lazy_static! {
    static ref SEGMENT:Regex = Regex::new(r"(?i)([astvzqmhlc])([^astvzqmhlc]*)").unwrap();
    static ref NUMBERS:Regex = Regex::new(r"(?i)-?[0-9]*\.?[0-9]+(?:e[-+]?\d+)?").unwrap();
}

pub fn parse(path:&str) {
    let mut data = Vec::new();
    for caps in SEGMENT.captures_iter(path) {
        let _type = caps.get(1).unwrap().as_str().to_lowercase();
        let mut args = parse_values(caps.get(2).unwrap().as_str());

        if _type.eq(&'m'.to_string()) && args.len() > 2 {
            data.push([caps.get(1).unwrap().as_str(), args].concat());
            // type = 'l'
            // command = command == 'm' ? 'l' : 'L'
        }
    }
}


fn parse_values(args:&str) -> Vec<BigDecimal> {
    NUMBERS.captures_iter(args).map(|x| BigDecimal::from_str(x.get(0).unwrap().as_str()).unwrap()).collect()
}