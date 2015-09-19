// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?

use std::collections::HashMap;

fn cache_key_for(start: u32, order: u32) -> String {
  return format!("{}-{}", start, order)
}

fn calculate(start: u32, order: u32, cache: &mut HashMap<String, u64>) -> u64 {
  if cache.contains_key(&cache_key_for(start, order)) {
    return *cache.get(&cache_key_for(start, order)).unwrap()
  } else if order == 0 {
    return start as u64
  } else {
    let mut result:u64 = 0;

    for i in 1..(start + 1) {
      result += calculate(i, order - 1, cache);
    }

    cache.insert(cache_key_for(start, order), result);

    return result
  }
}

fn solve(side: u32) -> u64 {
  let mut cache = HashMap::new();

  return calculate(side + 1, side - 1, &mut cache)
}

fn main() {
  assert_eq!(solve(2), 6);

  println!("Result: {}", solve(20));
}
