// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

fn is_prime(num: u64) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false
    }
  }

  return true
}

fn solve(num: u64) -> u64 {
  let mut count = 1;
  let mut result = 1;

  while count <= num {
    let mut i = result + 1;
    while !is_prime(i) { i += 1; }

    result = i;
    count += 1;
  }

  return result
}

fn main() {
  assert_eq!(solve(6), 13);
  println!("Result: {}", solve(10_001));
}
