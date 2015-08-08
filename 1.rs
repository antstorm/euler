// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn solve(max: u32, multiples: &[u32]) -> u32 {
  let mut result = 0;

  for i in 1..max {
    if let Some(_) = multiples.iter().find(|&x| i % *x == 0) {
      result += i;
    }
  }

  return result;
}

fn main() {
  assert_eq!(solve(10, &[3, 5]), 23);

  println!("Result: {}", solve(1000, &[3, 5]));
}
