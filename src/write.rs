use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn write(filename: &String, input: Vec<u8>){
    let mut file = OpenOptions::new().write(true).open(filename).unwrap();

    file.write_all(&input).unwrap();
}
