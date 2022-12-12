use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::slice::Iter;
use utilities::{lcm, read_input};

fn parse_starting_items(input: &str) -> Option<VecDeque<usize>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+Starting items:\s(?P<items>(?:\d+,?\s?)+)").unwrap();
    };
    let mut result = VecDeque::new();
    let captures = RE.captures(input)?;
    let items_strings = captures
        .name("items")?
        .as_str()
        .split(",")
        .map(|f| f.trim());
    for item in items_strings {
        result.push_back(item.parse::<usize>().ok()?);
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
    items: VecDeque<usize>,
    operation: Operation,
    test_condition: i32,
    test_branches: [(bool, i32); 2],
    inspection_count: usize,
}

impl Monkey {
    fn inspect_item(&self, item: usize) -> usize {
        match &self.operation {
            Operation::Addition(operand) => {
                item + match operand {
                    Operand::LastValue => item,
                    Operand::Number(num) => *num as usize,
                }
            }
            Operation::Multiplication(operand) => {
                item * match operand {
                    Operand::LastValue => item,
                    Operand::Number(num) => *num as usize,
                }
            }
        }
    }

    fn test_item(&self, item: usize) -> bool {
        (item as i32 % self.test_condition) == 0
    }

    fn throw_target(&self, test_result: bool) -> i32 {
        if self.test_branches[0].0 == test_result {
            self.test_branches[0].1
        } else {
            self.test_branches[1].1
        }
    }

    fn play_turn<F>(&mut self, score_reduction: &F) -> VecDeque<(usize, usize)>
    where
        F: Fn(usize) -> usize,
    {
        let mut thrown_items = VecDeque::new();
        while let Some(item) = self.items.pop_front() {
            let mut inspected_item = self.inspect_item(item);
            inspected_item = score_reduction(inspected_item);
            let throw_target = self.throw_target(self.test_item(inspected_item));
            thrown_items.push_back((inspected_item, throw_target as usize));
            self.inspection_count += 1;
        }

        thrown_items
    }
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
        inspection_count: 0,
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

fn catch_items(monkeys: &mut Vec<Monkey>, thrown_items: VecDeque<(usize, usize)>) {
    for (item, target) in thrown_items {
        monkeys[target].items.push_back(item)
    }
}

fn play_round<F>(monkeys: &mut Vec<Monkey>, score_reduction: F)
where
    F: Fn(usize) -> usize,
{
    for monkey_index in 0..monkeys.len() {
        let turn_result = monkeys[monkey_index].play_turn(&score_reduction);
        catch_items(monkeys, turn_result);
    }
}

fn default_score_reduction(input: usize) -> usize {
    input / 3
}

fn compute_business_level(monkeys: &Vec<Monkey>) -> usize {
    monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn main() {
    if let Ok(lines) = read_input("inputs/day11.txt") {
        let mut monkeys =
            parse_monkeys(&mut lines.clone().iter()).expect("Couldn't parse monkeys!");
        for _round in 1..=20 {
            play_round(&mut monkeys, default_score_reduction);
        }
        let business_level = compute_business_level(&monkeys);
        println!("The monkey's business is {}.", business_level);
        let mut monkeys =
            parse_monkeys(&mut lines.clone().iter()).expect("Couldn't parse monkeys!");
        let conditions_lcm = monkeys
            .iter()
            .map(|monkey| monkey.test_condition)
            .reduce(|accumulator, value| lcm(accumulator as usize, value as usize) as i32)
            .expect("Couldn't compute least common multiple!")
            as usize;
        for _round in 1..=10000 {
            play_round(&mut monkeys, |worry| worry % conditions_lcm);
        }
        let business_level = compute_business_level(&monkeys);
        println!(
            "The monkey's business is {} after 10000 rounds.",
            business_level
        );
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
        assert_eq!(
            parse_starting_items("  Starting items: 80"),
            Some(VecDeque::from_iter(vec![80]))
        );
        assert_eq!(
            parse_starting_items("  Starting items: 75, 83, 74"),
            Some(VecDeque::from_iter(vec![75, 83, 74]))
        );
        assert_eq!(
            parse_starting_items("  Starting items: 86, 67, 61, 96, 52, 63, 73"),
            Some(VecDeque::from_iter(vec![86, 67, 61, 96, 52, 63, 73]))
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
                items: VecDeque::from_iter(vec![80]),
                operation: Operation::Multiplication(Operand::Number(5)),
                test_condition: 2,
                test_branches: [(true, 4), (false, 3)],
                inspection_count: 0
            })
        );
    }

    #[test]
    fn test_monkey_play_turn() {
        let lines = read_input("inputs/day11-example.txt");
        assert!(lines.is_ok());

        let mut monkeys =
            parse_monkeys(&mut lines.unwrap().iter()).expect("Monkey 0 should throw items.");
        let mut turn_result = monkeys[0].play_turn(&default_score_reduction);
        catch_items(&mut monkeys, turn_result.clone());

        assert_eq!(turn_result.pop_front(), Some((500, 3)));
        assert_eq!(turn_result.pop_front(), Some((620, 3)));

        let mut turn_result = monkeys[1].play_turn(&default_score_reduction);
        catch_items(&mut monkeys, turn_result.clone());

        assert_eq!(turn_result.pop_front(), Some((20, 0)));
        assert_eq!(turn_result.pop_front(), Some((23, 0)));
        assert_eq!(turn_result.pop_front(), Some((27, 0)));
        assert_eq!(turn_result.pop_front(), Some((26, 0)));

        let mut turn_result = monkeys[2].play_turn(&default_score_reduction);
        catch_items(&mut monkeys, turn_result.clone());

        assert_eq!(turn_result.pop_front(), Some((2080, 1)));
        assert_eq!(turn_result.pop_front(), Some((1200, 3)));
        assert_eq!(turn_result.pop_front(), Some((3136, 3)));

        let mut turn_result = monkeys[3].play_turn(&default_score_reduction);
        catch_items(&mut monkeys, turn_result.clone());

        assert_eq!(turn_result.pop_front(), Some((25, 1)));
        assert_eq!(turn_result.pop_front(), Some((167, 1)));
        assert_eq!(turn_result.pop_front(), Some((207, 1)));
        assert_eq!(turn_result.pop_front(), Some((401, 1)));
        assert_eq!(turn_result.pop_front(), Some((1046, 1)));
    }
}
