enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Recursive cons list with Box
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    match list {
        Cons(n, b) => println!(
            "List has {} and cons has {}.",
            n,
            match *b {
                Cons(a, _) => a,
                Nil => -1,
            }
        ),
        Nil => println!("Empty list."),
    }
}
