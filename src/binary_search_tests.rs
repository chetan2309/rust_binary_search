#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search_success() {
        use crate::binary_search::BinarySearch;
        let vec  = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = BinarySearch::binary_search(&vec, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn func_should_return_minus_one_for_failure() {
        use crate::binary_search::BinarySearch;
        let vec  = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = BinarySearch::binary_search(&vec, 10);
        assert_eq!(result, -1);
    }
}