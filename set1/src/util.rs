use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn file_to_string_strip_newlines(filename: &str) -> String {
    let file = File::open(filename).expect("Problem opening file!");
    let reader = BufReader::new(file);
    let mut ret = String::new();
    for line in reader.lines() {
        ret.push_str(&line.expect("Problem reading line!"));
    }
    ret
}