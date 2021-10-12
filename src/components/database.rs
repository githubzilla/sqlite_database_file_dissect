use std::str;
use std::string::String;
use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use crate::components::file_manager::FileManager;
use crate::utils::error::MyError;
use crate::utils::primitives::TryFromByteSlice;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum FileFormatVersion {
    Legacy = 1,
    WAL = 2,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SchemaFormatNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum TextEncoding {
    UTF8 = 1,
    UTF16le = 2,
    UTF16be = 3,
}

#[derive(Debug)]
pub struct DatabaseHeader {
    header_string: String,
    page_size: u16,
    file_format_read_version: FileFormatVersion,
    file_format_write_version: FileFormatVersion,
    bytes_at_unused_page_end: u8,
    max_embedded_payload_fraction: u8,
    min_embedded_payload_fraction: u8,
    leaf_payload_fraction: u8,
    file_change_count: u32,
    in_header_database_size: u32,
    first_freelist_trunk_page_number: u32,
    total_freelist_page_number: u32,
    schema_cookie: u32,
    schema_format_number: SchemaFormatNumber,
    default_page_cache_size: u32,
    largest_root_btree_page_number: u32,
    text_encoding: TextEncoding,
    user_version: u32,
    incremental_vacuum_mode: u32,
    application_id: u32,
    reserved: [u8; 20],
    version_valid_for_number: u32,
    sqlite_version_number: u32,
}

/**
* implement of DatabaseHeader
*/
const SQLITE_DB_HEADER_STRING: &'static str = "SQLite format 3\0";

impl Default for DatabaseHeader {
   fn default() -> Self { 
        DatabaseHeader {
            header_string: "".into(),
            page_size: 0,
            file_format_read_version: FileFormatVersion::Legacy,
            file_format_write_version: FileFormatVersion::Legacy,
            bytes_at_unused_page_end: 0,
            max_embedded_payload_fraction: 0,
            min_embedded_payload_fraction: 0,
            leaf_payload_fraction: 0,
            file_change_count: 0,
            in_header_database_size: 0,
            first_freelist_trunk_page_number: 0,
            total_freelist_page_number: 0,
            schema_cookie: 0,
            schema_format_number: SchemaFormatNumber::One,
            default_page_cache_size: 0,
            largest_root_btree_page_number: 0,
            text_encoding: TextEncoding::UTF8,
            user_version: 0,
            incremental_vacuum_mode: 0,
            application_id: 0,
            reserved: Default::default(),
            version_valid_for_number: 0,
            sqlite_version_number: 0, 
        } 
    }
}

impl TryFrom<[u8; 100]> for DatabaseHeader {
    type Error= MyError;

    fn try_from(bytes: [u8; 100]) -> Result<Self, MyError> {
        //header_string
        let header_string = String::from_utf8_lossy(&bytes[0..=15]);
        //page_size
        let page_size = u16::try_from_be_byte_slice(&bytes[16..=17]).unwrap();
        //file_format_read_version
        let file_format_read_version: FileFormatVersion = num::FromPrimitive::from_u8(bytes[18]).unwrap();
        //file_format_write_version
        let file_format_write_version: FileFormatVersion = num::FromPrimitive::from_u8(bytes[19]).unwrap();
        //bytes_at_unused_page_end
        let bytes_at_unused_page_end = bytes[20];
        //max_embedded_payload_fraction
        let max_embedded_payload_fraction = bytes[21];
        //min_embedded_payload_fraction
        let min_embedded_payload_fraction = bytes[22];
        //leaf_payload_fraction
        let leaf_payload_fraction = bytes[23];
        //file_change_count
        let file_change_count = u32::try_from_be_byte_slice(&bytes[24..=27]).unwrap();
        //in_header_database_size
        //let in_header_database_size = u32::from_be_bytes(<[u8; 4]>::try_from(&bytes[28..=31]).unwrap());
        let in_header_database_size = u32::try_from_be_byte_slice(&bytes[28..=31]).unwrap();
        //first_freelist_trunk_page_number
        let first_freelist_trunk_page_number = u32::try_from_be_byte_slice(&bytes[32..=35]).unwrap();
        //total_freelist_page_number
        let total_freelist_page_number = u32::try_from_be_byte_slice(&bytes[36..=39]).unwrap();
        //schema_cookie
        let schema_cookie = u32::try_from_be_byte_slice(&bytes[40..=43]).unwrap();
        //schema_format_number
        let schema_format_number: SchemaFormatNumber = num::FromPrimitive::from_u32(u32::try_from_be_byte_slice(&bytes[44..=47]).unwrap()).unwrap();
        //default_page_cache_size
        let default_page_cache_size = u32::try_from_be_byte_slice(&bytes[48..=51]).unwrap();
        //largest_root_btree_page_number
        let largest_root_btree_page_number = u32::try_from_be_byte_slice(&bytes[52..=55]).unwrap();
        //text_encoding
        let text_encoding: TextEncoding = num::FromPrimitive::from_u32(u32::try_from_be_byte_slice(&bytes[56..=59]).unwrap()).unwrap();
        //user_version
        let user_version = u32::try_from_be_byte_slice(&bytes[60..=63]).unwrap();
        //incremental_vacuum_mode
        let incremental_vacuum_mode = u32::try_from_be_byte_slice(&bytes[64..=67]).unwrap();
        //application_id
        let application_id = u32::try_from_be_byte_slice(&bytes[68..=71]).unwrap();
        //reserved
        let mut reserved = [0; 20];
        reserved.copy_from_slice(&bytes[72..=91]);
        //version_valid_for_number
        let version_valid_for_number = u32::try_from_be_byte_slice(&bytes[92..=95]).unwrap();
        //sqlite_version_number
        let sqlite_version_number = u32::try_from_be_byte_slice(&bytes[96..=99]).unwrap();


        let db_header = DatabaseHeader{
           header_string: header_string.to_string(),
           page_size,
           file_format_read_version,
           file_format_write_version, 
           bytes_at_unused_page_end,
           max_embedded_payload_fraction,
           min_embedded_payload_fraction,
           leaf_payload_fraction,
           file_change_count,
           in_header_database_size,
           first_freelist_trunk_page_number,
           total_freelist_page_number,
           schema_cookie,
           schema_format_number,
           default_page_cache_size,
           largest_root_btree_page_number,
           text_encoding,
           user_version,
           incremental_vacuum_mode,
           application_id,
           reserved,
           version_valid_for_number,
           sqlite_version_number,
        };
        Ok(db_header)
    }
}

pub struct Database {
    fm: FileManager,
    //header: DatabaseHeader<'a>,
    freelist: Vec<()>,
    pages: Vec<()>, 
}

pub enum PageType {
    InteriorIndexBtreePage = 0x02,
    InteriorTableBtreePage = 0x05,
    LeafIndexBtreePage = 0x0a,
    LeafTableBtreePage = 0x0d,
} 

pub struct PageHeader {
    page_type: u8,
    first_free_block_offset: u16,
    cell_number: u16,
    cell_content_area_offset: u16,
    fragmented_free_bytes: u8,
    right_most_pointer: u32,
}

pub struct Page {
    header: PageHeader,
    cell_pointers: Vec<CellPointer>,
    cells: Vec<Cell>,

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

