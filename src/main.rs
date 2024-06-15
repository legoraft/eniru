use std::{env, ffi::OsString, fs, path::Path};

use eniru::parse_markdown;

fn main() {
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    fs::create_dir("./site/").expect("Couldn't initialize site!");

    file_generator::generate(posts);
}
