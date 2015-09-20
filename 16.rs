// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?

fn solve(power: u32) -> u64 {
  let mut result = vec!(1);

  for _ in 1..(power + 1) {
    let mut extra = false;

    for i in 0..result.len() {
      let x = result[i] * 2;
      result[i] = x % 10;
      if extra { result[i] += 1; }
      extra = x >= 10;
    }

    if extra { result.push(1); }
  }

  return result.iter().fold(0, |sum, &value| sum + value)
}

fn main() {
  assert_eq!(solve(15), 26);

  println!("Result: {}", solve(1000));
}
