// The Fibonacci sequence is defined by the recurrence relation:

// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:

// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.

// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

fn max(a: usize, b: usize) -> usize {
  if a > b {
    return a
  } else {
    return b
  }
}

fn str_digit(s: &str, n: usize) -> u8 {
  return s.chars().nth(n).unwrap().to_string().parse::<u8>().unwrap()
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

fn solve(max_digits: usize) -> u32 {
  let mut previous = "1".to_string();
  let mut current = "1".to_string();
  let mut new_value:String;
  let mut i = 2;

  while current.len() < max_digits {
    new_value = str_add(&current, &previous);
    previous = current;
    current = new_value;

    i += 1;
  }

  return i
}

fn main() {
  assert_eq!(solve(3), 12);

  println!("Result: {}", solve(1000));
}
