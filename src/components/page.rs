use std::convert::TryFrom;

use crate::components::page_header::PageHeader;
use crate::components::cell_pointer::CellPointer;
use crate::utils::error::MyError;

pub struct Page {
    header: PageHeader,
    cell_pointers: Vec<CellPointer>,
    //cells: Vec<()>,
}

impl TryFrom<&[u8]> for Page {
    type Error = MyError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        let header = PageHeader::try_from(value).unwrap();
        //cell cell_pointers
        let cell_pointers : Vec<CellPointer> = Vec::new();

        Ok(Page {
            header,
            cell_pointers,

        })
         
    }
}



