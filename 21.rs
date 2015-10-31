// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

// Evaluate the sum of all the amicable numbers under 10000.

fn divisors_of(n: u32) -> Vec<u32> {
  let mut divisors = vec!(1);

  for i in 2..(n / 2) + 1 {
    if n % i == 0 { divisors.push(i); }
  }

  return divisors
}

fn are_amicable(a: u32, b: u32, cache: &mut [u32;10000]) -> bool {
  let mut sum_a = cache[a as usize];
  let mut sum_b = cache[b as usize];

  if sum_a == 0 {
    sum_a = divisors_of(a).iter().fold(0, |mut sum, &val| { sum += val; sum });
    cache[a as usize] = sum_a;
  }

  if sum_b == 0 {
    sum_b = divisors_of(b).iter().fold(0, |mut sum, &val| { sum += val; sum });
    cache[b as usize] = sum_b;
  }

  return a != b && sum_a == b && sum_b == a
}

fn solve(max: u32) -> u32 {
  let mut cache: [u32;10000] = [0; 10000];
  let mut amicable = vec!();

  for i in 1..max {
    for j in i..max {
      if are_amicable(i, j, &mut cache) {
        amicable.push(i);
        amicable.push(j);
      }
    }
  }

  return amicable.iter().fold(0, |mut sum, &val| { sum += val; sum });
}

fn main() {
  println!("Result: {}", solve(10000));
}
