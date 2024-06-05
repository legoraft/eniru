pub fn parse(markdown: String) -> Vec<String> {
    let paragraphs: Vec<String> = markdown.split("\n\n").map(|p| p.to_string()).collect();

    paragraphs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_paragraphs() {
        let file: String = "\
Hello, world!

These are two seperate paragraphs".to_string();
        let paragraphs: Vec<&str> = vec!["Hello, world!", "These are two seperate paragraphs"];

        assert_eq!(paragraphs, parse(file));
    }
}