use std::fs::File;
use std::io;
use std::io::Read;
use std::cmp::Ordering;

fn main() {
    // let vec = vec![1, 2, 3, 4, 7, 9, 13];
    let binary_data = match read_data_from_file() {
        Ok(contents) => contents,
        Err(_) => {
            print!("Error reading....");
            return;
        }
    };
    let vec = binary_data
      .split(',')
      .map(|s| s.trim().parse().expect("Error....."))
      .collect();
    let mut search = String::new();

    io::stdin().read_line(&mut search).expect("Failed to read line");

    let search = match search.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = binary_search(&vec, search);
    println!("Result: {}", result);
    if result == -1 {
        println!("{} was not found in the search vector", search);
    } else {
        println!("Found {} at index {}", search, result);
    }
}

fn read_data_from_file() -> Result<String, io::Error> {
    let binary_data_file = File::open("binary_search_data.txt");
    let mut binary_data: File = match binary_data_file {
        Ok(file) => file,
        Err(err) => {
            return Err(err)
        },
    };

    let mut data = String::new();
    match binary_data.read_to_string(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e)
    }
}

fn binary_search(search_vector: &Vec<i32>, find: i32) -> isize {
    let mut lower_bound  = 0;
    let mut upper_bound = search_vector.len() - 1;

    while lower_bound <= upper_bound {
        let mid: usize = (lower_bound + upper_bound) / 2;
        match mid.cmp(&(find as usize)) {
            Ordering::Less => lower_bound = mid + 1,
            Ordering::Greater => upper_bound = mid - 1,
            Ordering::Equal => return mid as isize,
        }
    }
    -1
}