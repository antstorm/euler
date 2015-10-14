// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn is_prime(num: u64) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false
    }
  }

  return true
}

fn solve(max: u64) -> u64 {
  let mut new_max = max;
  let mut biggest_prime_factor = 0;
  let mut i = 1;

  while i < new_max {
    if max % i == 0 {
      new_max = max / i;
      
      if is_prime(i) { biggest_prime_factor = i; }
    }

    i += 1;
  }

  return biggest_prime_factor;
}

fn main() {
  println!("Result: {}", solve(600_851_475_143));
}
