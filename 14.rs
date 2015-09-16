// The following iterative sequence is defined for the set of positive integers:
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

fn sequence(start: u64, count: u32) -> u32 {
  if start == 1 { return count; }

  if start % 2 == 0 {
    return sequence(start / 2, count + 1);
  } else {
    return sequence(start * 3 + 1, count + 1);
  }
}

fn solve(max: u32) -> u32 {
  let mut max_steps = 0;
  let mut max_num = 0;

  for i in 1..max {
    let steps = sequence(i as u64, 1);

    if steps >= max_steps {
      max_steps = steps;
      max_num = i;
    }
  }

  return max_num
}

fn main() {
  println!("Result: {}", solve(1_000_000));
}
