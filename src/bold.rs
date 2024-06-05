use regex::Regex;

fn parse(paragraph: String) {
    let bold_re = Regex::new(r"\*\*(?<s>.*)\*\*").unwrap();
    bold_re.replace_all(&paragraph, "<b>$s</b>");
}

