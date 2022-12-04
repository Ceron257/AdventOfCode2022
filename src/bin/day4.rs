use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use utilities::*;

type Section = Range<u32>;

fn parse_section(input: &str) -> Result<Section, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d*)-(\d*)$").unwrap();
    }
    match RE.captures(input.trim()) {
        Some(captures) => match (captures.get(1), captures.get(2)) {
            (Some(first), Some(second)) => {
                return match (
                    first.as_str().parse::<u32>(),
                    second.as_str().parse::<u32>(),
                ) {
                    (Ok(start), Ok(end)) => Ok(start..end + 1),
                    (Err(_), _) => Err("Couldn't parse start of section".to_string()),
                    (_, Err(_)) => Err("Couldn't parse end of section".to_string()),
                };
            }
            (None, _) => return Err("Couldn't find start of section.".to_string()),
            (_, None) => return Err("Couldn't find end of section.".to_string()),
        },
        None => Err("Couldn't parse section.".to_string()),
    }
}

fn parse_line(input: &String) -> Result<(Section, Section), String> {
    let mut sections = input.split(",");
    match (sections.next(), sections.next()) {
        (Some(first), Some(second)) => Ok((parse_section(first)?, parse_section(second)?)),
        (None, _) => Err("Couldn't find first section in input.".to_string()),
        (_, None) => Err("Couldn't find second section in input.".to_string()),
    }
}

fn range_fully_contains(first: Section, second: Section) -> bool {
    first.contains(&second.start) && first.contains(&second.end)
        || second.contains(&first.start) && second.contains(&first.end)
}

fn ranges_overlap(first: Section, second: Section) -> bool {
    first.contains(&second.start)
        || first.contains(&(&second.end - 1))
        || second.contains(&first.start)
        || second.contains(&(&first.end - 1))
}

fn calculate_overlap(
    f: fn(Section, Section) -> bool,
    input: Result<(Section, Section), String>,
) -> bool {
    match input {
        Ok((first, second)) => f(first, second),
        Err(msg) => panic!("Failed to calculate overlap: {}", msg),
    }
}

fn main() {
    match read_input("inputs/day4.txt") {
        Ok(lines) => {
            let fully_contains_count = lines
                .iter()
                .map(parse_line)
                .map(|group| calculate_overlap(range_fully_contains, group))
                .filter(|fully_contains| *fully_contains)
                .count();

            let overlap_count = lines
                .iter()
                .map(parse_line)
                .map(|group| calculate_overlap(ranges_overlap, group))
                .filter(|fully_contains| *fully_contains)
                .count();

            println!(
                "{:#?} sections fully contain other sections of the same group.",
                fully_contains_count
            );
            println!(
                "{:#?} sections overlap with the other section of the same group.",
                overlap_count
            );
        }
        Err(err) => println!("Unable to read input: {}", err),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn simple_section_parse() {
        let r = parse_section("42-42");
        assert_eq!(r, Ok(42..43));
    }

    #[test]
    fn invalid_section_parse() {
        let section = parse_section("a-");
        assert!(section.is_err());
    }

    fn test_section_commutative(f: fn(Section, Section) -> bool, first: Section, second: Section) {
        assert_eq!(
            f(first.clone(), second.clone()),
            f(second, first),
            "f is expected to be commutative in its arguments"
        );
    }

    fn test_full_overlap_commutative(first: Section, second: Section, expected: bool) {
        assert_eq!(
            range_fully_contains(first.clone(), second.clone()),
            expected
        );
        test_section_commutative(range_fully_contains, first, second)
    }

    #[test]
    fn contains_no_overlap_right() {
        /*
          .234.....  2-4
          .....678.  6-8
        */
        test_full_overlap_commutative(2..5, 6..9, false)
    }

    #[test]
    fn contains_partial_overlap_right() {
        /*
          ....567..  5-7
          ......789  7-9
        */
        test_full_overlap_commutative(5..8, 7..10, false)
    }

    #[test]
    fn contains_fully_contained() {
        /*
          .2345678.  2-8
          ..34567..  3-7
        */
        test_full_overlap_commutative(2..9, 3..8, true)
    }

    #[test]
    fn contains_partial_overlap_left() {
        /*
         .....67..  6-7
         ...456...  4-6
        */
        test_full_overlap_commutative(6..8, 4..7, false)
    }

    #[test]
    fn contains_no_overlap_left() {
        /*
          .....6...  6-6
          ...45....  4-5
        */
        test_full_overlap_commutative(6..7, 4..6, false)
    }

    fn test_overlap_commutative(first: Section, second: Section, expected: bool) {
        assert_eq!(ranges_overlap(first.clone(), second.clone()), expected);
        test_section_commutative(ranges_overlap, first, second)
    }

    #[test]
    fn overlaps_no_overlap_right() {
        /*
          .234.....  2-4
          .....678.  6-8
        */
        test_overlap_commutative(2..5, 6..9, false)
    }

    #[test]
    fn overlaps_partial_overlap_right() {
        /*
          ....567..  5-7
          ......789  7-9
        */
        test_overlap_commutative(5..8, 7..10, true)
    }

    #[test]
    fn overlaps_fully_contained() {
        /*
          .2345678.  2-8
          ..34567..  3-7
        */
        test_overlap_commutative(2..9, 3..8, true)
    }

    #[test]
    fn overlaps_partial_overlap_left() {
        /*
         .....67..  6-7
         ...456...  4-6
        */
        test_overlap_commutative(6..8, 4..7, true)
    }

    #[test]
    fn overlaps_no_overlap_left() {
        /*
          .....6...  6-6
          ...45....  4-5
        */
        test_overlap_commutative(6..7, 4..6, false)
    }
}
