// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn divisible_by(num: u32, max: u32) -> bool {
  for x in 1..(max + 1) {
    if num % x != 0 {
      return false;
    }
  }

  return true
}

fn solve(max: u32) -> u32 {
  let mut result = 0;

  loop {
    // the result must be divisable by the max divider
    result += max;

    if divisible_by(result, max) {
      return result;
    }
  }
}

fn main() {
  println!("Result: {}", solve(20));
}
