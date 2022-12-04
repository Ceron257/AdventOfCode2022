use itertools::Itertools;
use utilities::*;

#[derive(Debug)]
struct Rucksack {
    first_compartment: String,
    second_compartment: String,
}

impl Rucksack {
    fn from(input: &mut String) -> Rucksack {
        let half_length = input.chars().count() / 2;
        let first = input.drain(..half_length).collect::<String>();
        Rucksack {
            first_compartment: first,
            second_compartment: input.clone(),
        }
    }

    fn duplicate(self) -> Result<char, String> {
        for item in self.first_compartment.chars() {
            if self.second_compartment.contains(item) {
                return Ok(item);
            }
        }
        Err(String::from("No duplicate found"))
    }

    fn str(&self) -> String {
        self.first_compartment.clone() + &self.second_compartment
    }
}

fn score(character: char) -> u32 {
    let offset: i32 = if character.is_uppercase() { 27 } else { 1 }
        - 'A'
            .to_digit(36)
            .expect("'A'.to_digit(36) failed unexpectedly") as i32;
    let result = character
        .to_digit(36)
        .expect("couldn't calculate digit for given character") as i32
        + offset;
    assert!(result >= 0);
    result as u32
}

fn group_badge(group: &[Rucksack]) -> char {
    assert_eq!(group.len(), 3);
    let group_items = group.iter().map(Rucksack::str).collect::<String>();
    let is_present_in_all_rucksacks =
        |&item: &char| group.iter().all(|rucksack| rucksack.str().contains(item));
    let mut uniq = group_items
        .chars()
        .filter(is_present_in_all_rucksacks)
        .unique();
    match uniq.next() {
        Some(v) => v,
        None => panic!("unable to find group badge"),
    }
}

fn main() {
    match read_input("inputs/day3.txt") {
        Ok(mut lines) => {
            let total_score = lines
                .clone()
                .iter_mut()
                .map(Rucksack::from)
                .map(Rucksack::duplicate)
                .map(|dup| dup.expect("No duplicate found"))
                .map(score)
                .sum::<u32>();

            println!("{}", total_score);

            let score: u32 = lines
                .iter_mut()
                .map(Rucksack::from)
                .collect::<Vec<Rucksack>>()
                .chunks(3)
                .map(group_badge)
                .map(score)
                .sum();
            println!("{}", score);
        }
        Err(err) => println!("Failed to read input: {}", err),
    }
}
