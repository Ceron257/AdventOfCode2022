use utilities::*;

#[derive(Debug)]
struct Rucksack {
    first_compartment: String,
    second_compartment: String,
}

impl Rucksack {
    fn from(mut input: String) -> Rucksack {
        let half_length = input.chars().count() / 2;
        let first = input.drain(..half_length).collect::<String>();
        Rucksack {
            first_compartment: first,
            second_compartment: input,
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

fn main() {
    if let Ok(input) = read_input("inputs/day3.txt") {
        let duplicates = input
            .map(|line| Rucksack::from(line.expect("Couldn't read line.")))
            .map(Rucksack::duplicate)
            .map(|dup| dup.expect("No duplicate found"))
            .map(score)
            .sum::<u32>();
        println!("{:?}", duplicates);
    }
}
