#[allow(unused)]

fn main() {
    // Function pointers
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // Receive a fn as parameter
    // fn's are all three: Fn, FnOnce and FnMut
    // fn's can always be passed where closures are expected.
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // It's better to write one of the closure traits than function pointers,
    // so closures can also be accepted. Example in map:
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect(); // with closure

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect(); // with fn

    // Using the initializer function: in tuple structs and tuple struct enum variants,
    // you can use the initializer as a function to return an instance of them.
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // [Status::Value(0), Status::Value(1), Status::Value(1), ...]
    // (same as defining a closure)

    // Function pointers can't be returned from functions. Closures also by default can't, as they're not Sized.
    // They can be returned as trait objects, within a Box.
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
