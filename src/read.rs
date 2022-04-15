use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;

pub fn read_u32(filename: &String, offset: usize) -> u32{
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buffer = [0; 4];
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    file.read(&mut buffer[..]).unwrap() as u32;
    
    return u32::from_be_bytes(buffer);
}

pub fn read_u64(filename: &String, offset: usize) -> u64{
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buffer = [0; 8];
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    file.read(&mut buffer[..]).unwrap() as u64;

    return u64::from_be_bytes(buffer);
}

pub fn read_array(filename: &String, offset: usize, length: usize) -> Vec<u8>{
    let file = std::fs::File::open(filename).unwrap();
    let mut file = BufReader::new(file);
    let mut buffer = vec!(0; length);
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    file.read(&mut buffer[..length]).unwrap() as u8;

    return buffer;
}