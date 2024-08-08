use std::{
    io::{Read, Write},
    mem::{transmute, transmute_copy},
};

//serial file table format. plain binary file containing an array of structures
//This is rust being coded by a mainly C developer lol. forgive my language
pub struct FileTableEntry {
    pub name: [u8; 256],      //we need fixed size data to make our life easier
    pub full_path: [u8; 512], //bigger size. you never know
    pub is_dir: bool,
}

pub fn load_table_from_file(path: String) -> Vec<FileTableEntry> {
    let mut file_table = Vec::new();
    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}", e),
    };

    let mut bc = 0;
    let mut name = [0; 256];
    let mut full_path = [0; 512];
    let mut is_dir = false;
    file.bytes().for_each(|byte| {
        //new entry. reset vars
        if bc == 0 {
            //push curr to filetable
            file_table.push(FileTableEntry {
                name,
                full_path,
                is_dir,
            });
            //reset vars
            name = [0; 256];
            full_path = [0; 512];
            is_dir = false;
        }

        if bc < 256 {
            name[bc] = byte.unwrap(); //byte to name
        } else if bc < 768 {
            full_path[bc - 256] = byte.unwrap(); //byte to full_path
        } else {
            is_dir = byte.unwrap() == 1; //last byte to is_dir
            bc = 0;
            return;
        }
        bc += 1;
    });
    return file_table;
}

pub fn save_table_to_file(path: String, file_table: Vec<FileTableEntry>) {
    let mut file = match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to create file: {}", e),
    };

    for entry in file_table {
        file.write_all(&entry.name).unwrap();
        file.write_all(&entry.full_path).unwrap();
        file.write_all(&[if entry.is_dir { 1 } else { 0 }]).unwrap();
    }
}
