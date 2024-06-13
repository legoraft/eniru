use regex::Regex;

pub fn parse(paragraph: String) -> String {
    let paragraph = paragraph
        .replace("<", "&lt;")
        .replace(">", "&gt;");

    let paragraph: String = parse_code(paragraph);
    let paragraph: String = parse_bold(paragraph);
    let paragraph: String = parse_italic(paragraph);

    paragraph
}

fn parse_bold(text: String) -> String {
    let bold_re = Regex::new(r"\*\*(?<b>.*)\*\*").unwrap();
    let text = bold_re.replace_all(&text, "<strong>$b</strong>").to_string();

    text
}

fn parse_italic(text: String) -> String {
    let italic_re = Regex::new(r"_(?<i>.*)_").unwrap();
    let text = italic_re.replace_all(&text, "<em>$i</em>").to_string();

    text
}

fn parse_code(text: String) -> String {
    let code_re = Regex::new(r"`(?<c>.*)`").unwrap();
    let text = code_re.replace_all(&text, "<code>$c</code>").to_string();

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

        assert_eq!(html, parse_italic(paragraph));
    }

    #[test]
    fn code_parsing() {
        let paragraph: String = "Hello, this is some text. I also have this `function()` to explain.".to_string();
        let html: String = "Hello, this is some text. I also have this <code>function()</code> to explain.".to_string();

        assert_eq!(html, parse_code(paragraph));
    }

    #[test]
    fn style_parsing() {
        let paragraph: String = "This is a simple sentence to check if both **bold** and _italic_ text work within Eniru. I also want to test this example `function()`.".to_string();
        let html: String = "This is a simple sentence to check if both <strong>bold</strong> and <em>italic</em> text work within Eniru. I also want to test this example <code>function()</code>.".to_string();

        assert_eq!(html, parse(paragraph));
    }
}