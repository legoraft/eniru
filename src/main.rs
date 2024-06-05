use std::{env, fs};

mod paragraph;
mod header;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1])
        .expect("Couldn't open file!");

    let mut paragraphs = paragraph::parse(file);

    for (id, paragraph) in paragraphs.iter().enumerate() {
        if paragraph.chars().nth(0).expect("Can't parse paragraph!") == '#' {
            paragraphs[id] = header::parse(&paragraph);
        }
    }
}
