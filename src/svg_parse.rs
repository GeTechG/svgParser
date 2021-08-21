use lazy_static::*;
use phf::phf_map;
use regex::{Regex, Captures, CaptureMatches};
use std::iter::Map;
use bigdecimal::BigDecimal;
use std::str::FromStr;

static LENGTH: phf::Map<&'static str, i32> = phf_map!{"a" => 7, "c" => 6, "h" => 1, "l" => 2, "m" => 2, "q" => 4, "s" => 4, "t" => 2, "v" => 1, "z" => 0};

lazy_static! {
    static ref SEGMENT:Regex = Regex::new(r"(?i)([astvzqmhlc])([^astvzqmhlc]*)").unwrap();
    static ref NUMBERS:Regex = Regex::new(r"(?i)-?[0-9]*\.?[0-9]+(?:e[-+]?\d+)?").unwrap();
}

pub fn parse(path:&str) -> Vec<Vec<String>> {
    let mut data: Vec<Vec<String>> = Vec::new();
    for caps in SEGMENT.captures_iter(path) {
        let mut command = caps.get(1).unwrap().as_str();
        let mut _type:String = command.to_lowercase();
        let mut args = parse_values(caps.get(2).unwrap().as_str());

        if _type.eq(&"m".to_string()) && args.len() > 2 {
            let mut temp = vec![String::from(caps.get(1).unwrap().as_str())];
            data.push([temp, args.splice(0..2,None).collect()].concat());
            _type = 'l'.to_string();
            command = if command == "m" { "l" } else { "L" };
        }

        loop {
            if args.len() == *LENGTH.get(_type.as_str()).unwrap() as usize {
                args.insert(0, command.parse().unwrap());
                data.push(args);
                break;
            }
            if args.len() < *LENGTH.get(_type.as_str()).unwrap() as usize {
                panic!("malformed path data");
            }
            let temp = vec![String::from(caps.get(1).unwrap().as_str())];
            data.push([temp, args.splice(0..*LENGTH.get(_type.as_str()).unwrap() as usize, None).collect()].concat())
        }
    }
    println!("{:?}", data);
    data
}


fn parse_values(args:&str) -> Vec<String> {
    NUMBERS.captures_iter(args).map(|x| String::from(x.get(0).unwrap().as_str())).collect()
}