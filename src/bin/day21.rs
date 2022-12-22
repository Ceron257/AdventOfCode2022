use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, hash::Hash};
use utilities::read_input;

#[derive(Debug, Clone)]
enum Operand {
    Number(i64),
    Monkey(String),
}

#[derive(Debug, Clone)]
enum Operation {
    Identity(Operand),
    Addition((Operand, Operand)),
    Subtraction((Operand, Operand)),
    Multiplication((Operand, Operand)),
    Division((Operand, Operand)),
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

fn parse_operand(input: &str) -> Operand {
    if let Ok(number) = input.parse::<i64>() {
        return Operand::Number(number);
    }

    Operand::Monkey(input.to_string())
}

fn parse_operation(input: &str) -> Option<Operation> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<first_operand>[a-z\d]+)\s*(?P<operator>[+\-*/])?\s*(?P<second_operand>[a-z]+)?"
        )
        .unwrap();
    };
    let captures = RE.captures(input).expect("parse_operation no match?");
    let first_operand = parse_operand(captures.name("first_operand")?.as_str());
    match captures.name("operator") {
        None => Some(Operation::Identity(first_operand)),
        Some(operator) => {
            let operator = parse_operator(operator.as_str()).expect("Couldn't parse operator");
            let second_operand = parse_operand(
                captures
                    .name("second_operand")
                    .expect("Given operator requires second operand")
                    .as_str(),
            );
            match operator {
                Operator::Plus => Some(Operation::Addition((first_operand, second_operand))),
                Operator::Minus => Some(Operation::Subtraction((first_operand, second_operand))),
                Operator::Multiply => {
                    Some(Operation::Multiplication((first_operand, second_operand)))
                }
                Operator::Divide => Some(Operation::Division((first_operand, second_operand))),
            }
        }
    }
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

fn do_monkey_math(monkeys: &HashMap<String, Monkey>, current_monkey: &String) -> Option<i64> {
    let monkey = monkeys.get(current_monkey)?;
    match monkey.operation.clone() {
        Operation::Addition((first_operand, second_operand)) => {
            let first_operand = match first_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };

            let second_operand = match second_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };
            Some(first_operand + second_operand)
        }

        Operation::Subtraction((first_operand, second_operand)) => {
            let first_operand = match first_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };

            let second_operand = match second_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };
            Some(first_operand - second_operand)
        }
        Operation::Multiplication((first_operand, second_operand)) => {
            let first_operand = match first_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };

            let second_operand = match second_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };
            Some(first_operand * second_operand)
        }
        Operation::Division((first_operand, second_operand)) => {
            let first_operand = match first_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };

            let second_operand = match second_operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };
            Some(first_operand / second_operand)
        }
        Operation::Identity(operand) => {
            let operand = match operand {
                Operand::Number(number) => number,
                Operand::Monkey(monkey) => do_monkey_math(monkeys, &monkey)?,
            };
            Some(operand)
        }
    }
}

fn main() {
    if let Ok(lines) = read_input("inputs/day21.txt") {
        let monkeys = parse_monkeys(lines.clone()).expect("Couldn't parse monkeys");
        let result = do_monkey_math(&monkeys, &"root".to_string()).expect("Couldn't do the math :/");
        println!("{:?}", result);
    } else {
        println!("Couldn't read input.");
    }
}
