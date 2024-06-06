use super::styling;

pub fn parse(text: String) -> String {
    let text = styling::parse(text);
    let output = format!("<p>{}</p>", text);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_parsing() {
        let paragraph: String = "\
This is a small paragraph to test the tagging.".to_string();
        let paragraphs: String = "<p>This is a small paragraph to test the tagging.</p>".to_string();

        assert_eq!(paragraphs, parse(paragraph));
    }

    #[test]
    fn styled_paragraph_parsing() {
        let paragraph: String = "\
This is a small paragraph with a bit of **styling** to test the _tagging_.".to_string();
        let paragraphs: String = "<p>This is a small paragraph with a bit of <strong>styling</strong> to test the <em>tagging</em>.</p>".to_string();

        assert_eq!(paragraphs, parse(paragraph));
    }
}