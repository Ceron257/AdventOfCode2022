use lazy_static::lazy_static;
use regex::Regex;
use utilities::read_input;

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Noop = 1,
    Addx = 2,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone)]
struct Screen {
    pixels: Vec<char>,
}

impl Screen {
    fn new() -> Screen {
        let mut pixels = Vec::new();
        pixels.resize(40 * 6, '.');
        Screen { pixels: pixels }
    }

    fn set_pixel(&mut self, x: i32, y: i32, crt_position: i32) {
        let index = crt_position + 40 * y;
        self.pixels[index as usize] = if is_pixel_active(crt_position, x) {
            '#'
        } else {
            ' '
        };
    }

    fn print(&self) {
        for line in self
            .pixels
            .chunks(40)
            .map(|window| window.iter().map(<char>::to_string).collect::<String>())
        {
            println!("{}", line);
        }
    }
}

fn cycle_to_crt_line(cycle: &usize) -> i32 {
    ((cycle.saturating_sub(1) / 40) % 6) as i32
}

fn cycle_to_crt_position(cycle: &usize) -> i32 {
    (cycle.saturating_sub(1) % 40) as i32
}

fn is_pixel_active(crt_position: i32, sprite_position: i32) -> bool {
    (sprite_position - crt_position).abs() < 2
}

#[derive(Debug, Clone)]
struct DisplaySystem {
    program: Vec<Instruction>,
    register_x: i32,
    screen: Screen,
}

impl DisplaySystem {
    fn new(program: Vec<Instruction>) -> DisplaySystem {
        DisplaySystem {
            program,
            register_x: 1,
            screen: Screen::new(),
        }
    }
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
            self.screen.set_pixel(
                self.register_x,
                cycle_to_crt_line(&cycle),
                cycle_to_crt_position(&cycle),
            );
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

        let mut display_system = DisplaySystem::new(program.clone());
        let signal_strength: i32 = display_system
            .clone()
            .execute_cycles(220, |cycle| cycle % 40 == 20)
            .iter()
            .sum();
        println!("The signal strength sum is {:?}", signal_strength);

        display_system.execute_cycles(
            program.iter().map(|i| i.operator.clone() as usize).sum(),
            |_| false,
        );
        display_system.screen.print();
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

        let mut display_system = DisplaySystem::new(program);

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

        let mut display_system = DisplaySystem::new(program);

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

    #[test]
    fn test_cycle_to_crt_position() {
        assert_eq!(cycle_to_crt_position(&1), 0);
        assert_eq!(cycle_to_crt_position(&6), 5);
        assert_eq!(cycle_to_crt_position(&40), 39);
        assert_eq!(cycle_to_crt_position(&41), 0);
        assert_eq!(cycle_to_crt_position(&80), 39);
        assert_eq!(cycle_to_crt_position(&201), 0);
        assert_eq!(cycle_to_crt_position(&240), 39);
    }

    #[test]
    fn test_is_pixel_active() {
        assert_eq!(is_pixel_active(1, -1), false);
        assert_eq!(is_pixel_active(1, 0), true);
        assert_eq!(is_pixel_active(1, 1), true);
        assert_eq!(is_pixel_active(1, 2), true);
        assert_eq!(is_pixel_active(1, 3), false);

        assert_eq!(is_pixel_active(40, 38), false);
        assert_eq!(is_pixel_active(40, 39), true);
        assert_eq!(is_pixel_active(40, 40), true);
        assert_eq!(is_pixel_active(40, 41), true);
        assert_eq!(is_pixel_active(40, 42), false);
    }

    #[test]
    fn test_cycle_to_crt_line() {
        assert_eq!(cycle_to_crt_line(&1), 0);
        assert_eq!(cycle_to_crt_line(&40), 0);
        assert_eq!(cycle_to_crt_line(&41), 1);
        assert_eq!(cycle_to_crt_line(&80), 1);
        assert_eq!(cycle_to_crt_line(&201), 5);
        assert_eq!(cycle_to_crt_line(&240), 5);
        assert_eq!(cycle_to_crt_line(&241), 0);
    }

    #[test]
    fn test_screen() {
        let lines = read_input("inputs/day10-example.txt");
        assert!(lines.is_ok());
        let program = parse_program(lines.unwrap());

        let mut display_system = DisplaySystem::new(program);
        display_system.execute_cycles(40, |_| true);

        assert_eq!(display_system.screen.pixels[0], '#');
        assert_eq!(display_system.screen.pixels[1], '#');
        assert_eq!(display_system.screen.pixels[2], '.');
        assert_eq!(display_system.screen.pixels[3], '.');
        assert_eq!(display_system.screen.pixels[4], '#');
        assert_eq!(display_system.screen.pixels[5], '#');
        assert_eq!(display_system.screen.pixels[6], '.');
        assert_eq!(display_system.screen.pixels[7], '.');
        assert_eq!(display_system.screen.pixels[8], '#');
        assert_eq!(display_system.screen.pixels[9], '#');
        assert_eq!(display_system.screen.pixels[10], '.');
        assert_eq!(display_system.screen.pixels[11], '.');
        assert_eq!(display_system.screen.pixels[12], '#');
        assert_eq!(display_system.screen.pixels[13], '#');
        assert_eq!(display_system.screen.pixels[14], '.');
        assert_eq!(display_system.screen.pixels[15], '.');
        assert_eq!(display_system.screen.pixels[16], '#');
        assert_eq!(display_system.screen.pixels[17], '#');
        assert_eq!(display_system.screen.pixels[18], '.');
        assert_eq!(display_system.screen.pixels[19], '.');
        assert_eq!(display_system.screen.pixels[20], '#');
    }
}
