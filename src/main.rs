use std::{env, fs};

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1])
        .expect("Couldn't open file!");

    let paragraphs: Vec<String> = paragraph::parse(file);
    let mut output_html: Vec<String> = Vec::new();

    for paragraph in paragraphs {
        if paragraph.chars().nth(0).expect("Can't parse paragraph!") == '#' {
            let output = header::parse(&paragraph);
            output_html.push(output);
        } else {
            let output = paragraph;
            output_html.push(output.clone());
        }
    }
}
