#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;

    use hex;

    use sqlite_database_file_dissect::components::database_header::DatabaseHeader;
    use sqlite_database_file_dissect::components::page_header::PageHeader;
    use sqlite_database_file_dissect::components::page_header::PageType;
    use sqlite_database_file_dissect::components::cell_pointer::CellPointer;
    use sqlite_database_file_dissect::components::cell::Cell;
    use sqlite_database_file_dissect::components::page::Page;
    use sqlite_database_file_dissect::utils::convert::TryFromBytes;

    #[test]
    fn test_interior_table_page_header() {
        const BINARY_AS_STR_12 : &str = "05000000020FF4000000001E";
        let mut page_header_12: [u8; 12] = [0; 12];
        let _ = hex::decode_to_slice(BINARY_AS_STR_12, &mut page_header_12).unwrap();
        let page_type = PageHeader::detect_page_type(page_header_12[0]);
        assert!(page_type == PageType::TableInteriorBtreePage);
        let page_header = PageHeader::try_from_be_bytes(&page_header_12).unwrap();
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
        assert!(page_type == PageType::IndexLeafBtreePage);
        let page_header = PageHeader::try_from_be_bytes(&page_header_8).unwrap();
        println!("{:?}", page_header);
        assert!(page_header.first_free_block_offset == 2616);
        assert!(page_header.cell_number == 369);
        assert!(page_header.cell_content_area_offset == 832);
        assert!(page_header.fragmented_free_bytes == 0);
        assert!(page_header.right_most_pointer == 0);
    }

    #[test]
    fn test_cell_pointer(){
        const BINARY_AS_STR_738:&str="0D680D600D580D500D480D400D380D300D280D200D180D100D080D000CF80CF00CE80CE00CD80CD00CC80CC00CB80CB00CA80CA00C980C900C880C800C780C700C680C600C580C500C480C400C380C300C280C200C180C100C080C000BF80BF00BE80BE00BD8056006B806B0055806A806A005500698069005480688068005400678067005380668066005300658065005280648064005200638063005180628062005100618061005080608060005000998099005F00988098005E80978097005E00968096005D80958095005D00948094005C80938093005C00928092005B80918091005B00908090005A808F808F005A008E808E0059808D808D0059008C808C0058808B808B0058008A808A00578089804F8089004F0088804E8088004E0087804D8087004D0086804C8086004C0085804B8085004B0084804A8084004A0083804980830049008280488082004800818041008100408080804780800040007F8047007F0046807E803F807E003F007D8046007D0045807C803E807C003E007B8045007B0044807A803D807A00440079803D0079003C80788043807800430077803C0077003B807680428076003B00758042007500418074803A8074003A0073803980730039007280388072003800718037807100370070803680700036006F8035806F003500FF805F80FF006E00FE809C00FE009A80FD809A00FD006D80FC803480FC003400FB806E80FB009B80FA809B00FA005700F9809E00F9009D80F8809D00F8009C80F7806D00F7009F80F6809F00F6009E80F5805680F500A080F480A000F4006C80F380A180F300A100F2806C00F200A200F180A280F100A300F080BD00F000BC80EF80BC00EF00BB80EE80BB00EE00BA80ED80BA00ED00B980EC80B900EC00B880EB80B800EB00B780EA80B700EA00B680E980B600E900E880B580AB00E800E780B500E700E680B480E600E580B400E500E480B380E400E380B300E300B280B200B180B100B080B000AF80AF00AE80AE00AD80AD00AC80AC00AB8";
        let mut cell_pointers_738: [u8; 738] = [0; 738];
        let _ = hex::decode_to_slice(BINARY_AS_STR_738, &mut cell_pointers_738).unwrap();
        let cell_pointers = <Vec<CellPointer>>::try_from_be_bytes(&cell_pointers_738).unwrap();
        println!("{:?}", cell_pointers);
        assert_eq!(cell_pointers.len(), 369);
        //assert!(false);
    }

    #[test]
    fn test_cell(){
        const BINARY_AS_STR_8: &str="070302020714062C";
        let mut cell_8: [u8; 8] =[0; 8];
        let _ = hex::decode_to_slice(BINARY_AS_STR_8, &mut cell_8);
        let cell = Cell::try_from_bytes(&cell_8, PageType::IndexLeafBtreePage, 4096);
        println!("{:?}", cell);
        assert!(false);
    }

    #[test]
    fn test_page14(){
        let mut f = File::open("test-data/Chinbook.db.4.analyze.14").unwrap();
        let mut buffer: [u8; 4096] = [0; 4096];
        // read the whole file
        let _r = f.read(&mut buffer);

        let page = Page::try_from_be_bytes(&buffer, None).unwrap();
        println!("{:?}", page);

        assert!(false);
    }
    
    #[test]
    fn test_page15(){
        let mut f = File::open("test-data/Chinbook.db.4.analyze.15").unwrap();
        let mut buffer: [u8; 4096] = [0; 4096];
        // read the whole file
        let _r = f.read(&mut buffer);

        let page = Page::try_from_be_bytes(&buffer, None).unwrap();
        println!("{:?}", page);

        assert!(false);
    }
    #[test]
    fn test_page2(){
        let mut f = File::open("test-data/Chinbook.db.4.analyze.2").unwrap();
        let mut buffer: [u8; 4096] = [0; 4096];
        // read the whole file
        let _r = f.read(&mut buffer);

        let page = Page::try_from_be_bytes(&buffer, None).unwrap();
        println!("{:?}", page);

        assert!(false);
    }
    
    #[test]
    fn test_page1(){
        let mut f = File::open("test-data/Chinbook.db.4.analyze.1").unwrap();
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
