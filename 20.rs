// n! means n × (n − 1) × ... × 3 × 2 × 1

// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

// Find the sum of the digits in the number 100!

fn str_digit(s: &str, n: usize) -> u8 {
  return s.chars().nth(n).unwrap().to_string().parse::<u8>().unwrap()
}

fn max(a: usize, b: usize) -> usize {
  if a > b {
    return a
  } else {
    return b
  }
}

fn str_add(a: &str, b: &str) -> String {
  let mut mem = 0;
  let mut result = String::new();

  for i in 0..max(a.len(), b.len()) {
    let mut a_digit = 0;
    let mut b_digit = 0;

    if i < a.len() { a_digit = str_digit(a, a.len() - i - 1); }
    if i < b.len() { b_digit = str_digit(b, b.len() - i - 1); }

    let local_result = a_digit + b_digit + mem;
    mem = local_result / 10;

    result = (local_result % 10).to_string() + &result;
  }

  if mem > 0 { result = mem.to_string() + &result; }

  return result
}

fn str_multiply(a: &str, b: u32) -> String {
  let mut result = "0".to_string();

  for _ in 0..b {
    result = str_add(&result, a);
  }

  return result
}

fn str_fact(n: u32) -> String {
  if n == 1 {
    return 1.to_string()
  } else {
    return str_multiply(&str_fact(n - 1), n);
  }
}

fn solve(n: u32) -> u32 {
  let mut result: u32 = 0;
  let fact = str_fact(n);

  for i in 0..fact.len() {
    result += str_digit(&fact, i) as u32;
  }

  return result
}

fn main() {
  assert_eq!(solve(10), 27);

  println!("Result: {}", solve(100));
}
