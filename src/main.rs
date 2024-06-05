use std::{env, fs};

use parser::{header, paragraph, styling};

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1])
        .expect("Couldn't open file!");

    let paragraphs: Vec<paragraph::Paragraph> = paragraph::parse(file);
    let mut output_html: Vec<String> = Vec::new();

/*    for paragraph in paragraphs {
        if paragraph.chars().nth(0).expect("Can't parse paragraph!") == '#' {
            let output = header::parse(&paragraph);
            output_html.push(output);
        } else {
            let output = styling::parse(paragraph);
            output_html.push("<p>".to_string());
            output_html.push(output.clone());
            output_html.push("</p>".to_string());
        }
    }

*/
    for line in output_html {
        println!("{}", line);
    }
}
