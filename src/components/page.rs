use serde_derive::Serialize;

use crate::components::cell::Cell;
use crate::components::cell_pointer::CellPointer;
use crate::components::page_header::PageHeader;
use crate::utils::convert::TryFromBytes;
use crate::utils::error::MyError;

#[derive(Debug, Serialize)]
pub struct Page {
    pub header: PageHeader,
    pub cell_pointers: Vec<CellPointer>,
    pub cells: Vec<Cell>,
}

impl Page {
    pub fn try_from_be_bytes(
        bytes: &[u8],
        header_start_index: Option<usize>,
    ) -> Result<Self, MyError> {
        //page header
        let (header, header_start_idx) = match header_start_index {
            Some(start_idx) => (
                PageHeader::try_from_be_bytes(&bytes[start_idx..]).unwrap(),
                start_idx,
            ),
            None => (PageHeader::try_from_be_bytes(bytes).unwrap(), 0),
        };
        //cell number
        let cell_number: usize = header.cell_number.into();
        //cell cell_pointers
        let cell_pointers_start_index: usize = (header_start_idx + header.length).into();
        let cell_pointers_length: usize =
            (std::mem::size_of::<u16>() as usize * cell_number).into();
        let cell_pointers_end_index = cell_pointers_start_index + cell_pointers_length - 1;
        let cell_pointers: Vec<CellPointer> = <Vec<CellPointer>>::try_from_be_bytes(
            &bytes[cell_pointers_start_index..=cell_pointers_end_index],
        )
        .unwrap();

        //cells
        let cells: Vec<Cell> = cell_pointers
            .iter()
            .map(|cell_pointer| {
                let offset: usize = cell_pointer.offset.into();
                let cell = Cell::try_from_bytes(&bytes[offset..], header.page_type, 4096).unwrap();
                return cell;
            })
            .collect();

        Ok(Page {
            header,
            cell_pointers,
            cells,
        })
    }
}
