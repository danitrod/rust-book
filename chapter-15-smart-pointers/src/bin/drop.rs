struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let d = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  println!("CustomSmartPointers created.");
  // Can't call explicitly the destructor like below
  // c.drop();

  // The std::mem::drop function works though
  drop(c);
}
