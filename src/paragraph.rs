pub fn parse(markdown: String) -> Vec<String> {
    let paragraphs: Vec<String> = markdown.split("\n\n").map(|p| p.to_string()).collect();

    paragraphs
}