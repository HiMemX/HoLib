use crate::section;
use crate::read;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};


pub struct mast{
    pub offset: u32,
    pub length: u32,
    pub section_amount: u32,
    pub sections: Vec<section::section>,
}

pub fn get_sections(path: &String, offset: u32, section_amount: u32) -> Vec<section::section>{
    let mut sections: Vec<section::section> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for sect in 0..section_amount{
        let section_name: u32 = read::read_u32(&path, (0x20+offset+sect*0x40) as usize);
        let mut section_offset: u32 = read::read_u32(&path, (0x20+offset+sect*0x40+0x1C) as usize) * 0x800;
        let mut section_length: u32 = read::read_u32(&path, (0x20+offset+sect*0x40+0x20) as usize);
        let mut section_table_offset: u32 = read::read_u32(&path, (0x20+offset+sect*0x40+0x38) as usize);
        let data = read::read_array(&path, section_offset as usize, section_length as usize);

        let mut section = section::section{name: section_name, offset: section_offset, length: section_length, is_name_container: section_name == 1346641952, section_table_offset: offset+section_table_offset, data: data, tables: section::get_tables(path, offset+section_table_offset, section_offset)};

        // println!("------Sect Nr.{} info: {} {} {} {}", sect, section_name, section_offset, section_length, section_table_offset);

        sections.push(section);
    }
    let mut names: Vec<String> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    for sect in sections.iter(){
        if sect.is_name_container == true{
            let mut pointer: u32 = read::read_u32(&path, (sect.section_table_offset+0x0C) as usize) + sect.offset;
            for _names in 0..read::read_u32(&path, (sect.section_table_offset+0x08) as usize){
                ids.push(read::read_u64(&path, pointer as usize));
                let offset = pointer as u64 + 0x20;

                let file = File::open(&path).unwrap();
                let mut file = BufReader::new(file);
                let mut data = Vec::new();
                file.seek(SeekFrom::Start(offset)).unwrap();
                file.read_until(b'\0', &mut data).unwrap();

                if data.last() == Some(&0) {
                    data.pop();
                }

                let name = String::from_utf8(data).unwrap();
                pointer = (((pointer + name.len() as u32 + 0x21) as f64 / 64.0).ceil() * 64.0) as u32;
                names.push(name);
            }
        }
    }
    for sect in 0..sections.len(){
        for table in 0..sections[sect].tables.len(){
            for asset in 0..sections[sect].tables[table].assets.len(){
                for name in 0..names.len(){
                    if sections[sect].tables[table].assets[asset].id == ids[name]{
                        sections[sect].tables[table].assets[asset].name = names[name].clone();
                        names.remove(name);
                        ids.remove(name);
                        break;
                    }
                }
            }
        }
    }
    return sections;
}