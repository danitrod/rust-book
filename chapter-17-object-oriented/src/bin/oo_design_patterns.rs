// First try: doing as we would with other oo languages
mod blog {
  const MIN_APPROVALS: u8 = 2;

  pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
  }

  impl Post {
    pub fn new() -> Post {
      Post {
        state: Some(Box::new(Draft {})),
        content: String::new(),
      }
    }

    pub fn add_text(&mut self, text: &str) {
      if self.state.as_ref().unwrap().is_editable() {
        self.content.push_str(text);
      }
    }

    pub fn content(&self) -> &str {
      self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
      if let Some(s) = self.state.take() {
        self.state = Some(s.request_review())
      }
    }

    pub fn approve(&mut self) {
      if let Some(s) = self.state.take() {
        self.state = Some(s.approve())
      }
    }

    pub fn reject(&mut self) {
      if let Some(s) = self.state.take() {
        self.state = Some(s.reject())
      }
    }
  }

  trait State {
    fn is_editable(&self) -> bool {
      false
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    #[allow(unused_variables)]
    fn content<'a>(&self, post: &'a Post) -> &'a str {
      ""
    }
  }

  struct Draft {}

  impl State for Draft {
    fn is_editable(&self) -> bool {
      true
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
      Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
      self
    }
  }

  struct PendingReview {
    approvals: u8,
  }

  impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
      if self.approvals >= MIN_APPROVALS - 1 {
        Box::new(Published {})
      } else {
        Box::new(PendingReview {
          approvals: self.approvals + 1,
        })
      }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
      Box::new(Draft {})
    }
  }

  struct Published {}

  impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
      &post.content
    }
  }
}

// Rethinking the state pattern for rust
mod rust_blog {
  pub const MIN_APPROVALS: u8 = 2;

  pub struct Post {
    content: String,
  }

  pub struct DraftPost {
    content: String,
  }

  pub struct PendingReviewPost {
    content: String,
    approvals: u8,
  }

  impl Post {
    pub fn new() -> DraftPost {
      DraftPost {
        content: String::new(),
      }
    }

    pub fn content(&self) -> &str {
      &self.content
    }
  }

  impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
      self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
      PendingReviewPost {
        content: self.content,
        approvals: 0,
      }
    }
  }

  // pub enum ApprovePostResult {
  //   Approved(Post),
  //   Awaiting(PendingReviewPost),
  // }

  // impl ApprovePostResult {
  //   pub fn approve(self) ->
  // }

  impl PendingReviewPost {
    pub fn get_approvals(&self) -> u8 {
      self.approvals
    }

    pub fn approve(&mut self) {
      self.approvals += 1;
    }

    pub fn publish(self) -> Result<Post, &'static str> {
      if self.approvals >= MIN_APPROVALS {
        Ok(Post {
          content: self.content,
        })
      } else {
        Err("Not enough approvals!")
      }
    }

    pub fn reject(self) -> DraftPost {
      DraftPost {
        content: self.content,
      }
    }
  }
}

// old main
// fn main() {
// use blog::Post;
//   let start = std::time::Instant::now();
//   let mut post = Post::new();

//   post.add_text("I ate a salad for lunch today");
//   assert_eq!("", post.content());

//   post.request_review();
//   assert_eq!("", post.content());
//   post.add_text("I ate a salad for lunch today"); // Not added: is only editable when in Draft state

//   post.reject();
//   post.request_review();

//   post.approve();
//   post.approve();
//   assert_eq!("I ate a salad for lunch today", post.content());
//   println!("Time: {:?}", start.elapsed());
// }

fn main() {
  use rust_blog::{Post, MIN_APPROVALS};
  let start = std::time::Instant::now();
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");

  let post = post.request_review();
  // Can't add text anymore
  // post.add_text("I ate a salad for lunch today"); // Not added: is only editable when in Draft state

  let post = post.reject();
  let mut post = post.request_review();

  while post.get_approvals() < MIN_APPROVALS {
    post.approve();
  }
  let post = post.publish().unwrap();
  assert_eq!("I ate a salad for lunch today", post.content());
  println!("Time: {:?}", start.elapsed());
}
