use crate::mast;
use crate::read;

pub struct ho_archive{
    pub platform: String,
    pub username: String,
    pub game: String,
    pub program: String,
    pub path: String,
    pub mast: mast::mast,
}


pub fn get_mast(path: String) -> mast::mast{
    let mut offset: u32 = read::read_u32(&path, 0x83C) * 0x800;
    let mut length: u32 = read::read_u32(&path, 0x840);
    let mut section_amount: u32 = read::read_u32(&path, (offset+0x04) as usize);
    
    let mut mast = mast::mast{offset: offset, length: length, section_amount: section_amount, sections: mast::get_sections(&path, offset, section_amount)};
    // println!("---Mast info: {} {} {}", offset, length, section_amount);
    return mast;
}
