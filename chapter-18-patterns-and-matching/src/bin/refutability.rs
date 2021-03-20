#[allow(unused_variables)]
#[allow(irrefutable_let_patterns)]

fn main() {
  // Irrefutable patterns: cannot fail no matter what expressions come
  let x = Some(1);

  // Refutable patterns: can fail depending on the expression

  // fails in at least one case
  if let Some(x) = x {};

  // if let and while let can accept refutable patterns

  // This compiles, but throws a warning, as it is an irrefutable pattern
  if let x = 5 {};
}
