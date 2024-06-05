use regex::Regex;

fn parse_bold(paragraph: String) -> String {
    let bold_re = Regex::new(r"\*\*(?<s>.*)\*\*").unwrap();
    let paragraph = bold_re.replace_all(&paragraph, "<b>$s</b>").to_string();

    paragraph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_parsing() {
        let paragraph: String = "Hello, this is some text. I also have some **bold** text.".to_string();
        let html: String = "Hello, this is some text. I also have some <b>bold</b> text.".to_string();

        assert_eq!(html, parse_bold(paragraph));
    }
}