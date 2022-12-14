use utilities::*;

#[derive(Clone, Copy)]
enum OpponentChoice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum MyChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn my_choice_score(choice: &MyChoice) -> u32 {
    *choice as u32
}

#[derive(Clone, Copy)]
enum RoundResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn round_result_score(result: RoundResult) -> u32 {
    result as u32
}

fn round_result(my_choice: MyChoice, opponent_choice: OpponentChoice) -> RoundResult {
    match (my_choice, opponent_choice) {
        (MyChoice::Paper, OpponentChoice::Paper) => RoundResult::Draw,
        (MyChoice::Paper, OpponentChoice::Rock) => RoundResult::Win,
        (MyChoice::Paper, OpponentChoice::Scissors) => RoundResult::Lose,
        (MyChoice::Rock, OpponentChoice::Paper) => RoundResult::Lose,
        (MyChoice::Rock, OpponentChoice::Rock) => RoundResult::Draw,
        (MyChoice::Rock, OpponentChoice::Scissors) => RoundResult::Win,
        (MyChoice::Scissors, OpponentChoice::Paper) => RoundResult::Win,
        (MyChoice::Scissors, OpponentChoice::Rock) => RoundResult::Lose,
        (MyChoice::Scissors, OpponentChoice::Scissors) => RoundResult::Draw,
    }
}

fn parse_my_choice(input: &char) -> MyChoice {
    match input {
        'X' => MyChoice::Rock,
        'Y' => MyChoice::Paper,
        'Z' => MyChoice::Scissors,
        _ => panic!("invalid input"),
    }
}

fn parse_opponent_choice(input: &char) -> OpponentChoice {
    match input {
        'A' => OpponentChoice::Rock,
        'B' => OpponentChoice::Paper,
        'C' => OpponentChoice::Scissors,
        _ => panic!("invalid input"),
    }
}

fn parse_desired_outcome(input: &char) -> RoundResult {
    match input {
        'X' => RoundResult::Lose,
        'Y' => RoundResult::Draw,
        'Z' => RoundResult::Win,
        _ => panic!("invalid input"),
    }
}

enum Part {
    One,
    Two,
}

fn result(my_choice: MyChoice, round_result: RoundResult) -> u32 {
    my_choice_score(&my_choice) + round_result_score(round_result)
}

fn my_choice_part_two(opponent_choice: OpponentChoice, round_result: RoundResult) -> MyChoice {
    match (opponent_choice, round_result) {
        (OpponentChoice::Paper, RoundResult::Draw) => MyChoice::Paper,
        (OpponentChoice::Rock, RoundResult::Draw) => MyChoice::Rock,
        (OpponentChoice::Scissors, RoundResult::Draw) => MyChoice::Scissors,
        (OpponentChoice::Paper, RoundResult::Win) => MyChoice::Scissors,
        (OpponentChoice::Rock, RoundResult::Win) => MyChoice::Paper,
        (OpponentChoice::Scissors, RoundResult::Win) => MyChoice::Rock,
        (OpponentChoice::Paper, RoundResult::Lose) => MyChoice::Rock,
        (OpponentChoice::Rock, RoundResult::Lose) => MyChoice::Scissors,
        (OpponentChoice::Scissors, RoundResult::Lose) => MyChoice::Paper,
    }
}

fn parse_line(line: &String, part: Part) -> u32 {
    let opponent_choice = parse_opponent_choice(&line.chars().nth(0).expect("No character given"));
    let second_character = &line.chars().nth(2).expect("Missing second character");
    let round_result = match part {
        Part::One => round_result(parse_my_choice(second_character), opponent_choice),
        Part::Two => parse_desired_outcome(second_character),
    };
    let my_choice = match part {
        Part::One => parse_my_choice(second_character),
        Part::Two => my_choice_part_two(opponent_choice, round_result),
    };
    result(my_choice, round_result)
}

fn main() {
    match read_input("inputs/day2.txt") {
        Ok(read_lines) => {
            let my_score: u32 = read_lines
                .clone()
                .iter()
                .map(|line| parse_line(line, Part::One))
                .sum();
            println!("My score is {}", my_score);
            let my_score_part2: u32 = read_lines
                .iter()
                .map(|line| parse_line(line, Part::Two))
                .sum();
            println!("My score for part 2 is {}", my_score_part2);
        }
        Err(err) => println!("Failed to read input: {}", err),
    }
}
