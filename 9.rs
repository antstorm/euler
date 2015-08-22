// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2

// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn solve(sum: u32) -> u32 {
  for a in 1..(sum / 3) {
    for b in (a + 1)..(sum - a) {
      let c = sum - a - b;

      if a.pow(2) + b.pow(2) == c.pow(2) {
        return a * b * c;
      }
    }
  }

  return 0
}

fn main() {
  println!("Result: {}", solve(1000));
}
