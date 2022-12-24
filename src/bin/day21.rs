use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use utilities::read_input;

#[derive(Debug, Clone)]
enum Operand {
    Number(i64),
    Monkey(String),
}

#[derive(Debug)]
struct Operation {
    operator: Option<Operator>,
    first_operand: Operand,
    second_operand: Option<Operand>,
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn parse_operator(input: &str) -> Option<Operator> {
    match input.trim() {
        "+" => Some(Operator::Plus),
        "-" => Some(Operator::Minus),
        "*" => Some(Operator::Multiply),
        "/" => Some(Operator::Divide),
        _ => None,
    }
}

fn parse_operand(input: &str) -> Option<Operand> {
    if let Ok(number) = input.parse::<i64>() {
        return Some(Operand::Number(number));
    }
    if input.is_empty() {
        return None;
    }
    Some(Operand::Monkey(input.to_string()))
}

fn parse_operation(input: &str) -> Option<Operation> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<first_operand>[a-z\d]+)\s*(?P<operator>[+\-*/])?\s*(?P<second_operand>[a-z]+)?"
        )
        .unwrap();
    };
    let captures = RE.captures(input).expect("parse_operation no match?");
    let first_operand = parse_operand(captures.name("first_operand")?.as_str())?;
    let second_operand = parse_operand(
        captures
            .name("second_operand")
            .map_or_else(|| "", |v| v.as_str()),
    );
    let operator = parse_operator(captures.name("operator").map_or_else(|| "", |v| v.as_str()));
    Some(Operation {
        operator,
        first_operand,
        second_operand,
    })
}

#[derive(Debug)]
struct Monkey {
    name: String,
    operation: Operation,
}

fn parse_monkey(input: &str) -> Option<Monkey> {
    let mut split = input.split(":");
    let name = split.next().expect("No name?").to_string();
    let operation = split.next().expect("No Operation?");
    Some(Monkey {
        name,
        operation: parse_operation(operation)?,
    })
}

fn parse_monkeys(input: Vec<String>) -> Option<HashMap<String, Monkey>> {
    let mut result = HashMap::new();
    for line in input {
        let monkey = parse_monkey(line.as_str()).expect("Couldn't parse monkey");
        result.entry(monkey.name.clone()).or_insert(monkey);
    }
    Some(result)
}

fn resolve_operand(monkeys: &HashMap<String, Monkey>, operand: &Operand) -> Option<i64> {
    match operand {
        Operand::Number(number) => Some(*number),
        Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey),
    }
}

fn do_monkey_math(monkeys: &HashMap<String, Monkey>, current_monkey: &String) -> Option<i64> {
    let monkey = monkeys.get(current_monkey)?;
    let first_operand = resolve_operand(monkeys, &monkey.operation.first_operand)?;
    if monkey.operation.operator.is_none() {
        return Some(first_operand);
    }
    let second_operand = resolve_operand(
        monkeys,
        &monkey
            .operation
            .second_operand
            .as_ref()
            .expect("Operator requires second operand."),
    )?;
    let result = match &monkey.operation.operator.as_ref().unwrap() {
        Operator::Divide => first_operand / second_operand,
        Operator::Multiply => first_operand * second_operand,
        Operator::Minus => first_operand - second_operand,
        Operator::Plus => first_operand + second_operand,
    };
    Some(result)
}

fn yell_chain<'a, 'b>(
    monkeys: &'a HashMap<String, Monkey>,
    from: &String,
    to: &'b String,
) -> Option<Vec<&'a String>>
where
    'b: 'a,
{
    if from == to {
        let result = Vec::new();
        return Some(result);
    }
    let current_monkey = monkeys.get(from)?;
    if let Operand::Monkey(monkey) = &current_monkey.operation.first_operand {
        let mut current = yell_chain(monkeys, &monkey, to);
        if current.is_none() {
            if let Operand::Monkey(monkey) = current_monkey.operation.second_operand.as_ref()? {
                current = yell_chain(monkeys, &monkey, to);
                current.as_mut()?.push(&monkey);
                return Some(current?);
            }
        }
        current.as_mut()?.push(&monkey);
        return Some(current?);
    }
    None
}

fn do_correct_monkey_math(monkeys: &HashMap<String, Monkey>) -> Option<i64> {
    let root = "root".to_string();
    let humn = "humn".to_string();

    let yell_chain = yell_chain(&monkeys, &root, &humn)?
        .iter()
        .rev()
        .map(|v| *v)
        .collect_vec();
    let mut iter = yell_chain.iter();
    let mut current_monkey = monkeys.get(&root)?;
    let next_to_solve = *iter.next()?;
    let mut expected_value;
    match &current_monkey.operation.first_operand {
        Operand::Number(expected_number) => {
            match current_monkey.operation.second_operand.as_ref()? {
                Operand::Number(_) => {
                    panic!("Sorry cannot solve number == number :/");
                }
                Operand::Monkey(_) => {
                    expected_value = *expected_number;
                }
            }
        }
        Operand::Monkey(first_monkey) => {
            if first_monkey == next_to_solve {
                match current_monkey.operation.second_operand.as_ref()? {
                    Operand::Number(number) => {
                        expected_value = *number;
                    }
                    Operand::Monkey(second_monkey) => {
                        expected_value = do_monkey_math(monkeys, second_monkey)?;
                    }
                }
            } else {
                expected_value = do_monkey_math(monkeys, first_monkey)?;
            }
        }
    }

    current_monkey = monkeys.get(next_to_solve)?;

    while let Some(next_to_solve) = iter.next() {
        match &current_monkey.operation.first_operand {
            Operand::Number(first_number) => {
                match current_monkey.operation.second_operand.as_ref()? {
                    Operand::Number(_) => {
                        panic!("Sorry cannot solve number == number :/ 2");
                    }
                    Operand::Monkey(second_monkey) => {
                        assert_eq!(*next_to_solve, second_monkey);
                        expected_value = match current_monkey.operation.operator.as_ref()? {
                            // first_number + second_monkey = expected_value
                            // => second_monkey = expected_value - first_number
                            Operator::Plus => expected_value - first_number,
                            // first_number - second_monkey = expected_value
                            // => second_monkey = first_number - expected_value
                            Operator::Minus => first_number - expected_value,
                            // first_number * second_monkey = expected_value
                            // => second_monkey = expected_value / first_number
                            Operator::Multiply => expected_value / first_number,
                            // expeced_number / second_monkey = expected_value
                            // => second_monkey = first_number / expected_value
                            Operator::Divide => first_number / expected_value,
                        };
                    }
                }
            }
            Operand::Monkey(first_monkey) => {
                if first_monkey == *next_to_solve {
                    let second_number = match current_monkey.operation.second_operand.as_ref()? {
                        Operand::Number(number) => *number,
                        Operand::Monkey(second_monkey) => do_monkey_math(monkeys, second_monkey)?,
                    };
                    expected_value = match current_monkey.operation.operator.as_ref()? {
                        // first_monkey + second_number = expected_value
                        // => first_monkey = expected_value - second_number
                        Operator::Plus => expected_value - second_number,
                        // first_monkey - second_number = expected_value
                        // => first_monkey = expected_value + second_number
                        Operator::Minus => expected_value + second_number,
                        // first_monkey * second_number = expected_value
                        // => first_monkey = expected_value / second_number
                        Operator::Multiply => expected_value / second_number,
                        // first_monkey / second_number = expected_value
                        // => first_monkey = expected_value * second_number
                        Operator::Divide => expected_value * second_number,
                    };
                } else {
                    let first_number = match &current_monkey.operation.first_operand {
                        Operand::Number(number) => *number,
                        Operand::Monkey(monkey) => do_monkey_math(monkeys, monkey)?,
                    };

                    expected_value = match current_monkey.operation.operator.as_ref()? {
                        // first_number + second_monkey = expected_value
                        // => second_monkey = expected_value - first_number
                        Operator::Plus => expected_value - first_number,
                        // first_number - second_monkey = expected_value
                        // => second_monkey = first_number - expected_value
                        Operator::Minus => first_number - expected_value,
                        // first_number * second_monkey = expected_value
                        // => second_monkey = expected_value / first_number
                        Operator::Multiply => expected_value / first_number,
                        // first_number / second_monkey = expected_value
                        // => second_monkey = first_number / expected_value
                        Operator::Divide => first_number / expected_value,
                    };
                }
            }
        }
        current_monkey = monkeys.get(*next_to_solve)?;
    }

    Some(expected_value)
}

fn main() {
    if let Ok(lines) = read_input("inputs/day21.txt") {
        let monkeys = parse_monkeys(lines.clone()).expect("Couldn't parse monkeys");
        let result =
            do_monkey_math(&monkeys, &"root".to_string()).expect("Couldn't do the math :/");
        println!("Root's result will be {:?}.", result);
        let result =
            do_correct_monkey_math(&monkeys).expect("Couldn't determine what humn has to say.");
        println!("Humn has to say {}.", result);
    } else {
        println!("Couldn't read input.");
    }
}
