use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use utilities::*;

fn parse_section(input: &str) -> Range<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d*)-(\d*)$").unwrap();
    }
    let maybe_captures = RE.captures(input.trim());
    if let Some(captures) = maybe_captures {
        let captures = (captures.get(1), captures.get(2));
        match captures {
            (Some(first), Some(second)) => {
                return Range::<u32> {
                    start: first
                        .as_str()
                        .parse::<u32>()
                        .expect("Couldn't parse start of section"),
                    end: second
                        .as_str()
                        .parse::<u32>()
                        .expect("Couldn't parse end of section")
                        + 1,
                }
            }
            _ => return 0..0,
        }
    }
    0..0
}

fn parse_line(input: String) -> (Range<u32>, Range<u32>) {
    let mut sections = input.split(",");
    let first_section = sections.next();
    let second_section = sections.next();

    match (first_section, second_section) {
        (Some(first), Some(second)) => (parse_section(first), parse_section(second)),
        (_, _) => (0..0, 0..0),
    }
}

fn range_fully_contains(first: Range<u32>, second: Range<u32>) -> bool {
    first.contains(&second.start) && first.contains(&second.end)
        || second.contains(&first.start) && second.contains(&first.end)
}

fn main() {
    if let Ok(input) = read_input("inputs/day4.txt") {
        let sections = input
            .map(|line| line.expect("Couldn't read line"))
            .map(parse_line)
            .map(|(first, second)| range_fully_contains(first, second))
            .filter(|fully_contains| *fully_contains)
            .count();
        println!("{:#?}", sections);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn simple_section_parse() {
        let r = parse_section("42-42");
        assert_eq!(r, 42..43);
    }

    #[test]
    fn invalid_section_parse() {
        let section = parse_section("a-");
        assert_eq!(section, 0..0);
    }

    fn test_full_overlap_commutative(first: Range<u32>, second: Range<u32>, expected: bool) {
        assert_eq!(
            range_fully_contains(first.clone(), second.clone()),
            expected
        );
        assert_eq!(
            range_fully_contains(first.clone(), second.clone()),
            range_fully_contains(second, first),
            "range_fully_contains is expected to be commutative in its arguments"
        );
    }

    #[test]
    fn no_overlap_right() {
        /*
          .234.....  2-4
          .....678.  6-8
        */
        let first = 2..5;
        let second = 6..9;
        test_full_overlap_commutative(first, second, false)
    }

    #[test]
    fn partial_overlap_right() {
        /*
          ....567..  5-7
          ......789  7-9
        */
        test_full_overlap_commutative(5..8, 7..10, false)
    }

    #[test]
    fn fully_contained() {
        /*
          .2345678.  2-8
          ..34567..  3-7
        */
        test_full_overlap_commutative(2..9, 3..8, true)
    }

    #[test]
    fn partial_overlap_left() {
        /*
         .....67..  6-7
         ...456...  4-6
        */
        test_full_overlap_commutative(6..8, 4..7, false)
    }

    #[test]
    fn no_overlap_left() {
        /*
          .....6...  6-6
          ...45....  4-5
        */
        test_full_overlap_commutative(6..7, 4..6, false)
    }
}
