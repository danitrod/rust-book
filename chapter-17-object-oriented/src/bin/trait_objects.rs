// Creating a GUI with multiple components
// we can't know the type of a component created by the user
// but it needs to implement the `draw` method
// so we use the `Draw` trait for the components
// and assure every component is a Draw trait object
pub trait Draw {
  fn draw(&self);
}

// Assure every entry of the Vec is Draw
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

// We can't use this:
// pub struct Screen<T: Draw> {
//   pub components: Vec<T>,
// }
// because that would restrict the Vec to only have one type

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!(
      "Drawing a {}x{} button labeled {}",
      self.width, self.height, self.label
    );
  }
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!(
      "Drawing a {}x{} select box with options: {:?}",
      self.width, self.height, self.options
    );
  }
}

fn main() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
      // Code won't compile if we try to add a not Draw type
      // Box::new(String::from("Hi")),
    ],
  };

  screen.run();
}

// Trait objects perform dynamic dispatch, as opposed to static dispatch
// that means the code isn't monomorphized: the compiler can't tell
// what code will run at compile time
// There is a runtime cost to look at what method to call inside the
// trait object
// It's a trade-off: a little bit of performance for more flexibillity

// Traits used in trait objects have to be object-safe:
// - the return type can't be Self
// - there can't be generic type parameters
