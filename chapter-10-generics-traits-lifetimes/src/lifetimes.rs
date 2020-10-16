// fn dangling_reference() {
//   let r;

//   {
//     let x = 5;
//     r = &x;
//   }
//   // Doesn't compile, as r refers to a variable which does not exist.
//   println!("r: {}", r);
// }

fn main() {
  // Example 1: same lifetimes
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);

  // Example 2: string2 has a shorter lifetime, but no problem
  let string1 = String::from("long string is long");
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }

  // Example 3: doesn't compile: string 2 is accessed after its lifetime ended
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {}", result);
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// This fails as the return value lifetime ends together with the function
fn longest<'a>(x: &str, y: &str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
}

// Using referenced types with structs: lifetime annotation is needed
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn structs() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
}

// Function from chapter 4
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
// In this case, the lifetime annotation was added to the compiler as it is predictable.
// The so called lifetime elision rules are applied to the function and the lifetimes are inferred.
