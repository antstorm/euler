// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

fn solve(max: usize) -> u64 {
  let mut sieve = vec![true; max];
  let mut result = 0;

  // 0 and 1 are not primes
  sieve[0] = false;
  sieve[1] = false;

  for i in 2..max {
    if sieve[i] {
      let mut j = i * 2;

      while j < max {
        sieve[j] = false;

        j += i;
      }
    }
  }

  for (index, &is_prime) in sieve.iter().enumerate() {
    if is_prime { result += index }
  }

  return result as u64
}

fn main() {
  assert_eq!(solve(10), 17);
  println!("Result: {}", solve(2_000_000));
}
