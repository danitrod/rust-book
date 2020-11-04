pub struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

pub fn main() {
  let mut c = Counter::new();
  assert_eq!(c.next(), Some(1));
  assert_eq!(c.next(), Some(2));
  assert_eq!(c.next(), Some(3));
  assert_eq!(c.next(), Some(4));
  assert_eq!(c.next(), Some(5));
  assert_eq!(c.next(), None);

  let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
  assert_eq!(18, sum);
}
