#[cfg(test)]
mod tests {

    use hex;
    use sqlite_database_file_dissect::components::page_header::PageHeader;
    use sqlite_database_file_dissect::components::page_header::PageType;
    use std::convert::TryFrom;

    #[test]
    fn test_interior_table_page_header() {
        const BINARY_AS_STR_12 : &str = "05000000020FF4000000001E";
        let mut page_header_12: [u8; 12] = [0; 12];
        let _ = hex::decode_to_slice(BINARY_AS_STR_12, &mut page_header_12).unwrap();
        let page_type = PageHeader::detect_page_type(page_header_12[0]);
        assert!(page_type == PageType::InteriorTableBtreePage);
        let page_header = PageHeader::try_from(&page_header_12).unwrap();
        println!("{:?}", page_header);
        assert!(page_header.first_free_block_offset == 0);
        assert!(page_header.cell_number == 2);
        assert!(page_header.cell_content_area_offset == 4084);
        assert!(page_header.fragmented_free_bytes == 0);
        assert!(page_header.right_most_pointer == 30);
    }

    #[test]
    fn test_leaf_index_page_header() {
        const BINARY_AS_STR_8  : &str = "0A0A380171034000";
        let mut page_header_8: [u8; 8] = [0; 8];
        let _ = hex::decode_to_slice(BINARY_AS_STR_8, &mut page_header_8).unwrap();
        let page_type = PageHeader::detect_page_type(page_header_8[0]);
        println!("{:?}", page_type);
        assert!(page_type == PageType::LeafIndexBtreePage);
        let page_header = PageHeader::try_from(&page_header_8).unwrap();
        println!("{:?}", page_header);
        assert!(page_header.first_free_block_offset == 2616);
        assert!(page_header.cell_number == 369);
        assert!(page_header.cell_content_area_offset == 832);
        assert!(page_header.fragmented_free_bytes == 0);
        assert!(page_header.right_most_pointer == 0);
    }

}
