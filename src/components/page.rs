use crate::components::page_header::PageHeader;
use crate::components::cell_pointer::CellPointer;
use crate::utils::error::MyError;
use crate::utils::error::ErrorKind;
use crate::utils::convert::TryFromBytes;

pub struct Page {
    header: PageHeader,
    cell_pointers: Vec<CellPointer>,
    //cells: Vec<()>,
}

impl TryFromBytes for Page {
    fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        Err(MyError::new(ErrorKind::NotImplemented))
    }
    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, MyError> {

        let header = PageHeader::try_from_be_bytes(bytes).unwrap();
        //cell cell_pointers
        let cell_pointers : Vec<CellPointer> = Vec::new();

        Ok(Page {
            header,
            cell_pointers,

        })
         
    }
}



