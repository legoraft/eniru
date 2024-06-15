use std::{env, fs, io::ErrorKind, path::Path};

use eniru::file_generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    env::set_current_dir(&args[1]).expect("Couldn't move into specified directory!");
    
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    let posts_template = fs::read_to_string("./template/posts/[id].html").expect("Post template doesn't exist.");
    
    if !Path::new("./site").exists() {
        fs::create_dir("./site").expect("Couldn't create site directory!");
    }

    post_generator::generate(posts, posts_template);
}
