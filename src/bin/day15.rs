use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::ops::{Range, RangeInclusive};
use utilities::read_input;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

fn parse_line(input: &str) -> Option<(Position, Position)> {
    lazy_static!(
      static ref RE: Regex = Regex::new(r"Sensor at x=(?P<SensorX>-?\d+), y=(?P<SensorY>-?\d+): closest beacon is at x=(?P<BeaconX>-?\d+), y=(?P<BeaconY>-?\d+)").unwrap();
    );
    let captures = RE.captures(input)?;
    let sensor_x = captures.name("SensorX")?.as_str().parse::<i64>().ok()?;
    let sensor_y = captures.name("SensorY")?.as_str().parse::<i64>().ok()?;
    let beacon_x = captures.name("BeaconX")?.as_str().parse::<i64>().ok()?;
    let beacon_y = captures.name("BeaconY")?.as_str().parse::<i64>().ok()?;

    Some((
        Position {
            x: sensor_x,
            y: sensor_y,
        },
        Position {
            x: beacon_x,
            y: beacon_y,
        },
    ))
}

fn manhattan_distance(first: &Position, second: &Position) -> usize {
    ((second.x - first.x).abs() + (second.y - first.y).abs()) as usize
}

#[derive(Debug, PartialEq)]
struct Sensor {
    position: Position,
    range: usize,
}

impl Sensor {
    fn new(input: (Position, Position)) -> Sensor {
        Sensor {
            position: input.0.clone(),
            range: manhattan_distance(&input.0, &input.1),
        }
    }

    fn no_beacon_range(&self, y: i64) -> Range<i64> {
        let width = self.range as i64 - (y - self.position.y).abs();
        self.position.x - width..self.position.x + width + 1
    }
}

fn count_no_beacon_positions(sensors: &Vec<Sensor>, y: i64) -> usize {
    let mut no_beacon_ranges = sensors
        .iter()
        .map(|sensor| sensor.no_beacon_range(y))
        .collect::<Vec<_>>();
    no_beacon_ranges.sort_by(|left, right| left.start.cmp(&right.start));
    merge_ranges(no_beacon_ranges)
        .map(|r| r.end.saturating_sub(r.start).saturating_sub(1))
        .sum::<i64>() as usize
}

fn find_beacon(sensors: Vec<Sensor>, range: RangeInclusive<i64>) -> Option<Position> {
    for y in range {
        let mut no_beacon_ranges = sensors
            .iter()
            .map(|sensor| sensor.no_beacon_range(y))
            .collect::<Vec<_>>();
        no_beacon_ranges.sort_by(|left, right| left.start.cmp(&right.start));
        let no_beacon_ranges = no_beacon_ranges
            .iter()
            .filter(|r| !r.is_empty())
            .map(|r| r.clone())
            .collect::<Vec<_>>();
        let merged_ranges = merge_ranges(no_beacon_ranges).collect::<Vec<_>>();
        // if there are two ranges left after merging we found a position that's not
        // covered by any sensor. That must be the beacon:
        if merged_ranges.len() > 1 {
            return Some(Position {
                x: merged_ranges[0].end,
                y,
            });
        }
    }
    None
}

fn tuning_frequency(position: Position) -> i64 {
    position.x * 4000000 + position.y
}

fn main() {
    if let Ok(lines) = read_input("inputs/day15.txt") {
        let sensors = lines
            .iter()
            .map(<String>::as_str)
            .map(|line| parse_line(line).expect("Couldn't parse input line"))
            .map(Sensor::new)
            .collect::<Vec<_>>();
        println!(
            "There are {:?} positions where no beacons can be.",
            count_no_beacon_positions(&sensors, 2000000)
        );
        let beacon_position = find_beacon(sensors, 0..=4000000).expect("Couldn't find beacon.");
        println!(
            "The tuning frequence is {}.",
            tuning_frequency(beacon_position)
        );
    } else {
        println!("Couldn't read input!");
    }
}

// taken from https://codereview.stackexchange.com/a/103989:

#[derive(Debug)]
struct MergedRanges<I> {
    values: I,
    current: Option<Range<i64>>,
}

fn merge_ranges<I>(iterator: I) -> MergedRanges<I::IntoIter>
where
    I: IntoIterator<Item = Range<i64>>,
{
    let mut iterator = iterator.into_iter();
    let current = iterator.next();

    MergedRanges {
        values: iterator,
        current,
    }
}

impl<I> Iterator for MergedRanges<I>
where
    I: Iterator<Item = Range<i64>>,
{
    type Item = Range<i64>;

    fn next(&mut self) -> Option<Range<i64>> {
        if let Some(mut current) = self.current.clone() {
            for new in &mut self.values {
                if current.end < new.start {
                    self.current = Some(new);
                    return Some(current);
                }

                current.end = cmp::max(current.end, new.end);
            }
            self.current = None;
            return Some(current);
        }
        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn load_example_input() -> Vec<Sensor> {
        let lines = read_input("inputs/day15-example.txt").expect("Couldn't read from input file.");
        lines
            .iter()
            .map(<String>::as_str)
            .map(|line| parse_line(line).expect("Couldn't parse input line"))
            .map(Sensor::new)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_load_input() {
        let sensors = load_example_input();
        assert_eq!(sensors.len(), 14);
        assert_eq!(
            sensors[0],
            Sensor {
                position: Position { x: 2, y: 18 },
                range: 7
            }
        );
        assert_eq!(
            sensors[12],
            Sensor {
                position: Position { x: 14, y: 3 },
                range: 1
            }
        );
    }

    #[test]
    fn test_no_beacon_range() {
        let sensors = load_example_input();
        assert_eq!(sensors[6].no_beacon_range(-3), 9..8);
        assert_eq!(sensors[6].no_beacon_range(-2), 8..9);
        assert_eq!(sensors[6].no_beacon_range(0), 6..11);
        assert_eq!(sensors[6].no_beacon_range(7), -1..18);
        assert_eq!(sensors[6].no_beacon_range(16), 8..9);
        assert_eq!(sensors[6].no_beacon_range(17), 9..8);
    }

    #[test]
    fn test_merged_ranges() {
        let first = 0..1;
        let second = 2..3;
        let ranges = vec![first, second];
        println!("{:?}", merge_ranges(ranges).collect::<Vec<_>>());
    }
}
