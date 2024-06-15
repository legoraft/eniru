use std::{env::{self, current_dir, set_current_dir}, fs::{self, ReadDir}, path::PathBuf};

use crate::parse_markdown;

pub fn generate(posts: ReadDir) {
    fs::create_dir("./site/posts/").expect("Couldn't create posts directory!");
    let working_dir = current_dir().unwrap();
    let site_posts_dir = [working_dir.to_str().unwrap(), "/site/posts"].concat();

    for post in posts {
        if current_dir().unwrap() == PathBuf::from(&site_posts_dir) {
            set_current_dir(&working_dir).expect("Couldn't move to working directory");
        }

        let path = post.expect("Couldn't get file path!").path();
        let file = fs::read_to_string(&path).expect("Couldn't read markdown file!");
        let html = parse_markdown(file);

        let filename = path.file_stem().unwrap();
        let output_file = [filename.to_str().unwrap(), ".html"].concat();
        
        set_current_dir(&site_posts_dir).expect("Couldn't move into posts dir!");
        fs::write(output_file, html).expect("Couldn't write file!");
    }

}