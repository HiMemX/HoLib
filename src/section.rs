use crate::table;
use crate::read;

pub struct section{
    pub name: u32,
    pub offset: u32,
    pub length: u32,
    pub is_name_container: bool,
    pub section_table_offset: u32,
    pub data: Vec<u8>,
    pub tables: Vec<table::table>,
}

pub fn get_tables(path: &String, offset: u32, section_offset: u32) -> Vec<table::table>{
    let mut tables: Vec<table::table> = Vec::new();
    let mut dirs: Vec<[u32; 2]> = Vec::new();
    let mut datas: Vec<[u32; 2]> = Vec::new();

    if read::read_u32(&path, offset as usize) == 1347636224{ // 1347636224 is the u32 for "PSL", which are actual table containers. 
        let mut table_amount: u32 = read::read_u32(&path, (offset+0x08) as usize);
        let mut pointer: usize = (offset) as usize;
        let mut total_offset = section_offset;
        let mut curr_length;

        if read::read_u32(&path, pointer+0x10) == 0{
            for _t in 0..(table_amount-1)/3{
                pointer += 0x10;
                curr_length = read::read_u32(&path, pointer+0x08);
                dirs.push([curr_length, total_offset]);
                total_offset += curr_length;
            }
            pointer += 0x10;
            total_offset += read::read_u32(&path, pointer+0x08);

            for _t in 0..(table_amount-1)/3{
                pointer += 0x20;
                curr_length = read::read_u32(&path, pointer+0x08);
                datas.push([curr_length, total_offset]);
                total_offset += curr_length;
            }
        }
        else{
            for _t in 0..(table_amount-1)/3{
                pointer += 0x20;
                curr_length = read::read_u32(&path, pointer+0x08);
                datas.push([curr_length, total_offset]);
                total_offset += curr_length;
            }
            pointer += 0x10;
            total_offset += read::read_u32(&path, pointer+0x08);

            for _t in 0..(table_amount-1)/3{
                pointer += 0x10;
                curr_length = read::read_u32(&path, pointer+0x08);
                dirs.push([curr_length, total_offset]);
                total_offset += curr_length;
            }
        }

        for t in 0..(table_amount-1)/3{
            let mut data_offset = datas[t as usize][1];
            let mut data_length = datas[t as usize][0];
            let mut dir_offset = dirs[t as usize][1];
            let mut dir_length = dirs[t as usize][0];

            // println!("------------Table Nr.{} info: {} {} {} {}", t, data_offset, data_length, dir_offset, dir_length);
            tables.push(table::table{data_offset: data_offset, data_length: data_length, dir_offset: dir_offset, dir_length: dir_length, assets: table::get_assets(&path, dirs[t as usize][1], data_offset)});
        }
    }
    return tables;
}