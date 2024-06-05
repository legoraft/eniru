use regex::Regex;

fn parse(paragraph: String) -> String {
    let paragraph: String = parse_bold(paragraph);
    let paragraph: String = parse_italic(paragraph);

    paragraph
}

fn parse_bold(text: String) -> String {
    let bold_re = Regex::new(r"\*\*(?<s>.*)\*\*").unwrap();
    let text = bold_re.replace_all(&text, "<strong>$s</strong>").to_string();

    text
}

fn parse_italic(text: String) -> String {
    let italic_re = Regex::new(r"_(?<s>.*)_").unwrap();
    let text = italic_re.replace_all(&text, "<em>$s</em>").to_string();

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_parsing() {
        let paragraph: String = "Hello, this is some text. I also have some **bold** text.".to_string();
        let html: String = "Hello, this is some text. I also have some <strong>bold</strong> text.".to_string();

        assert_eq!(html, parse_bold(paragraph));
    }

    #[test]
    fn italic_parsing() {
        let paragraph: String = "Hello, this is some text. I also have some _italic_ text.".to_string();
        let html: String = "Hello, this is some text. I also have some <em>italic</em> text.".to_string();

        assert_eq!(html, parse_bold(paragraph));
    }
}