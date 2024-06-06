pub mod heading;
pub mod paragraph;
pub mod list;

pub mod styling;

#[derive(PartialEq, Eq, Debug)]
pub enum ParagraphType {
    Heading,
    Code,
    List,
    Text
}

#[derive(PartialEq, Eq, Debug)]
pub struct Paragraph {
    pub paragraph_type: ParagraphType,
    pub text: String,
}

impl Paragraph {
    fn new(text: String) -> Paragraph {
        match &text[..2] {
            "``" => Paragraph {paragraph_type: ParagraphType::Code, text},
            "# " | "##" => Paragraph {paragraph_type: ParagraphType::Heading, text},
            "- " | _ if text.chars().nth(0).unwrap().is_digit(10) && text.chars().nth(1).unwrap() == '.' => Paragraph {paragraph_type: ParagraphType::List, text},
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

    #[test]
    fn code_parsing() {
        let file: String = "\
```
let code = \"possible\";
```

```
var code = \"impossible\";
```".to_string();
        let code = vec![
            Paragraph{
                paragraph_type: ParagraphType::Code,
                text: "```\nlet code = \"possible\";\n```".to_string(),
            },
            Paragraph{
                paragraph_type: ParagraphType::Heading,
                text: "```\nvar code = \"impossible\";\n```".to_string(),
            }];

        assert_eq!(code, parse(file));
    }

    #[test]
    fn heading_parsing() {
        let file: String = "\
## Hello, world!

# This is a better one".to_string();
        let headings = vec![
            Paragraph{
                paragraph_type: ParagraphType::Heading,
                text: "## Hello, world!".to_string(),
            },
            Paragraph{
                paragraph_type: ParagraphType::Heading,
                text: "# This is a better one".to_string(),
            }];

        assert_eq!(headings, parse(file));
    }

    #[test]
    fn list_parsing() {
        let file: String = "\
- lists
- are
- cool

1. ordered
2. are
3. too".to_string();
        let list = vec![
        Paragraph{
            paragraph_type: ParagraphType::List,
            text: "- lists\n- are\n- cool".to_string(),
        },
        Paragraph{
            paragraph_type: ParagraphType::List,
            text: "1. ordered\n2. are\n3. too".to_string(),
        }];

        assert_eq!(list, parse(file));
    }

    #[test]
    fn type_paragraph_parsing() {
        let file: String = "\
## Hello, world!

These are two seperate paragraphs

```
let code = \"possible\";
```

- lists
- are
- cool".to_string();
        let paragraphs: Vec<Paragraph> = vec![
            Paragraph{
                paragraph_type: ParagraphType::Heading,
                text: "## Hello, world!".to_string(),
            },
            Paragraph{
                paragraph_type: ParagraphType::Text,
                text: "These are two seperate paragraphs".to_string(),
            },
            Paragraph{
                paragraph_type: ParagraphType::Code,
                text: "```\nlet code = \"possible\";\n```".to_string(),
            },
            Paragraph{
                paragraph_type: ParagraphType::List,
                text: "- lists\n- are\n- cool".to_string(),
            }];

        assert_eq!(paragraphs, parse(file));
    }
}