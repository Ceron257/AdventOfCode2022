use lazy_static::lazy_static;
use regex::Regex;
use std::slice::Iter;
use utilities::read_input;

fn parse_starting_items(input: &str) -> Option<Vec<usize>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+Starting items:\s(?P<items>(?:\d+,?\s?)+)").unwrap();
    };
    let mut result = Vec::new();
    let captures = RE.captures(input)?;
    let items_strings = captures
        .name("items")?
        .as_str()
        .split(",")
        .map(|f| f.trim());
    for item in items_strings {
        result.push(item.parse::<usize>().ok()?);
    }
    Some(result)
}

#[derive(Debug, PartialEq)]
enum Operand {
    LastValue,
    Number(i32),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Multiplication(Operand),
    Addition(Operand),
}

fn parse_operation(input: &str) -> Option<Operation> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"\s+Operation:\snew\s=\sold\s(?P<operator>[*+])\s(?P<operand>old|\d+)")
                .unwrap();
    };
    let captures = RE.captures(input)?;
    let operand = match captures.name("operand")?.as_str() {
        "old" => Some(Operand::LastValue),
        value => Some(Operand::Number(value.parse::<i32>().ok()?)),
    }?;
    let operator = match captures.name("operator")?.as_str() {
        "*" => Operation::Multiplication(operand),
        "+" => Operation::Addition(operand),
        _ => panic!("Unknown operator!"),
    };
    Some(operator)
}

fn parse_test_condition(input: &str) -> Option<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+Test:\sdivisible\sby\s(?P<divisor>\d+)").unwrap();
    };
    Some(
        RE.captures(input)?
            .name("divisor")?
            .as_str()
            .parse::<i32>()
            .ok()?,
    )
}

fn parse_test_branch(input: &str) -> Option<(bool, i32)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"\s+If\s(?P<result>true|false):\sthrow to monkey\s(?P<monkey>\d+)")
                .unwrap();
    };
    let captures = RE.captures(input)?;
    let when = captures.name("result")?.as_str().parse::<bool>().ok()?;
    let monkey = captures.name("monkey")?.as_str().parse::<i32>().ok()?;
    Some((when, monkey))
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_condition: i32,
    test_branches: [(bool, i32); 2],
}

fn parse_monkey(input: &mut Iter<String>) -> Option<Monkey> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Monkey\s(?P<monkey>\d+):").unwrap();
    };
    if RE.captures(input.next()?).is_none() {
        panic!("Expected monkey!");
    }
    let starting_items = parse_starting_items(input.next()?)?;
    let operation = parse_operation(input.next()?)?;
    let test_condition = parse_test_condition(input.next()?)?;
    let test_branches = [
        parse_test_branch(input.next()?)?,
        parse_test_branch(input.next()?)?,
    ];
    Some(Monkey {
        items: starting_items,
        operation,
        test_condition,
        test_branches,
    })
}

fn parse_monkeys(input: &mut Iter<String>) -> Option<Vec<Monkey>> {
    let mut result = Vec::new();
    loop {
        let monkey = parse_monkey(input)?;
        result.push(monkey);
        if input.next().is_none() {
            break;
        }
    }
    Some(result)
}

fn main() {
    if let Ok(lines) = read_input("inputs/day11.txt") {
        println!("{:?}", parse_monkeys(&mut lines.iter()));
    } else {
        println!("Couldn't read input.");
    }
}

#[cfg(test)]
pub mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse_starting_items() {
        assert_eq!(parse_starting_items("  Starting items: 80"), Some(vec![80]));
        assert_eq!(
            parse_starting_items("  Starting items: 75, 83, 74"),
            Some(vec![75, 83, 74])
        );
        assert_eq!(
            parse_starting_items("  Starting items: 86, 67, 61, 96, 52, 63, 73"),
            Some(vec![86, 67, 61, 96, 52, 63, 73])
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("  Operation: new = old * 5"),
            Some(Operation::Multiplication(crate::Operand::Number(5)))
        );
        assert_eq!(
            parse_operation("  Operation: new = old + 7"),
            Some(Operation::Addition(crate::Operand::Number(7)))
        );
        assert_eq!(
            parse_operation("  Operation: new = old * old"),
            Some(Operation::Multiplication(crate::Operand::LastValue))
        );
    }

    #[test]
    fn parse_parse_test_condition() {
        assert_eq!(parse_test_condition("  Test: divisible by 2"), Some(2));
        assert_eq!(parse_test_condition("  Test: divisible by 11"), Some(11));
    }

    #[test]
    fn test_parse_test_branch() {
        assert_eq!(
            parse_test_branch("    If true: throw to monkey 4"),
            Some((true, 4))
        );
        assert_eq!(
            parse_test_branch("    If false: throw to monkey 0"),
            Some((false, 0))
        );
    }

    #[test]
    fn test_parse_monkey() {
        let lines = read_input("inputs/day11.txt");
        assert!(lines.is_ok());

        let monkey = parse_monkey(&mut lines.unwrap().iter());

        assert_eq!(
            monkey,
            Some(Monkey {
                items: vec![80],
                operation: Operation::Multiplication(Operand::Number(5)),
                test_condition: 2,
                test_branches: [(true, 4), (false, 3)]
            })
        );
    }
}
