use std::io;
mod binary_search;
mod data_reader;

use binary_search::BinarySearch;
use data_reader::FileReader;

use crate::data_reader::DataReader;
mod binary_search_tests;
mod main_test;

fn main() {
    let binary_data = match FileReader::new("binary_search_data.txt").read_data_from_file() {
        Ok(contents) => contents,
        Err(_) => {
            print!("Error reading....");
            return;
        }
    };
    let vec = convert_into_vector(&binary_data);
    
    let mut search = String::new();
    io::stdin()
        .read_line(&mut search)
        .expect("Failed to read line");

    let search = search.trim().parse().unwrap_or(0);

    let result = BinarySearch::binary_search(&vec, search);
    println!("Result: {}", result);
    if result == -1 {
        println!("{} was not found in the search vector", search);
    } else {
        println!("Found {} at index {}", search, result);
    }
}

fn convert_into_vector(data: &str) -> Vec<i32> {
    data.split(',')
        .map(|s| s.trim().parse().expect("Error....."))
        .collect()
}
