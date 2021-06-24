#[allow(unused)]

fn main() {
    // Type aliasing
    type Kilometers = i32;
    // This does not create a newtype; only a synonym for i32.

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // It is used mainly to avoid repetition
    // e.g. this long type
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| println!("hi"))
    }

    // Can be replaced with an alias
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_short_type(f: Thunk) {
        // --snip--
    }

    fn returns_short_type() -> Thunk {
        Box::new(|| println!("hi"))
    }

    // It reduces the amount of code by quite a bit, and also gives a more meaningful name to the
    // type.
    // It can also be used to simplify a repeated Result type:
    use std::fmt;
    use std::io::Error;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }

    use std::io::Result as ResultAliased;
    pub trait WriteAliased {
        fn write(&mut self, buf: &[u8]) -> ResultAliased<usize>;
        fn flush(&mut self) -> ResultAliased<()>;

        fn write_all(&mut self, buf: &[u8]) -> ResultAliased<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> ResultAliased<()>;
    }
    // std::io provides the Result alias so only one type param needs to be provided (the error
    // will always be std::io::Error)

    // The never type (!)
    // it is used for a function that never returns. For example, from the guessing game:
    let guess = "guess";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    // The match arms return different types, but continue actually returns a ! (never) type
    // to denote it never is executed. When the code comes to it, another iteration of the loop
    // will run, and so forth, until the code falls into the Ok.

    // It is also used for panic:
    struct WrappedOption<T>(Option<T>);
    impl<T> WrappedOption<T> {
        pub fn unwrap(self) -> T {
            match self.0 {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
                // panic returns a ! (it actually ends the program) so the match arms don't need
                // to return the same type
            }
        }
    }

    // Dynamically sized types, or commonly unsized types
    // let s1: str = "hey"; error: unknown size, can't compile
    // str on its own is a DST, can't know it's actual size until runtime.
    // Because different str's may have different length.
    // That's why we use &str, it is a slice with starting position and length.
    // We can know the size of the &str, because it holds just these two values, independently of
    // the actual size of the str.

    // Traits are also DSTs
    // To use traits as trait objects, should put them behind a pointer of some kind, like
    // Box<dyn Trait>

    // The std trait Sized determines whether any type can have known size at compile time.
    // Also, every generic type is Sized. To have a generic type that can not be Sized, the
    // following syntax can be used:
    fn generic<T: ?Sized>(t: &T) {}
    // The ? means that the type can or not be Sized. This syntax is available only for Sized.
    // As the type can be not Sized, it should be used from a pointer, therefore the &.
}
