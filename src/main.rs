mod svg_parse;

fn main() {
    let test_path = "M10 10 H 90 V 90 H 10 Z";
    svg_parse::parse(test_path);
}
