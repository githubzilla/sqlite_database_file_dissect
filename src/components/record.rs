use std::convert::TryInto;

use serde_derive::Serialize;

use crate::utils::convert::TryFromBytes;
use crate::utils::error::MyError;
use crate::utils::error::ErrorKind;
use crate::utils::varint::decode_varint_to_usize;

#[derive(Debug, Serialize)]
pub enum SerialType {
    UNKNOWN,
    NULL,
    I8,
    I16,
    I24,
    I32,
    I48,
    I64,
    F64,
    I0,
    I1,
    RESERVED10,
    RESERVED11,
    BLOB(usize /*blob length*/),
    STRING(usize /*string length*/),
}

#[derive(Debug, Serialize)]
pub enum Column {
    UNKNOWN,
    NULL,
    I8(i8),
    I16(i16),
    I24(i32),
    I32(i32),
    I48(i64),
    I64(i64),
    F64(f64),
    I0,
    I1,
    RESERVED10,
    RESERVED11,
    BLOB(Box<[u8]>),
    STRING(String),
}

#[derive(Debug, Serialize)]
pub struct Record {
    pub header_length: usize,
    pub serial_types: Vec<SerialType>,
    pub columns: Vec<Column>,
}

impl TryFromBytes for Record {
    fn try_from_le_bytes(_bytes: &[u8]) -> Result<Self, crate::utils::error::MyError> {
        Err(MyError::new(ErrorKind::NotImplemented))
    }

    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, crate::utils::error::MyError> {
        let (header_length, header_length_var_length) = decode_varint_to_usize(&bytes).unwrap();

        let header_in_bytes = &bytes[0..header_length.try_into().unwrap()];
        let mut serial_type_start_idx = header_length_var_length;
        let mut serial_types : Vec<SerialType> = Vec::new();

        //fetch all serial types
        while serial_type_start_idx < header_length {
            let (serial_type_val, serial_type_var_length) = decode_varint_to_usize(&header_in_bytes[serial_type_start_idx..]).unwrap();
            serial_type_start_idx += serial_type_var_length;
            let serial_type = match serial_type_val {
                0 => SerialType::NULL,
                1 => SerialType::I8,
                2 => SerialType::I16,
                3 => SerialType::I24,
                4 => SerialType::I32,
                5 => SerialType::I48,
                6 => SerialType::I64,
                7 => SerialType::F64,
                8 => SerialType::I0,
                9 => SerialType::I1,
                10 => SerialType::RESERVED10,
                11 => SerialType::RESERVED11,
                n if (n%2 == 0 && n >=12) => SerialType::BLOB((n-12)/12),
                n if (n%2 == 1 && n >=13) => SerialType::STRING((n-13)/2),
                _ => SerialType::UNKNOWN,
            }; 
            serial_types.push(serial_type);
        }

        //fetch columns according to serial types
        let mut column_start_idx = header_length;
        let mut columns : Vec<Column> = Vec::new();
        for serial_type in serial_types.iter() {
            let column = match serial_type {
                SerialType::UNKNOWN => {
                    Column::UNKNOWN
                },
                SerialType::NULL => {
                    Column::NULL
                },
                SerialType::I8 => {
                    let r = i8::try_from_be_bytes(&bytes[column_start_idx..=column_start_idx]).unwrap();
                    column_start_idx += 1;
                    Column::I8(r)
                }, 
                SerialType::I16 => {
                    let r = i16::try_from_be_bytes(&bytes[column_start_idx..(column_start_idx + 2)]).unwrap();
                    column_start_idx += 2;
                    Column::I16(r)
                },
                SerialType::I24 => {
                    let mut d: [u8; 4] = [0; 4];
                    &d[1..].copy_from_slice(&bytes[column_start_idx..(column_start_idx + 3)]);
                    let r = i32::try_from_be_bytes(&d).unwrap();
                    column_start_idx += 3;
                    Column::I24(r)
                },
                SerialType::I32 => {
                    let r = i32::try_from_be_bytes(&bytes[column_start_idx..(column_start_idx + 4)]).unwrap();
                    column_start_idx += 4;
                    Column::I32(r)
                },
                SerialType::I48 => {
                    let mut d: [u8; 8] = [0; 8];
                    &d[2..].copy_from_slice(&bytes[column_start_idx..(column_start_idx + 6)]);
                    let r = i64::try_from_be_bytes(&d).unwrap();
                    column_start_idx += 6;
                    Column::I48(r)
                },
                SerialType::I64 => {
                    let r = i64::try_from_be_bytes(&bytes[column_start_idx..(column_start_idx + 8)]).unwrap();
                    column_start_idx += 8;
                    Column::I64(r)
                },
                SerialType::F64 => {
                    let r = f64::try_from_be_bytes(&bytes[column_start_idx..(column_start_idx + 8)]).unwrap();
                    column_start_idx += 8;
                    Column::F64(r)
                },
                SerialType::I0 => {
                    Column::I0
                },
                SerialType::I1 => {
                    Column::I1
                },
                SerialType::RESERVED10 => {
                    Column::RESERVED10
                },
                SerialType::RESERVED11 => {
                    Column::RESERVED11
                },
                SerialType::BLOB(len) => {
                    let mut blob = vec![0; *len].into_boxed_slice();
                    blob.copy_from_slice(&bytes[column_start_idx..(column_start_idx + len)]);
                    column_start_idx += len;
                    Column::BLOB(blob)
                },
                SerialType::STRING(len) => {
                    let s = String::from_utf8(bytes[column_start_idx..(column_start_idx + len)].to_vec()).unwrap();
                    column_start_idx += len;
                    Column::STRING(s)
                },
            };
            
            columns.push(column);
        };

        Ok(Record {
            header_length,
            serial_types,
            columns,
        })

    }
}
