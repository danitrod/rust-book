// - 1. Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

const LIST: [usize; 10] = [19, 22, 27, 98, 52, 42, 42, 77, 77, 77];

pub fn ex1() {
  // Mean
  let mut sum = 0;
  for numb in LIST.iter() {
    sum += numb;
  }
  let mean: f32 = sum as f32 / LIST.len() as f32;
  println!("mean: {:.2}", mean);

  // Median
  let mut sorted = LIST.to_vec();
  sorted.sort();
  let len = sorted.len();
  let median = if len % 2 != 0 {
    sorted[len / 2] as f32
  } else {
    (sorted[len / 2] as f32 + sorted[(len / 2) - 1] as f32) / 2.
  };
  println!("median: {:.2}", median);

  // Mode
  use std::collections::HashMap;

  let mut map = HashMap::new();
  let mut mode = (LIST[0], 1);
  for numb in LIST.iter() {
    let count_ref = map.entry(numb).or_insert(0);
    *count_ref += 1;
    if *count_ref > mode.1 {
      mode = (*numb, *count_ref);
    }
  }
  println!("mode: {}", mode.0);
}
