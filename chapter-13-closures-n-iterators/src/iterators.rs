pub fn iterators() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  // let sum: i32 = v1_iter.sum(); sum() consumes the iterator

  for val in v1_iter {
    // could also be for val in v1 straight up
    println!("Got: {}", val);
  }

  // Mapping iterators
  let _v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

// Using filter
#[derive(PartialEq, Debug)]
pub struct Shoe {
  size: u32,
  style: String,
}

pub fn _shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn filters_by_size() {
    let shoes = vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 13,
        style: String::from("sandal"),
      },
      Shoe {
        size: 10,
        style: String::from("boot"),
      },
    ];

    let in_my_size = _shoes_in_my_size(shoes, 10);

    assert_eq!(
      in_my_size,
      vec![
        Shoe {
          size: 10,
          style: String::from("sneaker")
        },
        Shoe {
          size: 10,
          style: String::from("boot")
        },
      ]
    );
  }
}
