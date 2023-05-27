// File: data_reader.rs

use std::{fs::File, io::{self, Read}};

pub trait DataReader {
    fn read_data_from_file(&self) -> Result<String, io::Error>;
} 

pub struct FileReader {
    file_path: String,
}

impl FileReader {
    pub fn new(file_path: &str) -> Self {
        FileReader {
            file_path: file_path.to_string()
        }
    }
}

impl DataReader for FileReader {
    fn read_data_from_file(&self) -> Result<String, io::Error> {
      let binary_data_file = File::open(&self.file_path);
      let mut binary_data: File = match binary_data_file {
          Ok(file) => file,
          Err(err) => {
              return Err(err)
          },
      };

      let mut data = String::new();
      binary_data.read_to_string(&mut data)?;
      Ok(data)
    }
}