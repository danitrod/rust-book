use adder;

mod common;

#[test]
fn should_add_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
}
