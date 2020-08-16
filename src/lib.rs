pub mod lib{
    pub fn increase_in_array(vector: &mut Vec<i32>) {
        vector[0] = vector[0] + 1;
    }

    pub fn increase_in_array_2(vector: &mut Vec<i32>) -> i32 {
        vector[0] + 1
    }


    pub fn increase_in_array_3(element: i32) -> i32 {
        element + 1
    }
}

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

    // #[test]
    // fn cannot_add_an_element_to_an_immutable_vector() {
    //     let vector: Vec<i32> = Vec::new();

    //     vector.push(2);
    //     ^^^^^^ cannot borrow as mutable

    //     assert_eq!(vector.len(), 1);
    // }
}

#[cfg(test)]
mod library_tests{
    use crate::lib;
    #[test]
    fn increased_the_first_element() {
        let mut vector: Vec<i32> = Vec::new();
        vector.push(0);

        lib::increase_in_array(&mut vector);

        assert_eq!(vector[0], 1);
    }

    #[test]
    fn split_the_responsibility() {
        let mut vector: Vec<i32> = Vec::new();
        vector.push(0);

        vector[0] = lib::increase_in_array_2(&mut vector);

        assert_eq!(vector[0], 1);
    }


    #[test]
    fn make_it_more_idiomatic() {
        let mut vector: Vec<i32> = Vec::new();
        vector.push(0);

        vector[0] = lib::increase_in_array_3(vector[0]);

        assert_eq!(vector[0], 1);
    }
}