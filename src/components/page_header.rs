use std::convert::TryFrom;
use std::cmp::PartialEq;

use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use crate::utils::error::MyError;
use crate::utils::error::ErrorKind;
use crate::utils::convert::TryFromBytes;

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum PageType {
    UnknowType = 0x00,
    IndexInteriorBtreePage = 0x02,
    IndexLeafBtreePage = 0x0a,
    TableInteriorBtreePage = 0x05,
    TableLeafBtreePage = 0x0d,
} 

#[derive(Debug)]
pub struct PageHeader {
    pub page_type: PageType,
    pub first_free_block_offset: u16,
    pub cell_number: u16,
    pub cell_content_area_offset: u16,
    pub fragmented_free_bytes: u8,
    pub right_most_pointer: u32,
    pub length: usize,
}

impl Default for PageHeader {
    fn default() -> Self {
       PageHeader { 
            page_type: PageType::UnknowType, 
            first_free_block_offset: 0, 
            cell_number: 0, 
            cell_content_area_offset: 0, 
            fragmented_free_bytes: 0, 
            right_most_pointer: 0,
            length: 0,
        } 
    }
}

 fn try_from_be_8_bytes(value: &[u8; 8]) -> Result<PageHeader, MyError> {
     //page_type
     let page_type = num::FromPrimitive::from_u8(value[0]).unwrap();
     //first_free_block_offset
     let first_free_block_offset = u16::try_from_be_bytes(&value[1..=2]).unwrap();
     //cell_number
     let cell_number = u16::try_from_be_bytes(&value[3..=4]).unwrap();
     //cell_content_area_offset
     let cell_content_area_offset = u16::try_from_be_bytes(&value[5..=6]).unwrap();
     //fragmented_free_bytes
     let fragmented_free_bytes = value[7];
     //right_most_pointer
     let right_most_pointer = 0;

     Ok(PageHeader{
         page_type,
         first_free_block_offset,
         cell_number,
         cell_content_area_offset,
         fragmented_free_bytes,
         right_most_pointer,
         length: 8,
     })
 }

 fn try_from_be_12_bytes(value: &[u8; 12]) -> Result<PageHeader, MyError> {
     let mut value_8_bytes = [0; 8];
     value_8_bytes.copy_from_slice(&value[0..=7]);
     let mut page_header = try_from_be_8_bytes(&value_8_bytes).unwrap();
     //right_most_pointer
     let right_most_pointer = u32::try_from_be_bytes(&value[8..=11]).unwrap();
     page_header.right_most_pointer = right_most_pointer;
     page_header.length = 12;
     Ok(page_header)
 }

impl TryFromBytes for PageHeader {

    fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        Err(MyError::new(ErrorKind::NotImplemented))
    }

    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        let page_type = PageHeader::detect_page_type(bytes[0]);

        //page header
        let page_header = match page_type {
            PageType::IndexInteriorBtreePage | PageType::TableInteriorBtreePage => 
                try_from_be_12_bytes(<&[u8; 12]>::try_from(&bytes[0..12]).unwrap()),
            PageType::IndexLeafBtreePage | PageType::TableLeafBtreePage => 
                try_from_be_8_bytes(<&[u8; 8]>::try_from(&bytes[0..8]).unwrap()),
            PageType::UnknowType => return Err(MyError::new(ErrorKind::UnknowPageType(bytes[0]))),
        };

        page_header
    }
}

impl PageHeader {
    pub fn detect_page_type(value: u8) -> PageType {
        match value {
            2  => PageType::IndexInteriorBtreePage,
            5  => PageType::TableInteriorBtreePage,
            10 => PageType::IndexLeafBtreePage,
            13 => PageType::TableLeafBtreePage,
            _  => PageType::UnknowType,
        }
    }
}





