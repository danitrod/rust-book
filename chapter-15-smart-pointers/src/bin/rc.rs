// RC: Reference counting

// Fails: Try referencing cons lists with boxes more than once
// enum List {
//   Cons(i32, Box<List>),
//   Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//   let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//   let b = Cons(3, Box::new(a));
//   // Can't borrow a again as it is moved to b
//   // let c = Cons(4, Box::new(a));
// }

// Ok: using rc
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
