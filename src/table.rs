use crate::asset;
use crate::read;

pub struct table{
    pub data_offset: u32,
    pub data_length: u32,
    pub dir_offset: u32,
    pub dir_length: u32,
    pub assets: Vec<asset::asset>,
}

pub fn get_assets(path: &String, dir_offset: u32, data_offset: u32) -> Vec<asset::asset>{
    let mut assets: Vec<asset::asset> = Vec::new();
    let mut pointer = dir_offset as usize;
    let mut total_offset = data_offset;
    let asset_amount = read::read_u32(path, pointer);
    
    for _a in 0..asset_amount{
        pointer += 0x20;
        let mut length_with_padding = read::read_u32(&path, pointer);
        let mut offset = total_offset;
        let mut length = read::read_u32(&path, pointer+0x08);
        let mid_bytes = read::read_u32(&path, pointer+0x0C);
        let id = read::read_u64(&path, pointer+0x10);
        let flag = read::read_u32(&path, pointer+0x18);
        let trail_bytes = read::read_u32(&path, pointer+0x1C);
        
        let mut data = read::read_array(&path, total_offset as usize, length as usize);

        total_offset += length_with_padding;

        // println!("---------------Asset Nr.{} info: {} {} {} {} {} {}", a, length_with_padding, offset, length, id, flag, pointer);
        assets.push(asset::asset{length_with_padding: length_with_padding, offset: offset, length: length, id: id, flag: flag, data: data, name: String::new(), mid_bytes: mid_bytes, trail_bytes: trail_bytes});
    }
    return assets;
}