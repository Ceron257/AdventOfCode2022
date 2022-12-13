use std::slice::Iter;
use utilities::read_input;

type Value = usize;
type Row = Vec<Value>;
type Position = (i32, i32);

#[derive(Debug, PartialEq)]
struct Heightmap {
    rows: Vec<Row>,
    start: Option<Position>,
    finish: Option<Position>,
}

impl Heightmap {
    fn new(input: Iter<String>) -> Heightmap {
        let mut rows = Vec::new();
        let mut start = None;
        let mut finish = None;
        for (line_index, line) in input.enumerate() {
            if let Some(start_in_row) = find_start_in_row(line) {
                assert_eq!(start, None, "Found more than one start points.");
                start = Some((line_index as i32, start_in_row));
            }
            if let Some(finish_in_row) = find_finish_in_row(line) {
                assert_eq!(finish, None, "Found more than one finish points.");
                finish = Some((line_index as i32, finish_in_row));
            }
            rows.push(parse_row(line));
        }

        Heightmap {
            rows,
            start,
            finish,
        }
    }

    fn neighbors(&self, position: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if <i32>::abs(x) + <i32>::abs(y) != 1 {
                    continue;
                }
                neighbors.push((position.0 + x, position.1 + y));
            }
        }
        neighbors = neighbors
            .iter()
            .filter(|p| {
                p.0 >= 0
                    && p.1 >= 0
                    && p.0 < self.rows[0].len() as i32
                    && p.1 < self.rows.len() as i32
            })
            .map(|p| *p)
            .collect::<Vec<_>>();
        neighbors
    }

    fn height(&self, position: Position) -> usize {
        self.rows[position.0 as usize][position.1 as usize]
    }

    fn shortest_path(&self, from: Position, to: Position) -> Vec<Position> {
        Vec::new()
    }
}

fn parse_row(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|char| match char {
            'S' => 0,
            'E' => 25,
            _ => char.to_digit(36).expect("Couldn't parse value in row") as usize - 10,
        })
        .collect::<Vec<_>>()
}

fn find_in_row(row: &str, what: char) -> Option<i32> {
    Some(row.chars().position(|char| char == what)? as i32)
}

fn find_start_in_row(row: &str) -> Option<i32> {
    find_in_row(row, 'S')
}

fn find_finish_in_row(row: &str) -> Option<i32> {
    find_in_row(row, 'E')
}

fn main() {
    if let Ok(lines) = read_input("inputs/day12.txt") {
        let heightmap = Heightmap::new(lines.iter());
        println!("{:?}", heightmap);
    } else {
        println!("Couldn't read input.");
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{parse_row, Heightmap};

    #[test]
    fn test_parse_row() {
        assert_eq!(parse_row("abc"), vec![0, 1, 2]);
        assert_eq!(parse_row("SyzE"), vec![0, 24, 25, 25]);
    }

    #[test]
    fn test_heightmap_neighbors() {
        let heightmap = Heightmap {
            rows: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            start: None,
            finish: None,
        };

        assert_eq!(heightmap.neighbors((0, 0)), vec![(0, 1), (1, 0)]);
        assert_eq!(heightmap.neighbors((1, 0)), vec![(0, 0), (1, 1), (2, 0)]);
        assert_eq!(heightmap.neighbors((2, 0)), vec![(1, 0), (2, 1)]);

        assert_eq!(heightmap.neighbors((0, 1)), vec![(0, 0), (0, 2), (1, 1)]);
        assert_eq!(
            heightmap.neighbors((1, 1)),
            vec![(0, 1), (1, 0), (1, 2), (2, 1)]
        );
        assert_eq!(heightmap.neighbors((2, 1)), vec![(1, 1), (2, 0), (2, 2)]);

        assert_eq!(heightmap.neighbors((0, 2)), vec![(0, 1), (1, 2)]);
        assert_eq!(heightmap.neighbors((1, 2)), vec![(0, 2), (1, 1), (2, 2)]);
        assert_eq!(heightmap.neighbors((2, 2)), vec![(1, 2), (2, 1)]);
    }
}
