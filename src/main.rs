use std::{env, fs};

use parser::{code, heading, list, paragraph, ParagraphType};

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1])
        .expect("Couldn't open file!");

    let paragraphs: Vec<parser::Paragraph> = parser::parse(file);
    let mut output_html: Vec<String> = Vec::new();

    for paragraph in paragraphs {
        let output = match paragraph.paragraph_type {
            ParagraphType::Heading => heading::parse(paragraph.text),
            ParagraphType::List => list::parse(paragraph.text),
            ParagraphType::Text => paragraph::parse(paragraph.text),
            ParagraphType::Code => code::parse(paragraph.text),
            _ => parser::styling::parse(paragraph.text),
        };
        
        output_html.push(output);
    }

    for line in output_html {
        println!("{}", line);
    }
}
