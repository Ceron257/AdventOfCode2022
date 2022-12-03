use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn read_input<P>(file_name : P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
  let file = File::open(file_name)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  if let Ok(lines) = read_input("inputs/day1.txt") {
    let mut current_sum : u32 = 0;
    let mut sums : Vec<u32> = Vec::new();
    for line_result in lines {
      let line = line_result.expect("Cannot read line");
      if line.trim().is_empty() {
        sums.push(current_sum);
        current_sum = 0;
      }
      else {
          current_sum += line.parse::<u32>().expect("Cannot parse.");
      }
    }
    sums.sort();
    println!("Max = {}", sums.iter().max().unwrap());
    let top_three : u32 = sums.iter().rev().take(3).sum();
    println!("Part 2 = {}", top_three);
  }
}