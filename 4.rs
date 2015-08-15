// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(x: u32) -> bool {
  let s = x.to_string();

  return s == s.chars().rev().collect::<String>()
}

fn solve() -> u32 {
  let range = 100..999;
  let mut biggest_palindrome = 0;

  for i in range.clone() {
    for j in range.clone() {
      let candidate = i * j;

      if candidate > biggest_palindrome && is_palindrome(candidate) {
        biggest_palindrome = candidate;
      }
    }
  }

  return biggest_palindrome
}

fn main() {
  println!("Result: {}", solve());
}
