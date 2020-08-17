pub mod lib{
    pub fn select_element_and_perform_business_logic_and_assign(vector: &mut Vec<i32>) {
        vector[0] = vector[0] + 1;
    }

    pub fn select_element_and_perform_business_logic(vector: &mut Vec<i32>) -> i32 {
        vector[0] + 1
    }


    pub fn perform_business_logic(element: i32) -> i32 {
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

        lib::select_element_and_perform_business_logic_and_assign(&mut vector);

        assert_eq!(vector[0], 1);
    }

    #[test]
    fn split_the_responsibility() {
        let mut vector: Vec<i32> = Vec::new();
        vector.push(0);

        vector[0] = lib::select_element_and_perform_business_logic(&mut vector);

        assert_eq!(vector[0], 1);
    }


    #[test]
    fn make_it_more_idiomatic() {
        let mut vector: Vec<i32> = Vec::new();
        vector.push(0);

        vector[0] = lib::perform_business_logic(vector[0]);

        assert_eq!(vector[0], 1);
    }
}

#[cfg(test)]
mod hashmap_tests{
    use std::collections::HashMap;
    #[test]
    fn map_is_empty_by_default() {
        let map : HashMap<i32, i32> = HashMap::new();

        assert_eq!(map.len(), 0);
    }

    #[test]
    // Source: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get
    fn adding_one_element() {
        let mut map : HashMap<i32, i32> = HashMap::new();

        map.insert(10, 12);

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&10), Some(&12));
    }

    #[test]
    fn overriding_an_element() {
        let mut map : HashMap<i32, i32> = HashMap::new();

        map.insert(10, 12);
        map.insert(10, 20);

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&10), Some(&20));
    }
}