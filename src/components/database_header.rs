use serde_derive::Serialize;

use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use crate::utils::error::MyError;
use crate::utils::error::ErrorKind;
use crate::utils::convert::TryFromBytes;

#[derive(Debug, FromPrimitive, ToPrimitive, Serialize)]
pub enum FileFormatVersion {
    Legacy = 1,
    WAL = 2,
}

#[derive(Debug, FromPrimitive, ToPrimitive, Serialize)]
pub enum SchemaFormatNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[derive(Debug, FromPrimitive, ToPrimitive, Serialize)]
pub enum TextEncoding {
    UTF8 = 1,
    UTF16le = 2,
    UTF16be = 3,
}

#[derive(Debug, Serialize)]
pub struct DatabaseHeader {
    pub header_string: String,
    pub page_size: u16,
    pub file_format_read_version: FileFormatVersion,
    pub file_format_write_version: FileFormatVersion,
    pub bytes_at_unused_page_end: u8,
    pub max_embedded_payload_fraction: u8,
    pub min_embedded_payload_fraction: u8,
    pub leaf_payload_fraction: u8,
    pub file_change_count: u32,
    pub in_header_database_size: u32,
    pub first_freelist_trunk_page_number: u32,
    pub total_freelist_page_number: u32,
    pub schema_cookie: u32,
    pub schema_format_number: SchemaFormatNumber,
    pub default_page_cache_size: u32,
    pub largest_root_btree_page_number: u32,
    pub text_encoding: TextEncoding,
    pub user_version: u32,
    pub incremental_vacuum_mode: u32,
    pub application_id: u32,
    pub reserved: [u8; 20],
    pub version_valid_for_number: u32,
    pub sqlite_version_number: u32,
}

/**
* implement of DatabaseHeader
*/
pub const SQLITE_DB_HEADER_STRING: &'static str = "SQLite format 3\0";

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

impl TryFromBytes for DatabaseHeader {

    fn try_from_le_bytes(_bytes: &[u8]) -> Result<Self, MyError> {
        Err(MyError::new(ErrorKind::NotImplemented))
    }

    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        //header_string
        let header_string = String::from_utf8_lossy(&bytes[0..=15]);
        //page_size
        let page_size = u16::try_from_be_bytes(&bytes[16..=17]).unwrap();
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
        let file_change_count = u32::try_from_be_bytes(&bytes[24..=27]).unwrap();
        //in_header_database_size
        //let in_header_database_size = u32::from_be_bytes(<[u8; 4]>::try_from(&bytes[28..=31]).unwrap());
        let in_header_database_size = u32::try_from_be_bytes(&bytes[28..=31]).unwrap();
        //first_freelist_trunk_page_number
        let first_freelist_trunk_page_number = u32::try_from_be_bytes(&bytes[32..=35]).unwrap();
        //total_freelist_page_number
        let total_freelist_page_number = u32::try_from_be_bytes(&bytes[36..=39]).unwrap();
        //schema_cookie
        let schema_cookie = u32::try_from_be_bytes(&bytes[40..=43]).unwrap();
        //schema_format_number
        let schema_format_number: SchemaFormatNumber = num::FromPrimitive::from_u32(u32::try_from_be_bytes(&bytes[44..=47]).unwrap()).unwrap();
        //default_page_cache_size
        let default_page_cache_size = u32::try_from_be_bytes(&bytes[48..=51]).unwrap();
        //largest_root_btree_page_number
        let largest_root_btree_page_number = u32::try_from_be_bytes(&bytes[52..=55]).unwrap();
        //text_encoding
        let text_encoding: TextEncoding = num::FromPrimitive::from_u32(u32::try_from_be_bytes(&bytes[56..=59]).unwrap()).unwrap();
        //user_version
        let user_version = u32::try_from_be_bytes(&bytes[60..=63]).unwrap();
        //incremental_vacuum_mode
        let incremental_vacuum_mode = u32::try_from_be_bytes(&bytes[64..=67]).unwrap();
        //application_id
        let application_id = u32::try_from_be_bytes(&bytes[68..=71]).unwrap();
        //reserved
        let mut reserved = [0; 20];
        reserved.copy_from_slice(&bytes[72..=91]);
        //version_valid_for_number
        let version_valid_for_number = u32::try_from_be_bytes(&bytes[92..=95]).unwrap();
        //sqlite_version_number
        let sqlite_version_number = u32::try_from_be_bytes(&bytes[96..=99]).unwrap();


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
