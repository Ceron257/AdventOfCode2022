use std::{collections::HashMap, slice::Iter};
use utilities::read_input;

type Value = usize;
type Row = Vec<Value>;
type Position = (i32, i32);

#[derive(Debug, PartialEq, Clone)]
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
                    && p.0 < self.rows.len() as i32
                    && p.1 < self.rows[0].len() as i32
            })
            .map(|p| *p)
            .collect::<Vec<_>>();
        neighbors
    }

    fn height(&self, position: Position) -> usize {
        self.rows[position.0 as usize][position.1 as usize]
    }

    fn heuristic(from: Position, to: Position) -> usize {
        ((to.0 - from.0).abs() + (to.1 - from.1).abs()) as usize
    }

    fn reconstruct_path(
        predecessors: HashMap<Position, Option<Position>>,
        finish: Position,
    ) -> Vec<Position> {
        let mut path = Vec::new();
        let mut current = finish;
        path.push(current);
        while let Some(predecessor) = predecessors.get(&current) {
            path.insert(0, predecessor.unwrap());
            current = predecessor.unwrap();
        }
        path
    }

    fn shortest_path(&self, from: Position, to: Position) -> Vec<Position> {
        let mut open_nodes = Vec::new();
        let mut closed_nodes = Vec::new();

        let mut path = HashMap::new();

        open_nodes.push((from, 0));
        loop {
            let current_node = open_nodes
                .iter()
                .min_by(|x, y| x.1.cmp(&y.1))
                .unwrap()
                .clone();
            open_nodes.remove(open_nodes.iter().position(|&x| x == current_node).unwrap());
            if current_node.0 == to {
                return Heightmap::reconstruct_path(path, self.finish.unwrap());
            }
            closed_nodes.push(current_node.0);
            for neighbor in self.neighbors(current_node.0) {
                if closed_nodes.contains(&neighbor) {
                    continue;
                }
                let height_difference = self
                    .height(neighbor)
                    .saturating_sub(self.height(current_node.0));
                if height_difference > 1 {
                    continue;
                }
                let tentative_cost = current_node.1 + height_difference;
                // if we have not found a  better alternative just continue:
                if !open_nodes.is_empty()
                    && open_nodes.iter().any(|(position, _)| *position == neighbor)
                    && !open_nodes.iter().any(|(position, distance)| {
                        position == &neighbor && *distance > tentative_cost
                    })
                {
                    continue;
                }
                *path.entry(neighbor).or_insert(None) = Some(current_node.0);
                let estimated_cost = tentative_cost
                    + Heightmap::heuristic(neighbor, self.finish.expect("Got no finish"));
                if let Some(existing_index) = open_nodes
                    .iter()
                    .position(|(position, _)| *position == neighbor)
                {
                    open_nodes.remove(existing_index);
                }
                open_nodes.push((neighbor, estimated_cost));
            }
            if open_nodes.is_empty() {
                break;
            }
        }
        Vec::new()
    }

    fn shortest_hiking_trail(self) -> Vec<Position> {
        let finish = self.finish.unwrap();
        let mut path = self.shortest_path(self.start.unwrap(), finish);
        for row in 0..self.rows.len() {
            for column in 0..self.rows[0].len() {
                let position = (row as i32, column as i32);
                if self.height(position) != 0 {
                    continue;
                }
                let possibly_shorter_path = self.shortest_path(position, finish);
                if !possibly_shorter_path.is_empty() && possibly_shorter_path.len() < path.len() {
                    path = possibly_shorter_path;
                }
            }
        }
        path
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
        println!(
            "The shortest path takes {:?} steps.",
            heightmap
                .clone()
                .shortest_path(
                    heightmap.start.expect("Got no start."),
                    heightmap.finish.expect("Got no finish.")
                )
                .len()
                .saturating_sub(1)
        );
        let shortest_trail = heightmap.shortest_hiking_trail();
        println!(
            "The shortest hiking trail takes {:?} steps.",
            shortest_trail.len().saturating_sub(1)
        );
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
