use itertools::{enumerate, Itertools};
use utilities::read_input;

fn wrap_index(index: i64, length: usize) -> usize {
    let offset = index % length as i64;
    if offset < 0 {
        (length as i64 + offset) as usize
    } else {
        offset as usize
    }
}

fn parse_input(input: &[String]) -> Vec<(usize, i64)> {
    to_indexed_numbers(
        input
            .iter()
            .map(|v| v.as_str().parse::<i64>().expect("Couldn't parse number"))
            .collect_vec(),
    )
}

fn to_indexed_numbers(input: Vec<i64>) -> Vec<(usize, i64)> {
    enumerate(input).collect_vec()
}

fn do_step(numbers: &mut Vec<(usize, i64)>, index_to_move: usize) {
    let position = numbers
        .iter()
        .position(|(i, _)| *i == index_to_move)
        .expect("Can't find number!");
    let element_to_move = numbers.remove(position);
    let new_position = wrap_index(position as i64 + element_to_move.1, numbers.len());
    numbers.insert(new_position, element_to_move);
}

fn index_of_zero(numbers: &Vec<(usize, i64)>) -> usize {
    numbers
        .iter()
        .position(|(_, v)| *v == 0)
        .expect("Couldn't find zero!")
}

fn decrypt_step(numbers: &mut Vec<(usize, i64)>) {
    for number_index in 0..numbers.len() {
        do_step(numbers, number_index);
    }
}

fn decrypt(numbers: &mut Vec<(usize, i64)>, iterations: usize) -> usize {
    let number_length = numbers.len();
    for _iteration in 0..iterations {
        decrypt_step(numbers);
    }
    let zero_index = index_of_zero(&numbers);
    (numbers[wrap_index((zero_index + 1000) as i64, number_length)].1
        + numbers[wrap_index((zero_index + 2000) as i64, number_length)].1
        + numbers[wrap_index((zero_index + 3000) as i64, number_length)].1) as usize
}

fn main() {
    if let Ok(lines) = read_input("inputs/day20.txt") {
        let mut numbers = parse_input(&lines);
        let sum = decrypt(&mut numbers, 1);
        println!("{:?}", sum);

        let decryption_key = 811589153;
        let mut numbers = parse_input(&lines)
            .iter()
            .map(|(i, v)| (*i, v * decryption_key))
            .collect_vec();
        let sum = decrypt(&mut numbers, 10);
        println!("{:?}", sum);
    } else {
        println!("Couldn't read input.");
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_wrap_index() {
        let length: usize = 10;
        for i in 0..length {
            assert_eq!(wrap_index(i as i64, length), i as usize);
        }

        assert_eq!(wrap_index(-20, length), 0);
        assert_eq!(wrap_index(-10, length), 0);
        assert_eq!(wrap_index(-1, length), length - 1);
        assert_eq!(wrap_index(11, length), 1);
        assert_eq!(wrap_index(21, length), 1);
        assert_eq!(wrap_index(25, length), 5);
    }

    #[test]
    fn test_example_input() {
        let mut numbers =
            parse_input(&read_input("inputs/day20-example.txt").expect("Coulnd't read input"));

        do_step(&mut numbers, 0);
        assert_eq!(
            numbers,
            vec![(1, 2), (0, 1), (2, -3), (3, 3), (4, -2), (5, 0), (6, 4)]
        );
        do_step(&mut numbers, 1);
        assert_eq!(
            numbers,
            vec![(0, 1), (2, -3), (1, 2), (3, 3), (4, -2), (5, 0), (6, 4)]
        );
        do_step(&mut numbers, 2);
        assert_eq!(
            numbers,
            vec![(0, 1), (1, 2), (3, 3), (4, -2), (2, -3), (5, 0), (6, 4)]
        );
        do_step(&mut numbers, 3);
        assert_eq!(
            numbers,
            vec![(0, 1), (1, 2), (4, -2), (2, -3), (5, 0), (3, 3), (6, 4)]
        );
        do_step(&mut numbers, 4);
        assert_eq!(
            numbers,
            vec![(4, -2), (0, 1), (1, 2), (2, -3), (5, 0), (3, 3), (6, 4)]
        );
        do_step(&mut numbers, 5);
        assert_eq!(
            numbers,
            vec![(4, -2), (0, 1), (1, 2), (2, -3), (5, 0), (3, 3), (6, 4)]
        );
        do_step(&mut numbers, 6);
        assert_eq!(
            numbers,
            vec![(4, -2), (0, 1), (1, 2), (2, -3), (6, 4), (5, 0), (3, 3)]
        );

        let zero_index = index_of_zero(&numbers);
        assert_eq!(wrap_index((zero_index + 1000) as i64, numbers.len()), 4);
        assert_eq!(wrap_index((zero_index + 2000) as i64, numbers.len()), 3);
        assert_eq!(wrap_index((zero_index + 3000) as i64, numbers.len()), 2);
    }
}
