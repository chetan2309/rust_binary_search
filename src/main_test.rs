#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_convert_string_into_vector_success() {
        use crate::convert_into_vector;
        let vector_string = "1,2,3,4,5,6,7,8,9";
        let result = convert_into_vector(&vector_string);
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(vec == result);
    }

    #[test]
    #[should_panic(expected = "Error.....")]
    fn test_convert_string_into_vector_failure() {
        use crate::convert_into_vector;
        let vector_string = "1,2,3,4,5,6,7,8,9,";
        let result = convert_into_vector(&vector_string);
        assert!(result.is_empty());
    }
}