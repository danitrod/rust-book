pub fn add_two(n: isize) -> isize {
    n + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, add_two(2));
    }
}
