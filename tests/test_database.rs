#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::io::SeekFrom;
    use std::fs::File;
    use std::convert::TryInto;

    use hex;
    use sqlite_database_file_dissect::components::database_header::DatabaseHeader;
    use sqlite_database_file_dissect::components::database_header::SQLITE_DB_HEADER_STRING;
    use sqlite_database_file_dissect::utils::convert::TryFromBytes;
    use sqlite_database_file_dissect::components::page::Page;

    #[test]
    fn test_database_header() {
        const BINARY_AS_STR : &str = "53514C69746520666F726D6174203300100001010040202000003D0C000000E000000000000000000000001500000004000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000003D0C002E5748";
        let mut database_header: [u8; 100] = [0; 100];
        let _ = hex::decode_to_slice(BINARY_AS_STR, &mut database_header);
        let database_header = DatabaseHeader::try_from_be_bytes(&database_header).unwrap();
        println!("{:?}", database_header );
        assert_eq!(database_header.header_string, SQLITE_DB_HEADER_STRING);
        assert_eq!(database_header.page_size, 4096);
        assert_eq!(database_header.in_header_database_size, 224);
        assert_eq!(database_header.sqlite_version_number, 3037000);
    }

    fn travel_btree_page(f: &mut File, 
                         page_size: usize, 
                         page_index: usize, 
                         buffer: &mut [u8], 
                         page_travel_vec: &mut Vec<bool>, 
                         btree_id: usize, 
                         btree_level: usize) {
        page_travel_vec[page_index] = true;
        println!("page: {}, at level: {}", btree_id, btree_level);

        let page_start_offset: u64 = (page_index * page_size).try_into().unwrap();
        f.seek(SeekFrom::Start(page_start_offset)).unwrap();
        let _r = f.read(buffer);
        let page_start_offset;

        if page_index == 0 {
            let database_header = DatabaseHeader::try_from_be_bytes(&buffer[0..100]).unwrap();
            println!("DatabaseHeader {:?}", database_header);
            page_start_offset = 100;
        } else {
            page_start_offset = 0;
        } 
        
        let page = Page::try_from_be_bytes(&buffer, Some(page_start_offset)).unwrap();
        println!("page.header.page_type: {:?}", page.header.page_type);

        for cell_idx in 0..page.cells.len() {
            let cell = &page.cells[cell_idx];
            print!("cell.left_child_page_number: {}, cell.row_id: {} | ", cell.left_child_page_number.unwrap_or(0), cell.row_id.unwrap_or(0));
        }
        print!("\n");

        println!("page.header.right_most_pointer: {:?}", page.header.right_most_pointer);

        for cell_idx in 0..page.cells.len() {
            let cell = &page.cells[cell_idx];
            match cell.left_child_page_number {
                Some(left_child_page_number) => travel_btree_page(f, 
                                                    page_size, 
                                                    (left_child_page_number -1).try_into().unwrap(), 
                                                    buffer, 
                                                    page_travel_vec, 
                                                    btree_id, 
                                                    btree_level +1),
                None => (),
            }
        }

        if page.header.right_most_pointer != 0 {
            travel_btree_page(f, 
                page_size, 
                (page.header.right_most_pointer -1).try_into().unwrap(), 
                buffer, 
                page_travel_vec, 
                btree_id, 
                btree_level +1);
        }
    }

    #[test]
    fn test_database_page_structure() {
        let mut f = File::open("test-data/Chinook.db.4.analyze").unwrap();
        const PAGE_SIZE: usize = 4096;
        let mut buffer: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
        let f_length: usize = f.metadata().unwrap().len().try_into().unwrap();
        let page_num: usize = f_length/PAGE_SIZE;
        let mut page_travel_vec: Vec<bool>  = Vec::with_capacity(page_num);

        for _ in 0..page_num {
            page_travel_vec.push(false);
        }

        for page_index in 0..page_travel_vec.len() {
            let is_traveled = page_travel_vec[page_index];
            if is_traveled {
                continue;
            }
            // read the page
            println!("Start btree {}: -----------------------------", page_index);
            travel_btree_page(
                &mut f, 
                PAGE_SIZE, 
                page_index, 
                &mut buffer, 
                &mut page_travel_vec, 
                page_index,
                0);
            println!("End btree {}: -----------------------------", page_index);
        }

        assert!(false);


    }
}
