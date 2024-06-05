pub fn parse(heading: &str) -> String {
    let (count, text) = heading.split_once(" ").unwrap();

    let header_tag = format!("<h{}>{}</h{}>", count.len(), text, count.len());
    header_tag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_headings() {
        let headings: &str = "\
# This is a single heading";
        let paragraphs: String = "<h1>This is a single heading</h1>".to_string();

        assert_eq!(paragraphs, parse(headings));
    }
}