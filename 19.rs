// You are given the following information, but you may prefer to do some research for yourself.

// 1 Jan 1900 was a Monday.
// Thirty days has September,
// April, June and November.
// All the rest have thirty-one,
// Saving February alone,
// Which has twenty-eight, rain or shine.
// And on leap years, twenty-nine.
// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn is_leap_year(year: u32) -> bool {
  return (year % 1000 == 0 && year % 400 == 0) || (year % 1000 != 0 && year % 4 == 0)
}

fn solve() -> u32 {
  let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

  let mut day = 1;
  let mut week_day = 1;
  let mut month = 1;
  let mut year = 1900;

  let mut sunday_count = 0;

  while year <= 2000 && month <= 12 && day <= 31 {
    // Sundays on the first of the month starting with 1901
    if year >= 1901 && day == 1 && week_day == 7 { sunday_count += 1; }

    week_day += 1;
    if week_day > 7 { week_day = 1; }

    day += 1;
    if month == 2 && is_leap_year(year) {
      if day > 29 {
        day = 1;
        month += 1;
      }
    } else {
      if day > month_days[month - 1] {
        day = 1;
        month += 1;
      }
    }

    if month > 12 {
      month = 1;
      year += 1;
    }
  }

  return sunday_count
}

fn main() {
  println!("Result: {}", solve());
}
