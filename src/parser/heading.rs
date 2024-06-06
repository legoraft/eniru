use super::styling;

pub fn parse(heading: &str) -> String {
    let (count, text) = heading.split_once(" ").unwrap();
    let text = styling::parse(text.to_string());

    let header_tag = format!("<h{}>{}</h{}>", count.len(), text, count.len());
    header_tag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_large_heading() {
        let headings: &str = "\
# This is a single heading";
        let paragraphs: String = "<h1>This is a single heading</h1>".to_string();

        assert_eq!(paragraphs, parse(headings));
    }

    #[test]
    fn parse_small_heading() {
        let headings: &str = "\
### This is a level 3 heading";
        let paragraphs: String = "<h3>This is a level 3 heading</h3>".to_string();

        assert_eq!(paragraphs, parse(headings));
    }
}