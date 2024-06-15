use std::{env, fs};

use eniru::file_generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = env::set_current_dir(&args[1])
        .expect("Couldn't move into specified directory!");
    
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    fs::create_dir("./site/").expect("Couldn't initialize site!");

    file_generator::generate(posts);
}
