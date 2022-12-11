use lazy_static::lazy_static;
use regex::Regex;
use utilities::read_input;

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Noop = 1,
    Addx = 2,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    operator: Operator,
    operand: Option<i32>,
}

fn parse_operator(input: &str) -> Option<Operator> {
    match input {
        "noop" => Some(Operator::Noop),
        "addx" => Some(Operator::Addx),
        _ => None,
    }
}

fn parse_instruction<T>(input: T) -> Option<Instruction>
where
    T: Into<String>,
{
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<operator>addx|noop)\s?(?P<operand>-?\d+)?").unwrap();
    };
    let input_string = input.into();
    let captures = RE.captures(input_string.as_str())?;
    let operator = parse_operator(captures.name("operator")?.as_str())?;
    let operand = match captures.name("operand") {
        Some(capture) => match capture.as_str().parse::<i32>() {
            Ok(operand) => Some(operand),
            Err(_) => None,
        },
        None => None,
    };
    Some(Instruction { operator, operand })
}

struct DisplaySystem {
    program: Vec<Instruction>,
    register_x: i32,
}

impl DisplaySystem {
    fn execute_cycles(
        &mut self,
        cylce_count: usize,
        should_measure_signal_strength: fn(&usize) -> bool,
    ) -> Vec<i32> {
        if self.program.is_empty() {
            panic!("No instructions to execute!");
        }
        let mut result = Vec::new();
        let mut instruction_iter = self.program.iter();
        let mut current_instruction = instruction_iter.next();
        let mut cylce_to_next_instruction = current_instruction.unwrap().operator.clone() as i32;

        for cycle in 1..=cylce_count {
            if should_measure_signal_strength(&cycle) {
                result.push(self.register_x * cycle as i32);
            }
            cylce_to_next_instruction -= 1;
            if cylce_to_next_instruction == 0 {
                if current_instruction.is_none() {
                    panic!("Not enough instructions to execute {} cycles!", cylce_count);
                }
                if current_instruction.unwrap().operator == Operator::Addx {
                    self.register_x += current_instruction.unwrap().operand.unwrap();
                }
                current_instruction = instruction_iter.next();
                if let Some(current_instruction) = current_instruction {
                    cylce_to_next_instruction = current_instruction.operator.clone() as i32;
                }
            }
        }

        result
    }
}

fn parse_program(input: Vec<String>) -> Vec<Instruction> {
    input
        .iter()
        .map(|line| {
            if let Some(instruction) = parse_instruction(line) {
                instruction
            } else {
                panic!("Invalid instruction: {}", line);
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    if let Ok(lines) = read_input("inputs/day10.txt") {
        let program = parse_program(lines);

        let mut display_system = DisplaySystem {
            program,
            register_x: 1,
        };
        let signal_strength: i32 = display_system
            .execute_cycles(220, |cycle| cycle % 40 == 20)
            .iter()
            .sum();
        println!("The signal strength sum is {:?}", signal_strength);
    } else {
        println!("Couldn't read input!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction(""), None);
        assert_eq!(
            parse_instruction("noop"),
            Some(Instruction {
                operator: Operator::Noop,
                operand: None
            })
        );
        assert_eq!(
            parse_instruction("addx 3"),
            Some(Instruction {
                operator: Operator::Addx,
                operand: Some(3)
            })
        );
        assert_eq!(
            parse_instruction("addx -5"),
            Some(Instruction {
                operator: Operator::Addx,
                operand: Some(-5)
            })
        );
    }

    #[test]
    fn test_display_system() {
        let program = vec![
            parse_instruction("noop").unwrap(),
            parse_instruction("addx 3").unwrap(),
            parse_instruction("addx -5").unwrap(),
        ];

        let mut display_system = DisplaySystem {
            program,
            register_x: 1,
        };

        let signal_strengths = display_system.execute_cycles(5, |_cycle| true);

        assert_eq!(signal_strengths[0], 1 * 1);
        assert_eq!(signal_strengths[1], 2 * 1);
        assert_eq!(signal_strengths[2], 3 * 1);
        assert_eq!(signal_strengths[3], 4 * 4);
        assert_eq!(signal_strengths[4], 5 * 4);
    }

    #[test]
    fn test_display_system_huge_example() {
        let lines = read_input("inputs/day10-example.txt");
        assert!(lines.is_ok());
        let program = parse_program(lines.unwrap());

        let mut display_system = DisplaySystem {
            program,
            register_x: 1,
        };

        let signal_strengths = display_system
            .execute_cycles(220, |cycle| cycle % 40 == 20)
            .iter()
            .map(|v| *v)
            .collect::<Vec<_>>();

        assert_eq!(signal_strengths[0], 420);
        assert_eq!(signal_strengths[1], 1140);
        assert_eq!(signal_strengths[2], 1800);
        assert_eq!(signal_strengths[3], 2940);
        assert_eq!(signal_strengths[4], 2880);
        assert_eq!(signal_strengths[5], 3960);
    }
}
