use std::io;
use std::vec;

fn main() {
    let vec = vec![1, 2, 3, 4, 7, 9, 13];
    let mut search = String::new();

    io::stdin().read_line(&mut search).expect("Failed to read line");

    let search = match search.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = binary_search(&vec, search);
    if result < vec.len() && vec[result] == search {
        println!("Found {} at index {}", search, result);
    } else {
        println!("{} was not found in the search vector", search);
    }
}

fn binary_search(search_vector: &Vec<i32>, find: i32) -> usize {
    let mut lower_bound  = 0;
    let mut upper_bound = search_vector.len() - 1;

    while lower_bound < upper_bound {
        let mid: usize = (lower_bound + upper_bound) / 2;
        match search_vector[mid] {
            x if x == find => return mid,
            x if x < find => lower_bound = mid  + 1,
            _ => upper_bound = mid - 1,
        }
    }
    lower_bound
}