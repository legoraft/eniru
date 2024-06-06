use regex::Regex;

use super::styling;

pub fn parse(text: String) -> String {
    let text = styling::parse(text);
    let text = if &text[..1] == "- " { format!("<ul>\n{}\n</ul>", text) } else { format!("<ol>\n{}\n</ol>", text) };

    let list_re = Regex::new(r"- |\d. +(?<l>.*)").unwrap();
    let output = list_re.replace_all(&text, "<li>$l</li>").to_string();

    output
}