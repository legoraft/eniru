#[derive(PartialEq, Eq, Debug)]
enum ParagraphType {
    Heading,
    Code,
    List,
    Text
}

#[derive(PartialEq, Eq, Debug)]
pub struct Paragraph {
    paragraph_type: ParagraphType,
    text: String,
}

impl Paragraph {
    fn new(text: String) -> Paragraph {
        match text.chars().nth(0).unwrap() {
            '`' => Paragraph {paragraph_type: ParagraphType::Code, text},
            '#' => Paragraph {paragraph_type: ParagraphType::Heading, text},
            '-' => Paragraph {paragraph_type: ParagraphType::List, text},
            _ => Paragraph {paragraph_type: ParagraphType::Text, text},
        }
    }
}

pub fn parse(markdown: String) -> Vec<Paragraph> {
    let text: Vec<String> = markdown.split("\n\n").map(|p| p.to_string()).collect();
    let paragraphs: Vec<Paragraph> = text.iter().map(|p| Paragraph::new(p.to_string())).collect();

    paragraphs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_text_paragraphs() {
        let file: String = "\
Hello, world!

These are two seperate paragraphs".to_string();
        let paragraphs: Vec<Paragraph> = vec![
            Paragraph{
                paragraph_type: ParagraphType::Text,
                text: "Hello, world!".to_string(),
            },
            Paragraph{
                paragraph_type: ParagraphType::Text,
                text: "These are two seperate paragraphs".to_string(),
            }];

        assert_eq!(paragraphs, parse(file));
    }
}