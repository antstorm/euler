// A permutation is an ordered arrangement of objects.
// For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

// 012   021   102   120   201   210

// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

static mut permutations:u32 = 0;

fn permutate(numbers: Vec<u8>, len: u8) -> Option<Vec<u8>> {
  if unsafe { permutations == 1 } {
    return Some(numbers)
  }

  if len > 1 {
    let mut local_numbers:Vec<u8>;
    let from = (numbers.len() as u8) - len;
    let to = numbers.len() as u8;

    for i in from..to {
      local_numbers = numbers.to_vec();

      if i != from {
        let tmp = local_numbers.remove(i as usize);
        local_numbers.insert(from as usize, tmp);
      }

      let result = permutate(local_numbers, len - 1);

      if result.is_some() {
        return result
      }
    }
  } else {
    unsafe { permutations -= 1; }
  }

  return None
}

// Unfortunatelly this is not a very good solution, because of the global variable use
// TODO: rewrite this using a mutable pointer
fn solve(count: u8, n: u32) -> Vec<u8> {
  let mut numbers:Vec<u8> = vec!();

  for i in 0..count { numbers.push(i as u8); }

  unsafe { permutations = n; }

  return permutate(numbers, count).unwrap()
}

fn main() {
  assert_eq!(solve(3, 1), [0, 1, 2]);
  assert_eq!(solve(3, 2), [0, 2, 1]);
  assert_eq!(solve(3, 3), [1, 0, 2]);
  assert_eq!(solve(3, 4), [1, 2, 0]);
  assert_eq!(solve(3, 5), [2, 0, 1]);
  assert_eq!(solve(3, 6), [2, 1, 0]);

  println!("Result: {:?}", solve(10, 1000000));
}
