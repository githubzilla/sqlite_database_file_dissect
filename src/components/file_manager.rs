use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use std::result::Result;

use memmap::Mmap;
use memmap::MmapOptions;

use crate::components::database::Database;

pub struct FileManager{
    f: File,
    mmap: Arc<Mutex<Mmap>>,
    db: Arc<Mutex<Database>>,
}

impl FileManager {
    pub fn open(&mut self, file_name: String ) -> Result<Arc<Mutex<Database>>, std::io::Error> {

        self.f = match File::open(file_name) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mmap = unsafe{ MmapOptions::new().map(&self.f)? };
        self.mmap = Arc::new(Mutex::new(mmap));

        let result = Arc::clone(&self.db);
        Ok(result)

    }

    pub fn read(&self, offset: usize, length: usize) -> Vec<u8> {

        vec![]
    }

    pub fn write(&self, offset: usize, data: Vec<u8>) -> Result<(), String> {

        Ok(())
    }

    pub fn append(&self, data: Vec<u8>) -> Result<(), String> {

        Ok(())

    }
    
}
