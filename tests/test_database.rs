#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;

    use hex;
    use sqlite_database_file_dissect::components::database_header::DatabaseHeader;
    use sqlite_database_file_dissect::components::page::Page;
    use sqlite_database_file_dissect::components::database_header::SQLITE_DB_HEADER_STRING;
    use sqlite_database_file_dissect::utils::convert::TryFromBytes;

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

    #[test]
    fn test_database_page1(){
        let mut f = File::open("Chinbook.db.4.analyze.1").unwrap();
        let mut buffer: [u8; 4096] = [0; 4096];
        // read the whole file
        let _r = f.read(&mut buffer);

        let database_header = DatabaseHeader::try_from_be_bytes(&buffer[0..100]).unwrap();
        println!("{:?}", database_header);
        let page = Page::try_from_be_bytes(&buffer, Some(100)).unwrap();
        println!("{:?}", page);
        assert!(false);

    }
}
