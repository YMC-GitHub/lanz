pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // this is a test func , please add #[test] above it.
        let result = add(2, 2);
        assert!(result == 4); // test bool
        assert_eq!(result, 4); //test eq
        assert_ne!(result, 8); //test ne
    }
}
