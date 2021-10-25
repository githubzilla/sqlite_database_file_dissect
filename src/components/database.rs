use crate::components::file_manager::FileManager;
use crate::components::database_header::DatabaseHeader;

pub struct Database {
    fm: FileManager,
    header: DatabaseHeader,
    //freelist: Vec<()>,
    //pages: Vec<()>, 
}



pub struct CellPointer {
    pointer: u16,
}

pub struct Cell {
    left_child_page_number: u32,
    payload_length: u64,
    rowid: u64,
    payload: Vec<u8>,
    overflow_page_number: u32,


}

/**
* implement of Database
*/

impl Database {
    //pub fn new(file_manager: &FileManager) -> Self{

        //let mut db: Database = Database { fm: file_manager, header: (), freelist: (), pages: () };

        //let database_header_bytes = file_manager.read(0, 100);

    //}
}

