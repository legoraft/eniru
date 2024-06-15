use std::{env, fs};

use eniru::parse_markdown;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1])
        .expect("Couldn't open file!");

    let html = parse_markdown(file);
    println!("{html}");
}
