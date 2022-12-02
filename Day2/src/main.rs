use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_input<P>(file_name : P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
  let file = File::open(file_name)?;
  Ok(io::BufReader::new(file).lines())
}

enum OpponentChoice {
  Rock,
  Paper,
  Scissors
}

enum MyChoice {
  Rock,
  Paper,
  Scissors
}

fn my_choice_score(choice : &MyChoice) -> u32 {
  match choice {
    MyChoice::Rock => 1,
    MyChoice::Paper => 2,
    MyChoice::Scissors => 3
  }
}

enum RoundResult {
  Win,
  Lose,
  Draw
}

fn round_result(my_choice : MyChoice, opponent_choice : OpponentChoice) -> RoundResult
{
  match (my_choice, opponent_choice) {
    (MyChoice::Paper,    OpponentChoice::Paper)    => RoundResult::Draw,
    (MyChoice::Paper,    OpponentChoice::Rock)     => RoundResult::Win,
    (MyChoice::Paper,    OpponentChoice::Scissors) => RoundResult::Lose,
    (MyChoice::Rock,     OpponentChoice::Paper)    => RoundResult::Lose,
    (MyChoice::Rock,     OpponentChoice::Rock)     => RoundResult::Draw,
    (MyChoice::Rock,     OpponentChoice::Scissors) => RoundResult::Win,
    (MyChoice::Scissors, OpponentChoice::Paper)    => RoundResult::Win,
    (MyChoice::Scissors, OpponentChoice::Rock)     => RoundResult::Lose,
    (MyChoice::Scissors, OpponentChoice::Scissors) => RoundResult::Draw,
  }
}

fn round_result_score(result : RoundResult) -> u32 {
  match result {
    RoundResult::Win => 6,
    RoundResult::Lose => 0,
    RoundResult::Draw => 3
  }
}

fn parse_my_choice(input : &str) -> MyChoice
{
  match input {
    "X" => MyChoice::Rock,
    "Y" => MyChoice::Paper,
    "Z" => MyChoice::Scissors,
    _ => panic!("invalid input")
  }
}

fn parse_opponent_choice(input : &str) -> OpponentChoice
{
  match input {
    "A" => OpponentChoice::Rock,
    "B" => OpponentChoice::Paper,
    "C" => OpponentChoice::Scissors,
    _ => panic!("invalid input")
  }
}

fn result (my_choice : MyChoice, opponent_choice : OpponentChoice) -> u32 {
  my_choice_score(&my_choice) + round_result_score(round_result(my_choice, opponent_choice))
}

fn parse_line(line : Result<String, std::io::Error>) -> u32 {
  let line_result = line.expect("Help?");
  let opponent_choice = parse_opponent_choice(&line_result[0..1]);
  let my_choice = parse_my_choice(&line_result[2..]);
  result(my_choice, opponent_choice)
}

fn main() {
  if let Ok(lines) = read_input("input.txt") {
    let my_score : u32 = lines.map(parse_line).sum();
    println!("My score is {}", my_score);
  }
}