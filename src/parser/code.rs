pub fn parse(text: String) -> String {
    let text = text.replace("```", "");
    let output = format!("<pre>\n<code>\n{}\n</code>\n</pre>", text);

    output
}