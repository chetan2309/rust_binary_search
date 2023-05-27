use std::cmp::Ordering;

pub struct BinarySearch;
impl BinarySearch {
    pub fn new() -> Self {
        BinarySearch
    }

    pub fn binary_search(search_vector: &Vec<i32>, find: i32) -> isize {
        let mut lower_bound = 0;
        let mut upper_bound = search_vector.len() - 1;
    
        while lower_bound <= upper_bound {
            let mid: usize = (lower_bound + upper_bound) / 2;
            match search_vector[mid].cmp(&(find)) {
                Ordering::Less => lower_bound = mid + 1,
                Ordering::Greater => upper_bound = mid - 1,
                Ordering::Equal => return mid as isize,
            }
        }
        -1
    }
}

impl Default for BinarySearch {
    fn default() -> Self {
        Self::new()
    }
}