use md_parser::{code, heading, list, paragraph, ParagraphType};

mod md_parser;
pub mod post_generator;

pub fn parse_markdown(file: String) -> String {
    let paragraphs: Vec<md_parser::Paragraph> = md_parser::parse(file);
    let mut output_html: Vec<String> = Vec::new();
    let mut output_string: String = String::new();

    for paragraph in paragraphs {
        let output = match paragraph.paragraph_type {
            ParagraphType::Heading => heading::parse(paragraph.text),
            ParagraphType::List => list::parse(paragraph.text),
            ParagraphType::Text => paragraph::parse(paragraph.text),
            ParagraphType::Code => code::parse(paragraph.text),
        };
        
        output_html.push(output);
    }

    for line in output_html {
        output_string.push_str(&[&line, "\n"].concat());
    }
    
    output_string
}