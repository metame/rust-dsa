pub mod array_list;
pub mod binary_search_tree;
pub mod doubly_linked_list;
pub mod linked_list;
pub mod priority_queue;
pub mod queue;
pub mod ring_buffer;
pub mod stack;
// rustonomicon reference implementation
pub mod vec;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
