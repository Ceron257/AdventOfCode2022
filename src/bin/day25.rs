use itertools::enumerate;
use utilities::read_input;

fn digit_to_decimal(input: char) -> i64 {
  match input {
    '=' => -2,
    '-' => -1,
    '0' => 0,
    '1' => 1,
    '2' => 2,
    digit => panic!("Invalid digit: '{}'", digit),
  }
}

fn digit_to_snafu(input: i64) ->char {
  match input {
    -2 => '=',
    -1 => '-',
    0 => '0',
    1 => '1',
    2 => '2',
    digit => panic!("Invalid digit: '{}'", digit),
  }
}

fn to_decimal(snafu: &str) -> i64 {
  let mut result = 0;
  let current_digit = snafu.chars().rev();

  for (index, digit) in enumerate(current_digit) {
    result += 5_i64.pow(index as u32) * digit_to_decimal(digit);
  }
  result
}

fn to_snafu(number: i64) -> String {
  let mut result = String::new();
  let mut current_number = number;
  loop {
    let rest = (current_number + 2) % 5;
    current_number = (current_number + 2) / 5;
    result.push(digit_to_snafu(rest - 2));
    if current_number == 0 {
      break;
    }
  }

  result.chars().rev().collect::<String>()
}

fn main() {
  if let Ok(lines) = read_input("inputs/day25.txt") {
    let sum = lines.iter().map(|number| to_decimal(number)).sum::<i64>();
    println!("{:?}", to_snafu(sum));
  }
  else {
      println!("Couldn't read input.");
  }
}