use integer_encoding::*;

use crate::components::page_header::PageType;
use crate::utils::convert::TryFromBytes;
use crate::utils::error::MyError;
use crate::utils::error::ErrorKind;

#[derive(Debug)]
pub struct Cell {
    pub page_type: PageType,
    pub left_child_page_number: Option<u32>,
    pub row_id: Option<usize>,
    pub payload_length: Option<usize>,
    pub payload: Option<Vec<u8>>,
    pub overflow_page_number: Option<u32>,
    pub overflow_length: Option<usize>,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            page_type: PageType::UnknowType,
            left_child_page_number: None,
            row_id: None,
            payload_length: None,
            payload: None,
            overflow_page_number: None,
            overflow_length: None,
        }
    }
}

impl Cell {
    pub fn try_from_bytes(bytes: &[u8], page_type: PageType, page_size: u16) -> Result<Cell, MyError> {

        //payload_length
        if page_type == PageType::IndexInteriorBtreePage {
            return Self::build_index_interior_page_cell(&bytes, page_size.into());
        } else if page_type == PageType::TableLeafBtreePage {
            return Self::build_table_leaf_page_cell(&bytes, page_size.into());
        } else if page_type == PageType::TableInteriorBtreePage {
            return Self::build_table_interior_page_cell(&bytes);
        } else if page_type == PageType::IndexLeafBtreePage {
            return Self::build_index_leaf_page_cell(&bytes, page_size.into());
        }

        Err(
            MyError::new(ErrorKind::UnreachableCode)
        )

    }

    fn get_left_child_page_number(bytes: &[u8]) -> u32 {
        u32::try_from_be_bytes(&bytes[0..=3]).unwrap()
    }

    fn get_row_id(bytes: &[u8]) -> (usize, usize) {
       usize::decode_var(&bytes[4..]).unwrap()
    }

    fn get_payload_length(bytes: &[u8]) -> (usize, usize) {
       usize::decode_var(&bytes[4..]).unwrap()
    }

    fn get_payload(bytes: &[u8], payload_length: usize, page_size: usize, x: usize) -> (Option<&[u8]>/*payload*/, 
                                                                                        Option<u32>/*overflow_page_number*/, 
                                                                                        Option<usize>/*remaining page length in overflow page*/) {
        let u: usize = page_size;
        let p: usize = payload_length;

        if p <= x {
            return (Some(bytes), None, None);
        } else {
            let m: usize = ((u-12)*32/255)-23;
            let k: usize = m+((p-m)%(u-4));

            if p > x && k <= x {
                let k: usize = m+((p-m)%(u-4));
                let overflow_page_number =  Self::get_overflow_page_number(&bytes[(k-1)..]);
                return (Some(&bytes[0..k]), Some(overflow_page_number), Some(p -k)); 
            } else if p > x && k > x {
                let overflow_page_number =  Self::get_overflow_page_number(&bytes[(m-1)..]);
                return (Some(&bytes[0..m]), Some(overflow_page_number), Some(p -m)); 
            }
        } 

        (None, None, None)
    }

    fn get_overflow_page_number(bytes: &[u8]) -> u32 {
        u32::try_from_be_bytes(&bytes[0..=3]).unwrap()
    } 

    fn build_table_interior_page_cell(bytes: &[u8]) -> Result<Cell, MyError> {
        let left_child_page_number = Self::get_left_child_page_number(&bytes[0..=3]);
        let (row_id, row_id_varint_len) = Self::get_row_id(&bytes[4..]);

        Ok(Cell{
            page_type: PageType::TableInteriorBtreePage,
            left_child_page_number: Some(left_child_page_number),
            row_id: Some(row_id),
            ..Default::default()
        })
    }

    fn build_table_leaf_page_cell(bytes: &[u8], page_size: usize) -> Result<Cell, MyError> {
        let (payload_length, payload_length_varint_len) = Self::get_payload_length(&bytes);
        let (row_id, row_id_varint_len) = Self::get_row_id(&bytes[payload_length_varint_len -1..]);

        let u = page_size;
        let x = u -35;
        let payload_start_index = payload_length_varint_len +row_id_varint_len;
        let (payload, overflow_page_number, overflow_length) = 
        Self::get_payload(&bytes[payload_start_index-1..], payload_length, page_size, x);

        Ok(Cell{
            page_type: PageType::TableLeafBtreePage,
            payload_length: Some(payload_length),
            row_id: Some(row_id),
            payload: match payload {Some(bytes) => Some(bytes.to_vec()), None => None, },
            overflow_page_number,
            overflow_length, 
            ..Default::default()
        })
    }

    fn build_index_interior_page_cell(bytes: &[u8], page_size: usize) -> Result<Cell, MyError> {
        let left_child_page_number = Self::get_left_child_page_number(&bytes);
        let (payload_length, payload_length_varint_len) = Self::get_payload_length(&bytes[4..]);
        let u = page_size;
        let x = ((u-12)*64/255)-23;
        let payload_start_index = 4 + payload_length_varint_len -1;
        let (payload, overflow_page_number, overflow_length) = Self::get_payload(&bytes[payload_start_index..], payload_length, page_size, x);

        Ok(Cell{
            page_type: PageType::IndexInteriorBtreePage,
            left_child_page_number: Some(left_child_page_number),
            payload_length: Some(payload_length),
            payload: match payload {Some(bytes) => Some(bytes.to_vec()), None => None, },
            overflow_page_number,
            overflow_length,
            ..Default::default()
        })
    }

    fn build_index_leaf_page_cell(bytes: &[u8], page_size: usize) -> Result<Cell, MyError> {
        let (payload_length, payload_length_varint_len) = Self::get_payload_length(&bytes);
        let u = page_size;
        let x = ((u-12)*64/255)-23;
        let payload_start_index = payload_length_varint_len -1;
        let (payload, overflow_page_number, overflow_length) = Self::get_payload(&bytes[payload_start_index..], payload_length, page_size, x);

        Ok(Cell{
            page_type: PageType::IndexLeafBtreePage,
            payload_length: Some(payload_length),
            payload: match payload {Some(bytes) => Some(bytes.to_vec()), None => None, },
            overflow_page_number,
            overflow_length,
            ..Default::default()
        })
    }
}

