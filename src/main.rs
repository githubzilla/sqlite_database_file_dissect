#![feature(destructuring_assignment)]

mod components;
mod utils;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::convert::TryInto;  

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use sqlite_database_file_dissect::components::database_header::DatabaseHeader;
use sqlite_database_file_dissect::utils::convert::TryFromBytes;
use sqlite_database_file_dissect::components::page::Page;

fn travel_btree_page(f: &mut File, 
                     page_size: usize, 
                     page_index: usize, 
                     buffer: &mut [u8], 
                     page_parents: &mut Vec<i32>, 
                     btree_id: usize, 
                     btree_level: usize) {
    if page_parents[page_index] == -100{
        page_parents[page_index] = -1;
    }

    let page_start_offset: u64 = (page_index * page_size).try_into().unwrap();
    f.seek(SeekFrom::Start(page_start_offset)).unwrap();
    let _r = f.read(buffer);
    let page_start_offset;

    if page_index == 0 {
        let _ = DatabaseHeader::try_from_be_bytes(&buffer[0..100]).unwrap();
        page_start_offset = 100;
    } else {
        page_start_offset = 0;
    } 
    
    let page = Page::try_from_be_bytes(&buffer, Some(page_start_offset)).unwrap();

    for cell_idx in 0..page.cells.len() {
        let cell = &page.cells[cell_idx];
        match cell.left_child_page_number {
            Some(left_child_page_number) => {
                page_parents[(left_child_page_number-1) as usize] = page_index as i32;
            },
            None => (),
        };
    }

    match page.header.right_most_pointer {
        Some(right_most_pointer) => page_parents[(right_most_pointer -1) as usize] = page_index as i32,
        None => (),
    }

    for cell_idx in 0..page.cells.len() {
        let cell = &page.cells[cell_idx];
        match cell.left_child_page_number {
            Some(left_child_page_number) => travel_btree_page(f, 
                                                page_size, 
                                                (left_child_page_number -1).try_into().unwrap(), 
                                                buffer, 
                                                page_parents, 
                                                btree_id, 
                                                btree_level +1),
            None => (),
        }
    }

    match page.header.right_most_pointer {
        Some(right_most_pointer) => travel_btree_page(
                                                       f, 
                                                       page_size, 
                                                       (right_most_pointer -1) as usize, 
                                                       buffer, 
                                                       page_parents, 
                                                       btree_id, 
                                                       btree_level +1),
        None => (),
    }

}

#[get("/btree_hierachy")]
async fn btree_hierachy() -> impl Responder{

    let mut f = File::open("test-data/Chinook.db.4.analyze").unwrap();
    const PAGE_SIZE: usize = 4096;
    let mut buffer: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
    let f_length: usize = f.metadata().unwrap().len().try_into().unwrap();
    let page_num: usize = f_length/PAGE_SIZE;
    let mut page_parents: Vec<i32>  = Vec::with_capacity(page_num);

    for _ in 0..page_num {
        page_parents.push(-100);
    }

    for page_index in 0..page_parents.len() {
        let is_traveled = page_parents[page_index];
        if is_traveled != -100 {
            continue;
        }
        // read the page
        travel_btree_page(
            &mut f, 
            PAGE_SIZE, 
            page_index, 
            &mut buffer, 
            &mut page_parents, 
            page_index,
            0);
    }

    let r = serde_json::to_string_pretty(&page_parents).unwrap();

    HttpResponse::Ok().body(r)
}

#[get("/btree_page/{page_index}")]
async fn btree_page(web::Path(page_index): web::Path<usize>) -> impl Responder {
    let mut f = File::open("test-data/Chinook.db.4.analyze").unwrap();
    const PAGE_SIZE: usize = 4096;
    let mut buffer: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

    let page_start_offset: u64 = (page_index * PAGE_SIZE).try_into().unwrap();
    f.seek(SeekFrom::Start(page_start_offset)).unwrap();
    let _r = f.read(&mut buffer);
    let page_start_offset;

    if page_index == 0 {
        let _ = DatabaseHeader::try_from_be_bytes(&buffer[0..100]).unwrap();
        page_start_offset = 100;
    } else {
        page_start_offset = 0;
    } 

    let page = Page::try_from_be_bytes(&buffer, Some(page_start_offset)).unwrap();

    let r = serde_json::to_string_pretty(&page).unwrap();

    HttpResponse::Ok().body(r)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(btree_hierachy)
            .service(btree_page)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
