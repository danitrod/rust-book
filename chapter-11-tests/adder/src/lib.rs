// Options can be passed to the compiler with cargo test --<options>
// or to the binaries with cargo test -- --<options>

// Tests run in parallel by defualt. They can be run synchronously by passing
// --test-threads=1 to the binaries
// e.g.
// $ cargo test -- --test-threads=1

// Specific tests can be run by passing a substring included in the function names to the compiler.
// e.g.
// $ cargo test it_works
// will run it_works and it_works_with_result

// Private functions can also be tested.

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { _value: value }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;

    // Basic testing
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    // Assert eq
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Adding messages to assert
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // Expected panicking
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // Can also be a substring: #[should_panic(expected = "Guess")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Console output will only show if the test fails
    // Passing logs can be shown with the option --show-output on binaries
    // #[test]
    // fn fails_with_logs() {
    //     println!("Will make a valid assertion");
    //     assert_eq!(2, 2);
    //     println!("Ok. Will make an invalid assertion");
    //     assert_eq!(1, 2);
    //     println!("Test will have failed at this point.");
    // }

    #[test]
    fn passes_with_logs() {
        println!("This log will not show, as the test passes.");
        assert_eq!(1, 1);
    }

    // Ignoring a specific test: simpy add #[ignore]
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(1, 2);
    }

    // Ignored tests can be run exclusively with `cargo test -- --ignored`
}
