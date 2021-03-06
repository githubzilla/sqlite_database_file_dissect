#![feature(destructuring_assignment)]

mod components;
mod utils;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::convert::TryInto;  

use actix_web::{get, web, App as WebApp, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use clap::{Arg, App as ClapApp};
use serde_json::json;
use lazy_static::*;
use std::sync::Mutex;
use std::sync::MutexGuard;

use sqlite_database_file_dissect::components::database_header::DatabaseHeader;
use sqlite_database_file_dissect::utils::convert::TryFromBytes;
use sqlite_database_file_dissect::components::page::Page;
use sqlite_database_file_dissect::utils::error::HttpErrorKind;
use sqlite_database_file_dissect::utils::error::HttpError;

lazy_static!{
    static ref SQLITE_DATABASE_FILE: Mutex<Option<String>> = Mutex::new(None);
    static ref SQLITE_DATABASE_FILE_PAGE_SIZE: Mutex<Option<usize>> = Mutex::new(None);
}

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
    let page_size: usize = SQLITE_DATABASE_FILE_PAGE_SIZE.lock().unwrap().unwrap();
    let mut f = File::open(SQLITE_DATABASE_FILE.lock().unwrap().as_ref().unwrap()).unwrap();
    let mut buffer = vec![0 as u8; page_size];
    let f_length: usize = f.metadata().unwrap().len().try_into().unwrap();
    let page_num: usize = f_length/page_size;
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
            page_size, 
            page_index, 
            &mut buffer, 
            &mut page_parents, 
            page_index,
            0);
    }

    let r = serde_json::to_string(&page_parents).unwrap();

    HttpResponse::Ok().body(r)
}

#[get("/btree_page_num")]
async fn btree_page_num() -> impl Responder {
    let page_size: usize = SQLITE_DATABASE_FILE_PAGE_SIZE.lock().unwrap().unwrap();
    let f = File::open(SQLITE_DATABASE_FILE.lock().unwrap().as_ref().unwrap()).unwrap();
    let f_length: usize = f.metadata().unwrap().len().try_into().unwrap();
    let page_num: usize = f_length/page_size;

    let r = json!({
        "page_num": page_num
    });

    HttpResponse::Ok().body(r)
}

#[get("/database_header")]
async fn fetch_database_header() -> impl Responder {
    let mut f = File::open(SQLITE_DATABASE_FILE.lock().unwrap().as_ref().unwrap()).unwrap();
    let mut buffer = vec![0 as u8; 100];
    let _r = f.read(&mut buffer);
    let database_header = DatabaseHeader::try_from_be_bytes(&buffer[0..100]).unwrap();

    let r = serde_json::to_string(&database_header).unwrap();

    HttpResponse::Ok().body(r)
}

#[get("/btree_page/{page_index}")]
async fn btree_page(web::Path(page_index): web::Path<usize>) -> impl Responder {
    let page_size: usize = SQLITE_DATABASE_FILE_PAGE_SIZE.lock().unwrap().unwrap();
    let mut f = File::open(SQLITE_DATABASE_FILE.lock().unwrap().as_ref().unwrap()).unwrap();
    let f_length: usize = f.metadata().unwrap().len().try_into().unwrap();
    let page_num: usize = f_length/page_size;

    if page_index < 0 || page_index > page_num {
        let r = serde_json::to_string(
            &HttpError::new(HttpErrorKind::PageIndexError(page_index))
        ).unwrap();
        return HttpResponse::BadRequest().body(r);
    }

    let mut buffer = vec![0 as u8; page_size];

    let page_start_offset: u64 = (page_index * page_size).try_into().unwrap();
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

    let r = serde_json::to_string(&page).unwrap();

    HttpResponse::Ok().body(r)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let matches = ClapApp::new("Sqlite Database File Dissect")
                    .version("1.0")
                    .author("Xiaoji Tian")
                    .about("Dissect sqlite database file")
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .value_name("FILE")
                         .help("sqlite database file name")
                         .required(true))
                    .get_matches();

    {
        let mut file_guard: MutexGuard<'_, Option<String>> = SQLITE_DATABASE_FILE.lock().unwrap();
        *file_guard = Some(matches.value_of("file").unwrap().to_string());
    }

    let mut f = File::open(SQLITE_DATABASE_FILE.lock().unwrap().as_ref().unwrap()).unwrap();
    let mut buffer = vec![0 as u8; 100];
    let _r = f.read(&mut buffer);
    let database_header = DatabaseHeader::try_from_be_bytes(&buffer[0..100]).unwrap();

    {
        let mut page_size_guard: MutexGuard<'_, Option<usize>> = SQLITE_DATABASE_FILE_PAGE_SIZE.lock().unwrap();
        *page_size_guard = Some(database_header.page_size as usize);
    }

    HttpServer::new(|| {
        WebApp::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(btree_hierachy)
            .service(btree_page)
            .service(btree_page_num)
            .service(fetch_database_header)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
