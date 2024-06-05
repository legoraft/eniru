pub fn parse(heading: &str) -> String {
    let (count, text) = heading.split_once(" ").unwrap();

    let header_tag = format!("<h{}>{}</h{}>", count.len(), text, count.len());
    header_tag
}