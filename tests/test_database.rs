#[cfg(test)]
mod tests {

    use hex;
    use sqlite_database_file_dissect::components::database_header::DatabaseHeader;
    use std::convert::TryFrom;

    #[test]
    fn test_database_header() {
        const BINARY_AS_STR : &str = "53514C69746520666F726D6174203300100001010040202000003D0C000000E000000000000000000000001500000004000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000003D0C002E5748";
        let mut database_header: [u8; 100] = [0; 100];
        let _ = hex::decode_to_slice(BINARY_AS_STR, &mut database_header);
        let _r = DatabaseHeader::try_from(&database_header).unwrap();
        println!("{:?}",_r );
        assert!(false);


    }
}
