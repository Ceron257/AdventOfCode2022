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

fn main() {
    if let Ok(lines) = read_input("inputs/day21.txt") {
        let monkeys = parse_monkeys(lines.clone()).expect("Couldn't parse monkeys");
        let result =
            do_monkey_math(&monkeys, &"root".to_string()).expect("Couldn't do the math :/");
        println!("{:?}", result);
        println!(
            "{:?}",
            yell_chain(&monkeys, &"root".to_string(), &"humn".to_string())
        );
    } else {
        println!("Couldn't read input.");
    }
}
