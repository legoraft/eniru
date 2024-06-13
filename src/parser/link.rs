use regex::Regex;

fn parse(text: String) -> String {
    let link_re = Regex::new(r"[(?<t>.*)]((?<l>.*))").unwrap();
    let text = link_re.replace_all(&text, "<a href=\"$l\">$t</a>").to_string();

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_parsing() {
        let paragraph: String = "\
This is a small paragraph with a simple [link](../homepage.md).".to_string();
        let paragraphs: String = "<p>This is a small paragraph with a simple <a href=\"../homepage.md\">link</a>".to_string();

        assert_eq!(paragraphs, parse(paragraph));
    }
}