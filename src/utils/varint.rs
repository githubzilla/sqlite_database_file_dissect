use crate::utils::error::MyError;

use sqlite_varint::read_varint;

pub fn decode_varint_to_usize(bytes: &[u8]) -> Result<(usize, usize), MyError> {
    let (v, l) = read_varint(bytes);
    Ok((v as usize, l))
}
