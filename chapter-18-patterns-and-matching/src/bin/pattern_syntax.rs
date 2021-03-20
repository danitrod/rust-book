#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
  let x = 1;

  // match with multiple patterns
  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // Matching in range (only for numeric or char values)
  let x = 5;

  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }

  let x = 'é';

  match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }

  // Destructuring structs
  struct Point {
    x: i32,
    y: i32,
  }

  let p = Point { x: 0, y: 7 };

  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);

  // or
  let Point { x, y } = p;
  assert_eq!(0, x);
  assert_eq!(7, y);

  // matching a part of a struct
  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }

  // destructuring enums
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);

  // The match destructures each enum type in a way
  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
  }

  // Destructuring nested types
  enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
  }

  enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
  }

  let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    Message2::ChangeColor(Color::Rgb(r, g, b)) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
    Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
      "Change the color to hue {}, saturation {}, and value {}",
      h, s, v
    ),
    _ => (),
  }

  // Destructuring structs and tuples
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
  assert_eq!(feet, 3);
  assert_eq!(x, 3);
  assert_eq!(y, -10);

  // Ignoring values in a pattern
  fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
    // println!("try to use _: {}", _); (fails)
  }
  foo(3, 4);

  // Ignoring parts of a value with nested _
  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }
    _ => {
      setting_value = new_setting_value;
    }
  }

  println!("setting is {:?}", setting_value);

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, _, third, _, fifth) => {
      println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
  }

  // _ and _<name> is different: only _ will not bind the value to the variable, while _x will
  let s = Some(String::from("Hello!"));

  if let Some(_) = s {
    println!("found a string");
  }

  println!("{:?}", s); // works: _ doesnt take s

  let s = Some(String::from("Hello!"));

  if let Some(_s) = s {
    println!("found a string");
  }

  // println!("{:?}", s); panics: _s takes s

  // Using .. to ignore parts of a value
  struct Point2 {
    x: i32,
    y: i32,
    z: i32,
  }

  let origin = Point2 { x: 0, y: 0, z: 0 };

  match origin {
    // ignore y and z
    Point2 { x, .. } => println!("x is {}", x),
  }

  // ignore numbers in the middle of a tuple
  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }

  // .. usage cannot be ambiguous
  // match numbers {
  //   (.., second, ..) => {
  //     println!("Some numbers: {}", second)
  //   }
  // }
  // (cant know if first .. has more values than second ..)

  // Match guards: more conditions in a match arm
  let num = Some(4);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  // Match shadowing problem: we can't use y to compare in a match arm
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);

  // That can be solved with a match guard:
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {}", n),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {}", x, y);

  // match guard precedence befavior
  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }
  // This works like (4 | 5 | 6) if y
  // rather  than (4 | 5 ) | 6 if y

  // @ bindings: used when we want to name a variable for use within
  // the match arm, but also check the value of the variable
  enum Message3 {
    Hello { id: i32 },
  }

  let msg = Message3::Hello { id: 5 };

  match msg {
    Message3::Hello {
      id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Message3::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
      // we can't use id here
    }
    Message3::Hello { id } => println!("Found some other id: {}", id),
  }
}
