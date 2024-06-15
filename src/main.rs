use std::{env, ffi::OsString, fs, path::Path};

use eniru::parse_markdown;

fn main() {
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    fs::create_dir("./site/").expect("Couldn't initialize site!");

    for post in posts {
        let path = post.expect("Couldn't get file path!").path();
        println!("{:?}", &path);
        let file = fs::read_to_string(&path).expect("Couldn't read markdown file!");
        let html = parse_markdown(file);

        let filename = path.file_stem().unwrap();
        let output_file = [filename.to_str().unwrap(), ".html"].concat();
        
        fs::create_dir("./site/posts/").expect("Couldn't create posts directory!");
        env::set_current_dir("./site/posts/").expect("Couldn't move into dir!");
        fs::write(output_file, html).expect("Couldn't write file!");
    }
}
