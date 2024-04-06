pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // All array in the test file are sorted.
    array.binary_search(&key).ok()
}
