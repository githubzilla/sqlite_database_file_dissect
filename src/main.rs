#![feature(destructuring_assignment)]

mod components;
mod utils;

use std::env;

use sqlite_database_file_dissect::components::database_header::*;
use sqlite_database_file_dissect::components::database::*;
use sqlite_database_file_dissect::components::page_header::*;
use sqlite_database_file_dissect::components::page::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
