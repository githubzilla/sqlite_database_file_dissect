use std::convert::TryInto;
use std::result::Result;
use std::mem;

use crate::utils::error::{MyError, ErrorKind};

pub trait TryFromBytes where Self: Sized {
    const SIZE_IN_BYTE: usize = mem::size_of::<Self>();
    fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, MyError>;
    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, MyError>;
}

macro_rules! impl_try_from_bytes {
    ($($t:ty),+) => {
        $(impl TryFromBytes for $t {
            fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, MyError> {
                if bytes.len() != Self::SIZE_IN_BYTE {
                    return Err(MyError::new(ErrorKind::SliceLengthError(Self::SIZE_IN_BYTE, bytes.len())));
                }
                Ok(<$t>::from_le_bytes(bytes.try_into().unwrap()))
            }

            fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, MyError> {
                if bytes.len() != Self::SIZE_IN_BYTE {
                    return Err(MyError::new(ErrorKind::SliceLengthError(Self::SIZE_IN_BYTE, bytes.len())));
                }
                Ok(<$t>::from_be_bytes(bytes.try_into().unwrap()))
            }
        })+
    }
}

impl_try_from_bytes!(u16, u32, usize, i8, i16, i32, i64, f64, u64);

impl<T: TryFromBytes> TryFromBytes for Vec<T>
{ 
    fn try_from_le_bytes(slice: &[u8]) -> Result<Self, MyError> {
       let result = (0..slice.len())
                       .step_by(T::SIZE_IN_BYTE )
                       .map(|i| T::try_from_le_bytes(&slice[i..i + T::SIZE_IN_BYTE]).unwrap())
                       .collect();
       Ok(result)
    }

    fn try_from_be_bytes(slice: &[u8]) -> Result<Self, MyError> {
       let result = (0..slice.len())
                       .step_by(T::SIZE_IN_BYTE )
                       .map(|i| T::try_from_be_bytes(&slice[i..i + T::SIZE_IN_BYTE]).unwrap())
                       .collect();
       Ok(result)
    }
}
