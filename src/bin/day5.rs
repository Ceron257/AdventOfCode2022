use itertools::enumerate;
use lazy_static::lazy_static;
use regex::Regex;
use std::slice::Iter;
use utilities::*;

fn parse_stack_line(input: &str) -> Vec<char> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\s{3,3})\s?|(?:\[(?P<crate>[A-Z])\])\s?").unwrap();
    }
    let mut columns: Vec<char> = Vec::new();
    for captures in RE.captures_iter(input) {
        match captures.name("crate") {
            Some(group) => match group.as_str().chars().nth(0) {
                Some(crate_name) => columns.push(crate_name),
                None => continue,
            },
            None => columns.push(' '),
        }
    }
    columns
}

fn is_stack_name_line(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s(\d+\s+)+").unwrap();
    }
    RE.is_match(input)
}

#[derive(PartialEq, Debug)]
struct MoveInstruction {
    amount: usize,
    from: usize,
    to: usize,
}

type MoveInstructions = Vec<MoveInstruction>;

fn parse_move_instruction(input: &str) -> Option<MoveInstruction> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^move\s(?P<amount>\d+)\sfrom\s(?P<from>\d+)\sto\s(?P<to>\d+)").unwrap();
    }
    match RE.captures(input) {
        Some(captures) => {
            match (
                captures.name("from"),
                captures.name("to"),
                captures.name("amount"),
            ) {
                (Some(from), Some(to), Some(amount)) => match (
                    from.as_str().parse::<usize>(),
                    to.as_str().parse::<usize>(),
                    amount.as_str().parse::<usize>(),
                ) {
                    (Ok(from), Ok(to), Ok(amount)) => Some(MoveInstruction {
                        from: from,
                        to: to,
                        amount: amount,
                    }),
                    _ => None,
                },
                _ => None,
            }
        }
        None => None,
    }
}

fn parse_move_instructions(input: Iter<String>) -> Option<Vec<MoveInstruction>> {
    let mut result = MoveInstructions::new();
    for line in input {
        match parse_move_instruction(line) {
            Some(instruction) => result.push(instruction),
            None => return None,
        };
    }
    Some(result)
}

type State = Vec<Vec<char>>;

fn parse_state(mut input: Iter<String>) -> (Iter<String>, Option<State>) {
    let mut state = State::new();
    while let Some(line) = input.next() {
        if is_stack_name_line(line) {
            return (input, Some(state));
        }
        let new_items = parse_stack_line(line);
        if state.len() < new_items.len() {
            state.resize(new_items.len(), Vec::new());
        }
        for (index, new_item) in enumerate(new_items) {
            if new_item == ' ' {
                continue;
            }
            match state.get_mut(index) {
                Some(stack) => stack.insert(0, new_item),
                None => return (input, None),
            }
        }
    }
    (input, None)
}

fn apply_instruction(mut input: State, instruction: MoveInstruction) -> Result<State, String> {
    let mut crates;
    match input.get_mut(instruction.from - 1) {
        Some(from_state) => {
            crates = from_state.split_off(from_state.len().saturating_sub(instruction.amount));
            crates.reverse();
        }
        _ => return Err("Can't apply instruction to this state.".to_string()),
    }

    match input.get_mut(instruction.to - 1) {
        Some(to_state) => {
            to_state.append(&mut crates);
            Ok(input)
        }
        _ => Err("Can't apply instruction to this state.".to_string()),
    }
}

fn main() {
    match read_input("inputs/day5.txt") {
        Ok(lines) => {
            let (mut iter, state) = parse_state(lines.iter());
            iter.next(); // skip empty line
            match state {
                Some(mut state) => match parse_move_instructions(iter) {
                    Some(instructions) => {
                        for instruction in instructions {
                            match apply_instruction(state, instruction) {
                                Ok(new_state) => state = new_state,
                                Err(msg) => panic!("Couldn't apply state to state: {}", msg),
                            }
                        }
                        let mut solution = String::new();
                        for stack in state {
                            if !stack.is_empty() {
                                solution.push(*stack.last().unwrap());
                            }
                        }
                        println!("My solution would be {}.", solution);
                    }
                    None => println!("Couldn't parse move instructions."),
                },
                None => println!("Couldn't parse state."),
            }
        }
        Err(err) => println!("Unable to read input: {}", err),
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn parse_stack_lines() {
        assert_eq!(
            parse_stack_line("            [J] [Z] [G]            "),
            [' ', ' ', ' ', 'J', 'Z', 'G', ' ', ' ', ' '].to_vec()
        );
        assert_eq!(
            parse_stack_line("[R]         [Q] [V] [B] [G] [J]    "),
            ['R', ' ', ' ', 'Q', 'V', 'B', 'G', 'J', ' '].to_vec()
        );
        assert_eq!(
            parse_stack_line("[W] [W]     [N] [L] [V] [W] [C]    "),
            ['W', 'W', ' ', 'N', 'L', 'V', 'W', 'C', ' '].to_vec()
        );
        assert_eq!(
            parse_stack_line("[F] [Q]     [T] [G] [C] [T] [T] [W]"),
            ['F', 'Q', ' ', 'T', 'G', 'C', 'T', 'T', 'W'].to_vec()
        );
        assert_eq!(
            parse_stack_line("[S] [S] [B] [D] [F] [L] [Z] [N] [L]"),
            ['S', 'S', 'B', 'D', 'F', 'L', 'Z', 'N', 'L'].to_vec()
        );
    }

    #[test]
    fn test_is_stack_name_line() {
        assert_eq!(
            is_stack_name_line(" 1   2   3   4   5   6   7   8   9 "),
            true
        );
        assert_eq!(
            is_stack_name_line("[S] [S] [B] [D] [F] [L] [Z] [N] [L]"),
            false
        );
        assert_eq!(is_stack_name_line(""), false);
        assert_eq!(is_stack_name_line("move 4 from 2 to 1"), false);
    }

    #[test]
    fn test_parse_move_instruction() {
        assert_eq!(
            parse_move_instruction("move 15 from 6 to 4"),
            Some(MoveInstruction {
                amount: 15,
                from: 6,
                to: 4
            })
        );
        assert_eq!(
            parse_move_instruction("move 7 from 5 to 6"),
            Some(MoveInstruction {
                amount: 7,
                from: 5,
                to: 6
            })
        );
    }

    #[test]
    fn test_parse_state() {
        let input = [
            "[T]".to_string(),
            "[S] [A]".to_string(),
            " 1   2 ".to_string(),
        ]
        .to_vec();
        let (_, state) = parse_state(input.iter());
        assert!(state.is_some());
        assert_eq!(
            state.unwrap(),
            [['S', 'T'].to_vec(), ['A'].to_vec()].to_vec()
        );
        let input = [
            "    [T]".to_string(),
            "[S] [A]".to_string(),
            " 1   2 ".to_string(),
        ]
        .to_vec();
        let (_, state) = parse_state(input.iter());
        assert!(state.is_some());
        assert_eq!(
            state.unwrap(),
            [['S'].to_vec(), ['A', 'T'].to_vec()].to_vec()
        );
        let input = [
            "    [T]    ".to_string(),
            "[S] [A] [B]".to_string(),
            " 1   2   3 ".to_string(),
        ]
        .to_vec();
        let (_, state) = parse_state(input.iter());
        assert!(state.is_some());
        assert_eq!(
            state.unwrap(),
            [['S'].to_vec(), ['A', 'T'].to_vec(), ['B'].to_vec()].to_vec()
        );
    }

    #[test]
    fn test_apply_instruction() {
        let state = [['S', 'T'].to_vec(), ['A'].to_vec()].to_vec();

        assert_eq!(
            apply_instruction(
                state,
                MoveInstruction {
                    amount: 2,
                    from: 1,
                    to: 2
                }
            ),
            Ok([[].to_vec(), ['A', 'T', 'S'].to_vec()].to_vec())
        )
    }
}
