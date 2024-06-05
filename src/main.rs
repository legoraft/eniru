use std::{env, fs};

mod paragraph;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1])
        .expect("Couldn't open file!");

    let paragraphs = paragraph::parse(file);
}
