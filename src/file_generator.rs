use std::{env::{self, current_dir}, fs::{self, ReadDir}, path::PathBuf};

use crate::parse_markdown;

pub fn generate(posts: ReadDir) {
    fs::create_dir("./site/posts/").expect("Couldn't create posts directory!");
    println!("{:?}", current_dir().unwrap());

    for post in posts {
        env::set_current_dir("./site/posts").expect("Couldn't move into posts dir!");

        if current_dir().unwrap() == PathBuf::from("./site/posts") {
            println!("{:?}", current_dir().unwrap());
        }

        /* let path = post.expect("Couldn't get file path!").path();
        println!("{:?}", &path);
        let file = fs::read_to_string(&path).expect("Couldn't read markdown file!");
        let html = parse_markdown(file);

        let filename = path.file_stem().unwrap();
        let output_file = [filename.to_str().unwrap(), ".html"].concat();
        
        fs::create_dir("./site/posts/").expect("Couldn't create posts directory!");
        env::set_current_dir("./site/posts/").expect("Couldn't move into dir!");
        fs::write(output_file, html).expect("Couldn't write file!");
        */
    }

}