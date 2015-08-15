// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385

// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 552 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn sum_of_squares(max: u64) -> u64 {
  let mut result = 0;

  for x in 1..(max + 1) {
    result += x.pow(2);
  }

  return result
}

fn square_of_sum(max: u64) -> u64 {
  let mut result = 0;

  for x in 1..(max + 1) {
    result += x;
  }

  return result.pow(2)
}

fn solve(max: u64) -> u64 {
  return square_of_sum(max) - sum_of_squares(max)
}

fn main() {
  assert_eq!(solve(10), 2640);
  println!("Result: {}", solve(100));
}
