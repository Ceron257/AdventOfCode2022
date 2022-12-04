use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use utilities::*;

type Section = Range<u32>;

fn parse_section(input: &str) -> Section {
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

fn parse_line(input: String) -> (Section, Section) {
    let mut sections = input.split(",");
    let first_section = sections.next();
    let second_section = sections.next();

    match (first_section, second_section) {
        (Some(first), Some(second)) => (parse_section(first), parse_section(second)),
        (_, _) => (0..0, 0..0),
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

fn main() {
    if let Ok(input) = read_input("inputs/day4.txt") {
        let fully_contains_count = input
            .map(|line| line.expect("Couldn't read line"))
            .map(parse_line)
            .map(|(first, second)| range_fully_contains(first, second))
            .filter(|fully_contains| *fully_contains)
            .count();
        println!(
            "{:#?} sections fully contain other sections of the same group.",
            fully_contains_count
        );
    }
    if let Ok(input) = read_input("inputs/day4.txt") {
        let overlap_count = input
            .map(|line| line.expect("Couldn't read line"))
            .map(parse_line)
            .map(|(first, second)| ranges_overlap(first, second))
            .filter(|fully_contains| *fully_contains)
            .count();
        println!(
            "{:#?} sections overlap with the other section of the same group.",
            overlap_count
        );
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
