

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_file(path: &str) -> std::io::Result<String> {
    let file = File::open(path)?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
    
}

pub fn split_by<'a>(contents: &'a String, delimeter: &str) -> Vec<&'a str> {
    contents.split(delimeter).collect::<Vec<&str>>()
}
