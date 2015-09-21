// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and
// 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

fn number_to_string(number: u32) -> String {
  let mut tmp_number = number;

  let digits = vec!("one", "two", "three", "four", "five", "six", "seven", "eight", "nine");
  let tens = vec!("twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety");
  let teens = vec!("ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen");

  let mut left_part = String::new();
  let mut right_part = String::new();

  if tmp_number >= 1000 {
    left_part = left_part + &format!("{}thousand", digits[(tmp_number / 1000 - 1) as usize]);
    tmp_number %= 1000;
  }

  if tmp_number >= 100 {
    left_part = left_part + &format!("{}hundred", digits[(tmp_number / 100 - 1) as usize]);
    tmp_number %= 100;
  }

  if tmp_number >= 20 {
    right_part = right_part + tens[(tmp_number / 10 - 2) as usize];
    tmp_number %= 10;
  } else if tmp_number >= 10 {
    right_part = right_part + teens[(tmp_number % 10) as usize];
    tmp_number = 0;
  }

  if tmp_number > 0 {
    right_part = right_part + digits[(tmp_number - 1) as usize];
  }

  if right_part.len() > 0 && left_part.len() > 0 {
    return format!("{}and{}", left_part, right_part)
  } else if right_part.len() > 0 {
    return right_part
  } else {
    return left_part
  }
}

fn solve(max: u32) -> u32 {
  let mut result = 0;

  for i in 1..(max + 1) {
    result += number_to_string(i).len();
  }

  return result as u32
}

fn main() {
  assert_eq!(solve(5), 19);

  println!("Result: {}", solve(1000));
}
