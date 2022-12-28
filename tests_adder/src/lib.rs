pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn this_does_not() {
        let result = add(2, 2);
        assert_ne!(result, 4);
    }

    #[test]
    fn wierd_stuff() {
        let result = add(2, 2);
        assert!(result == 4);
    }

    #[test]
    #[should_panic]
    fn meep_meep() {
        panic!("Panic in tha house");
    }

    // using Result type
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) // sending Ok will pass the test
        } else {
            Err(String::from("two plus two does not equal four")) // sending Err will cause the test to fail
        }
    }
}
