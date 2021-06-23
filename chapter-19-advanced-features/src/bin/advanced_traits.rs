#[allow(unused)]

fn main() {
    // Associated types: similar to generics, but you can't actually implement a trait with different
    // type parameters, so you don't have to anotate the type when using the trait.
    // e.g.
    struct Counter {}

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(5)
        }
    }

    let mut counter = Counter {};
    let u = counter.next(); // We know it is a u32

    // With generics:
    struct GenericCounter {}

    trait GenericIterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    impl GenericIterator<u32> for GenericCounter {
        fn next(&mut self) -> Option<u32> {
            Some(5)
        }
    }

    impl GenericIterator<String> for GenericCounter {
        fn next(&mut self) -> Option<String> {
            Some(String::new())
        }
    }

    let mut counter = GenericCounter {};
    // Next line errors without anotation: can't know which next type to use
    // let _ = counter.next();

    // Default generic type params
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Here, Add has a generic type which defines the RHS,
    // by default it is the same type of self
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // For example, we can feed a generic to Add for adding meters with mms
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // this is useful:
    // To extend a type without breaking existing code
    // To allow customization in specific cases most users won’t need (case of Add)

    // Calling methods with the same name
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    // By default, the method directly of the type will be called
    let person = Human;
    person.fly(); // "*waving arms furiously*"

    // To call trait specific methods, we need to use the trait
    Pilot::fly(&person);
    Wizard::fly(&person);
    // We can also do it with the struct
    Human::fly(&person); // same as person.fly()

    // An exception: for associated functions (they don't have the self parameter)
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name()); // We wanted to get "puppy", but get "Spot"

    // We also can't do the following, it can't figure out who implemented the trait
    // Animal::baby_name();

    // For that, we use fully qualified syntax:
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // We specify that we want
                                                                         // the method from the trait

    // Supertraits: defining traits that require another trait's functionality
    // For example: an OutlinePrint trait that requires Display to display an object outlined
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string(); // We can use to_string because we know self is Display
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // The following errors: We can't impl OutlinePrint because Point is not Display
    // impl OutlinePrint for Point {}

    // Let's implement
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // Now we can
    impl OutlinePrint for Point {}
    let point = Point { x: 1, y: 2 };
    point.outline_print();

    // Newtypes: getting around the restriction of not implementint traits on types from other crates
    // Simply add a tuple struct wrapper to the type
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // To treat the wrapper as the proper type, without having to use the wrapper.0 syntax to
    // access it, we could implement Deref for the wrapper, for that to be done automatically.
}
