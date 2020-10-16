use std::fmt::Debug;
use std::fmt::Display;

trait Summary {
  // Force each type to implement the method
  fn summarize_author(&self) -> String;

  // Implement default method
  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// Use default trait implementation
impl Summary for NewsArticle {
  // summarize has default implementation as it is ommited

  fn summarize_author(&self) -> String {
    self.author.clone()
  }
}

// Specify trait method implementation
// impl Summary for NewsArticle {
//   fn summarize(&self) -> String {
//     format!("{}, by {} ({})", self.headline, self.author, self.location)
//   }
// }

// Implement a method for any type who has a specific trait
trait ToString2 {
  fn to_string2(&self) -> String;
}

impl<T: Display> ToString2 for T {
  fn to_string2(&self) -> String {
    self.to_string()
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

// Using traits on functions that use generic types
fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// Same as above:
// pub fn notify<T: Summary>(item: &T) {
//   println!("Breaking news! {}", item.summarize());
// }

// Multiple traits
fn notify2<T: Summary + Display>(item: &T) {}

// Alternate sintax with 'where':
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
  5
}
// the above is equal to this
fn _some_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{
  5
}

// Returning a generic type with trait
// fn returns_summarizable() -> impl Summary {
//   Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from("of course, as you probably already know, people"),
//     reply: false,
//     retweet: false,
//   }
// }

// Only one type that implement a trait can be returned. This doesnt compile:
// fn returns_summarizable(switch: bool) -> impl Summary {
//   if switch {
//     NewsArticle {
//       headline: String::from("Penguins win the Stanley Cup Championship!"),
//       location: String::from("Pittsburgh, PA, USA"),
//       author: String::from("Iceburgh"),
//       content: String::from(
//         "The Pittsburgh Penguins once again are the best \
//                hockey team in the NHL.",
//       ),
//     }
//   }
//   Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from("of course, as you probably already know, people"),
//     reply: false,
//     retweet: false,
//   }
// }

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
    ),
  };

  println!("New article available! {}", article.summarize());
  println!("1 new tweet: {}", tweet.summarize());
}
