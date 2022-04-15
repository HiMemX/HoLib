mod ho_archive;
mod mast;
mod section;
mod table;
mod asset;
mod read;
mod write;
mod math;


pub fn create_archive(path: String) -> ho_archive::ho_archive{
    let mut platform_byte = read::read_array(&path, 0x400 as usize, 0x10);
    let mut username_byte = read::read_array(&path, 0x43C as usize, 0x40);
    let mut game_byte = read::read_array(&path, 0x47C as usize, 0x40);
    let mut program_byte = read::read_array(&path, 0x4BC as usize, 0x40);

    platform_byte.retain(|&x| x != 0);
    username_byte.retain(|&x| x != 0);
    game_byte.retain(|&x| x != 0);
    program_byte.retain(|&x| x != 0);

    let mut platform = String::from_utf8(platform_byte).unwrap();
    let mut username = String::from_utf8(username_byte).unwrap();
    let mut game = String::from_utf8(game_byte).unwrap();
    let mut program = String::from_utf8(program_byte).unwrap();

    return ho_archive::ho_archive{platform: platform, username: username, game: game, program: program, path: path.clone(), mast: ho_archive::get_mast(path)};
}

pub fn save_archive(archive: ho_archive::ho_archive){
    let mut to_write: Vec<u8> = Vec::new();

    let mut pointer = 0x400;
    to_write.extend(read::read_array(&archive.path, 0, pointer));
    pointer = 0x43C;

    let platform = archive.platform.as_bytes();
    let username = archive.username.as_bytes();
    let game = archive.game.as_bytes();
    let program = archive.program.as_bytes();

    for i in 0..platform.len(){
        to_write.push(0);
        to_write.push(platform[i]);
    }
    to_write.extend(vec![0; pointer-to_write.len()]);

    pointer += 0x40;
    for i in 0..username.len(){
        to_write.push(0);
        to_write.push(username[i]);
    }
    to_write.extend(vec![0; pointer-to_write.len()]);

    pointer += 0x40;
    for i in 0..game.len(){
        to_write.push(0);
        to_write.push(game[i]);
    }
    to_write.extend(vec![0; pointer-to_write.len()]);

    pointer += 0x40;
    for i in 0..program.len(){
        to_write.push(0);
        to_write.push(program[i]);
    }
    to_write.extend(vec![0; pointer-to_write.len()]);

    to_write.extend(read::read_array(&archive.path, pointer, (archive.mast.offset as usize + 0x20)-pointer));
    pointer = archive.mast.offset as usize + 0x20;

    for sect in archive.mast.sections.iter(){
        to_write.extend(read::read_array(&archive.path, pointer, 0x1C)); pointer += 0x1C;
        to_write.extend((sect.offset / 0x800).to_be_bytes()); pointer += 0x04;
        to_write.extend(sect.length.to_be_bytes()); pointer += 0x04;
        to_write.extend(sect.length.to_be_bytes()); pointer += 0x04;
        to_write.extend(read::read_array(&archive.path, pointer, 0x18)); pointer += 0x18;
    }
    for sect in archive.mast.sections.iter(){
        to_write.extend(read::read_array(&archive.path, pointer, (sect.section_table_offset-pointer as u32) as usize)); pointer = sect.section_table_offset as usize;
        
        if sect.is_name_container{
            to_write.extend(read::read_array(&archive.path, pointer, 0x20)); pointer += 0x20;
            continue;
        }

        if sect.tables[0].data_offset < sect.tables[0].dir_offset{
            to_write.extend(read::read_array(&archive.path, pointer, 0x10)); pointer += 0x10;
            let mut total_length: u32 = 0;
            for table in sect.tables.iter(){
                to_write.extend(read::read_array(&archive.path, pointer, 0x18)); pointer += 0x18;
                to_write.extend(table.data_length.to_be_bytes()); pointer += 0x04;
                to_write.extend(read::read_array(&archive.path, pointer, 0x04)); pointer += 0x04;
                total_length += table.data_length;
            }
            to_write.extend(read::read_array(&archive.path, pointer, 0x08)); pointer += 0x08;
            to_write.extend((0x800 - (total_length % 0x800)).to_be_bytes()); pointer += 0x04;
            to_write.extend(read::read_array(&archive.path, pointer, 0x04)); pointer += 0x04;
            for table in sect.tables.iter(){
                to_write.extend(read::read_array(&archive.path, pointer, 0x08)); pointer += 0x08;
                to_write.extend(table.dir_length.to_be_bytes()); pointer += 0x04;
                to_write.extend(read::read_array(&archive.path, pointer, 0x04)); pointer += 0x04;
            }
        }
        else{
            to_write.extend(read::read_array(&archive.path, pointer, 0x10)); pointer += 0x10;
            let mut total_length: u32 = 0;
            for table in sect.tables.iter(){
                to_write.extend(read::read_array(&archive.path, pointer, 0x08)); pointer += 0x08;
                to_write.extend(table.dir_offset.to_be_bytes()); pointer += 0x04;
                to_write.extend(read::read_array(&archive.path, pointer, 0x04)); pointer += 0x04;
                total_length += table.dir_length;
            }
            to_write.extend(read::read_array(&archive.path, pointer, 0x08)); pointer += 0x08;
            to_write.extend(total_length.to_be_bytes()); pointer += 0x04;
            to_write.extend(read::read_array(&archive.path, pointer, 0x04)); pointer += 0x04;
            for table in sect.tables.iter(){
                to_write.extend(read::read_array(&archive.path, pointer, 0x18)); pointer += 0x18;
                to_write.extend(table.data_offset.to_be_bytes()); pointer += 0x04;
                to_write.extend(read::read_array(&archive.path, pointer, 0x04)); pointer += 0x04;
            }
        }
    }
    for sect in archive.mast.sections.iter(){
        if sect.is_name_container{
            to_write.extend(vec![0x33; sect.offset as usize-to_write.len()]);
            to_write.extend(sect.data.clone());
            continue;
        }

        if sect.tables[0].data_offset < sect.tables[0].dir_offset{
            for table in sect.tables.iter(){
                to_write.extend(vec![0x33; table.data_offset as usize-to_write.len()]);
                for asset in table.assets.iter(){
                    to_write.extend(asset.data.clone());
                    to_write.extend(vec![0x33; (asset.length_with_padding-asset.length) as usize]);
                }
            }
            for table in sect.tables.iter(){
                to_write.extend(vec![0x33; table.dir_offset as usize-to_write.len()]);
                to_write.extend((table.assets.len() as u32).to_be_bytes());
                to_write.extend([0xFF, 0xFF, 0xFF, 0xFF, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74]);
                for asset in table.assets.iter(){
                    to_write.extend(asset.length_with_padding.to_be_bytes());
                    to_write.extend(asset.offset.to_be_bytes());
                    to_write.extend(asset.length.to_be_bytes());
                    to_write.extend(asset.mid_bytes.to_be_bytes());
                    to_write.extend(asset.id.to_be_bytes());
                    to_write.extend(asset.flag.to_be_bytes());
                    to_write.extend(asset.trail_bytes.to_be_bytes());
                }
            }
        }
        else{
            for table in sect.tables.iter(){
                to_write.extend(vec![0x33; table.dir_offset as usize-to_write.len()]);
                to_write.extend((table.assets.len() as u32).to_be_bytes());
                to_write.extend([0xFF, 0xFF, 0xFF, 0xFF, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74]);
                for asset in table.assets.iter(){
                    to_write.extend(asset.length_with_padding.to_be_bytes());
                    to_write.extend(asset.offset.to_be_bytes());
                    to_write.extend(asset.length.to_be_bytes());
                    to_write.extend(asset.mid_bytes.to_be_bytes());
                    to_write.extend(asset.id.to_be_bytes());
                    to_write.extend(asset.flag.to_be_bytes());
                    to_write.extend(asset.trail_bytes.to_be_bytes());
                }
            }
            for table in sect.tables.iter(){
                to_write.extend(vec![0x33; table.data_offset as usize-to_write.len()]);
                for asset in table.assets.iter(){
                    to_write.extend(asset.data.clone());
                    to_write.extend(vec![0x33; (asset.length_with_padding-asset.length) as usize]);
                }
            }
        }
    }
    to_write.extend(vec![0x33; math::round_up_to(to_write.len(), 0x800) as usize-to_write.len()]);
    write::write(&archive.path, to_write);
}

pub fn update_archive(archive: ho_archive::ho_archive) -> ho_archive::ho_archive{
    let mut archive = archive;

    if archive.platform.len() > 0x1E{archive.platform = archive.platform[..0x1E].to_string();}
    if archive.username.len() > 0x20{archive.username = archive.username[..0x20].to_string();}
    if archive.game.len() > 0x20 {archive.game = archive.game[..0x20].to_string();}
    if archive.program.len() > 0x20{archive.program = archive.program[..0x20].to_string();}
    
    for sect in 0..archive.mast.sections.len(){
        if sect > 0{archive.mast.sections[sect].offset = archive.mast.sections[sect-1].offset + math::round_up_to(archive.mast.sections[sect-1].length as usize, 0x800) as u32;}
        if archive.mast.sections[sect].is_name_container{continue;}
        let mut table_total_offset = archive.mast.sections[sect].offset;
        let mut table_total_length = 0;
        // This if/else is needed because some ho files store the data before declaration, others after
        if archive.mast.sections[sect].tables[0].data_offset < archive.mast.sections[sect].tables[0].dir_offset{
            for table in 0..archive.mast.sections[sect].tables.len(){
                let mut total_offset = archive.mast.sections[sect].tables[table].data_offset;
                let mut total_length = 0;
                for asset in 0..archive.mast.sections[sect].tables[table].assets.len(){
                    archive.mast.sections[sect].tables[table].assets[asset].length = archive.mast.sections[sect].tables[table].assets[asset].data.len() as u32;
                    archive.mast.sections[sect].tables[table].assets[asset].length_with_padding = math::round_up_to(archive.mast.sections[sect].tables[table].assets[asset].length as usize, 0x40) as u32;
                    archive.mast.sections[sect].tables[table].assets[asset].offset = total_offset;
                    total_offset += archive.mast.sections[sect].tables[table].assets[asset].length_with_padding;
                    total_length += archive.mast.sections[sect].tables[table].assets[asset].length_with_padding;
                }
                archive.mast.sections[sect].tables[table].data_length = total_length;
                archive.mast.sections[sect].tables[table].data_offset = table_total_offset;
                table_total_offset += total_length;
                table_total_length += total_length;
            }
            table_total_length = math::round_up_to(table_total_length as usize, 0x800) as u32;
            table_total_offset = math::round_up_to(table_total_offset as usize, 0x800) as u32;
            for table in 0..archive.mast.sections[sect].tables.len(){
                archive.mast.sections[sect].tables[table].dir_offset = table_total_offset;
                archive.mast.sections[sect].tables[table].dir_length = math::round_up_to((archive.mast.sections[sect].tables[table].assets.len() * 0x20 + 0x20) as usize, 0x40) as u32;
                table_total_length += archive.mast.sections[sect].tables[table].dir_length;
                table_total_offset += archive.mast.sections[sect].tables[table].dir_length;
            }
        }
        else{
            for table in 0..archive.mast.sections[sect].tables.len(){
                archive.mast.sections[sect].tables[table].dir_offset = table_total_offset;
                archive.mast.sections[sect].tables[table].dir_length = math::round_up_to((archive.mast.sections[sect].tables[table].assets.len() * 0x20 + 0x20) as usize, 0x40) as u32;
                table_total_length += archive.mast.sections[sect].tables[table].dir_length;
                table_total_offset += archive.mast.sections[sect].tables[table].dir_length;
            }
            table_total_length = math::round_up_to(table_total_length as usize, 0x800) as u32;
            table_total_offset = math::round_up_to(table_total_offset as usize, 0x800) as u32;
            for table in 0..archive.mast.sections[sect].tables.len(){
                let mut total_offset = archive.mast.sections[sect].tables[table].data_offset;
                let mut total_length = 0;
                for asset in 0..archive.mast.sections[sect].tables[table].assets.len(){
                    archive.mast.sections[sect].tables[table].assets[asset].length = archive.mast.sections[sect].tables[table].assets[asset].data.len() as u32;
                    archive.mast.sections[sect].tables[table].assets[asset].length_with_padding = math::round_up_to(archive.mast.sections[sect].tables[table].assets[asset].length as usize, 0x40) as u32;
                    archive.mast.sections[sect].tables[table].assets[asset].offset = total_offset;
                    total_offset += archive.mast.sections[sect].tables[table].assets[asset].length_with_padding;
                    total_length += archive.mast.sections[sect].tables[table].assets[asset].length_with_padding;
                }
                archive.mast.sections[sect].tables[table].data_length = total_length;
                archive.mast.sections[sect].tables[table].data_offset = table_total_offset;
                table_total_offset += total_length;
                table_total_length += total_length;
            }
        }
        archive.mast.sections[sect].length = table_total_length;
    }

    return archive;
}
