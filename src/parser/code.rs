pub fn parse(text: String) -> String {
    let text = text.replace("```", "");
    let output = format!("<pre>\n<code>{}</code>\n</pre>", text);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_parsing() {
        let text: String = "\
```
let code = \"possible\";
```".to_string();
        let code = "<pre>\n<code>\nlet code = \"possible\";\n</code>\n</pre>";

        assert_eq!(code, parse(text));
    }

    #[test]
    fn test_multiline_parsing() {
        let text: String = "\
```
let num = 4;

for i in 0..num {
    println!(\"Hello, world!\");
}
```".to_string();
        let code = "<pre>\n<code>\nlet num = 4;\n\nfor i in 0..num {\n    println!(\"Hello, world!\");\n}\n</code>\n</pre>";

        assert_eq!(code, parse(text));
    }
}