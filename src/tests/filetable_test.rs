mod filetable;
mod tree;

//move this to main for testing. didnt configure testing setup in cargo
fn main() {
    let mut table = Vec::new();
    {
        let mut f1 = filetable::FileTableEntry {
            name: [0; 128],
            full_path: [0; 256],
            is_dir: false,
        };
        //first name declare
        let name = b"testing it all.txt";
        f1.name[..name.len()].copy_from_slice(name);
        //full path declare
        let path = b"/home/user/documents/testing it all.txt";
        f1.full_path[..path.len()].copy_from_slice(path);
        table.push(f1);
    }
    {
        let mut f1 = filetable::FileTableEntry {
            name: [0; 128],
            full_path: [0; 256],
            is_dir: true,
        };
        //first name declare
        let name = b"this is a second test!!!!";
        f1.name[..name.len()].copy_from_slice(name);
        //full path declare
        let path = b"/home/user/documents/this is a second test!!!!";
        f1.full_path[..path.len()].copy_from_slice(path);
        table.push(f1);
    }

    filetable::save_table_to_file("./testing_table.bin", table)
}
