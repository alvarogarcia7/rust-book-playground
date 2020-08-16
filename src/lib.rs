#[cfg(test)]
mod tests {
    #[test]
    fn canary_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn a_vector_is_empty_by_default() {
        let vector: Vec<i32> = Vec::new();

        assert_eq!(vector.len(), 0);
    }

    #[test]
    fn adding_elements_to_a_vector() {
        let mut vector: Vec<i32> = Vec::new();

        vector.push(2);

        assert_eq!(vector.len(), 1);
    }
}