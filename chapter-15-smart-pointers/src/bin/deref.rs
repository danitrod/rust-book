use std::ops::Deref;

// A Box allocated on the heap
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

fn main() {
  // This doesn't work. A number and a reference to it are different types.
  // let x = 5;
  // let y = &x;
  // assert_eq!(5, x);
  // assert_eq!(5, *y);

  // This works.
  let x = 5;
  let y = Box::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y);

  // Deref coercion
  // Mybox<String> -> String -> &str
  let m = MyBox::new(String::from("Rust"));
  hello(&m);
}
