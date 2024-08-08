use std::{
    io::Read,
    mem::{transmute, transmute_copy},
};

//serial file table format. plain binary file containing an array of structures
//This is rust being coded by a mainly C developer lol. forgive my language
pub struct SerialFileTable {
    pub file_table: Vec<FileTableEntry>, //heap allocated array of file table entries
}

pub struct FileTableEntry {
    pub name: [u8; 256],      //we need fixed size data to make our life easier
    pub full_path: [u8; 512], //bigger size. you never know
    pub is_dir: bool,
}

pub fn load_table_from_file(path: String) -> SerialFileTable {
    let mut file_table = Vec::new();
    let file_table_entry_size = std::mem::size_of::<FileTableEntry>();
    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}", e),
    };

    //Map file bytes into struct one by one
    let accumulator = Vec::new();
    file.bytes().for_each(|byte| {
        accumulator.push(byte);
        if accumulator.len() == file_table_entry_size {
            let mut entry = FileTableEntry {
                name: unsafe {
                    let mut name = [0; 256];
                    name.copy_from_slice(&accumulator[0..256]);
                    name
                },
                full_path: unsafe {
                    let mut full_path = [0; 512];
                    full_path.copy_from_slice(&accumulator[256..768]);
                    full_path
                },
                is_dir: false,
            };
            file_table.push(entry);
            accumulator.clear() //nuke acc. start over
        }
    });
    return file_table;
}

pub fn save_table_to_file(path: String) {}
