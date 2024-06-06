use regex::Regex;

use super::styling;

pub fn parse(text: String) -> String {
    let text = styling::parse(text);
    let text = if &text[..1] == "- " { format!("<ul>\n{}\n</ul>", text) } else { format!("<ol>\n{}\n</ol>", text) };

    let list_re = Regex::new(r"- (?<ul>.*)|\d. +(?<ol>.*)").unwrap();
    let output = list_re.replace_all(&text, "<li>$ul$ol</li>").to_string();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unordered_parsing() {
        let list: String = "\
- This
- Is an
- Unordered
- List".to_string();
        let html: String = "<ul>\n<li>This</li>\n<li>Is an</li>\n<li>Unordered</li>\n<li>List</li>\n</ul>".to_string();

        assert_eq!(html, parse(list));
    }

    #[test]
    fn ordered_parsing() {
        let list: String = "\
1. This
2. Is an
3. Ordered
4. List".to_string();
        let html: String = "<ol>\n<li>This</li>\n<li>Is an</li>\n<li>Ordered</li>\n<li>List</li>\n</ol>".to_string();

        assert_eq!(html, parse(list));
    }
}