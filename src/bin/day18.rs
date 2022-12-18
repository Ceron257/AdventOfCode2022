use itertools::Itertools;
use std::{collections::HashSet, ops::Range};
use utilities::read_input;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_position(input: &str) -> Option<Position> {
    let coordinates = input.split(",").collect_vec();
    if coordinates.len() != 3 {
        return None;
    }
    let x = coordinates[0].parse::<i64>().ok()?;
    let y = coordinates[1].parse::<i64>().ok()?;
    let z = coordinates[2].parse::<i64>().ok()?;

    Some(Position { x, y, z })
}

fn manhattan_distance(first: &Position, second: &Position) -> usize {
    ((second.x - first.x).abs() + (second.y - first.y).abs() + (second.z - first.z).abs()) as usize
}

fn non_diagonal_neighbors(position: &Position) -> Vec<Position> {
    let mut result = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if <i64>::abs(x) + <i64>::abs(y) + <i64>::abs(z) != 1 {
                    continue;
                }
                result.push(Position {
                    x: &position.x + x,
                    y: &position.y + y,
                    z: &position.z + z,
                });
            }
        }
    }
    result
}

struct BoundingBox {
    x: Range<i64>,
    y: Range<i64>,
    z: Range<i64>,
}

impl BoundingBox {
    fn new(positions: &Vec<Position>) -> BoundingBox {
        let x_min = positions
            .iter()
            .min_by(|first, second| first.x.cmp(&second.x))
            .expect("Unable to find minimum x")
            .x;
        let y_min = positions
            .iter()
            .min_by(|first, second| first.y.cmp(&second.y))
            .expect("Unable to find minimum y")
            .y;
        let z_min = positions
            .iter()
            .min_by(|first, second| first.z.cmp(&second.z))
            .expect("Unable to find minimum z")
            .z;
        let x_max = positions
            .iter()
            .max_by(|first, second| first.x.cmp(&second.x))
            .expect("Unable to find maximum x")
            .x;
        let y_max = positions
            .iter()
            .max_by(|first, second| first.y.cmp(&second.y))
            .expect("Unable to find maximum y")
            .y;
        let z_max = positions
            .iter()
            .max_by(|first, second| first.z.cmp(&second.z))
            .expect("Unable to find maximum z")
            .z;

        BoundingBox {
            x: x_min..x_max + 1,
            y: y_min..y_max + 1,
            z: z_min..z_max + 1,
        }
    }

    fn enlarge(&mut self, amount: i64) {
        self.x = (self.x.start - amount)..(self.x.end + amount);
        self.y = (self.y.start - amount)..(self.y.end + amount);
        self.z = (self.z.start - amount)..(self.z.end + amount);
    }

    fn clip_positions(&self, positions: Vec<Position>) -> Vec<Position> {
        positions
            .into_iter()
            .filter(|p| self.x.contains(&p.x) && self.y.contains(&p.y) && self.z.contains(&p.z))
            .collect_vec()
    }
}

fn compute_surface_area(positions: &Vec<Position>) -> usize {
    let mut surface_area = 0;
    for p1 in &*positions {
        let mut cube_area: usize = 6;
        for p2 in &*positions {
            if p1 == p2 {
                continue;
            }
            if manhattan_distance(p1, p2) == 1 {
                cube_area = cube_area.saturating_sub(1);
            }
        }
        surface_area += cube_area;
    }
    surface_area
}

fn compute_outer_surface_area(positions: &Vec<Position>) -> usize {
    let solid_positions : HashSet<Position> = HashSet::from_iter(positions.iter().cloned());
    let mut surface_area = 0;
    let mut bounding_box = BoundingBox::new(&positions);
    bounding_box.enlarge(1);
    let mut open_positions = Vec::new();
    open_positions.push(Position {
        x: bounding_box.x.start,
        y: bounding_box.y.start,
        z: bounding_box.z.start,
    });

    let mut closed_positions = HashSet::new();

    loop {
        let current_position = open_positions.pop().unwrap();

        if !closed_positions.contains(&current_position) {
            let mut neighbors = non_diagonal_neighbors(&current_position);
            closed_positions.insert(current_position);
            neighbors = bounding_box.clip_positions(neighbors);

            for neighbor in neighbors {
                if closed_positions.contains(&neighbor) {
                    continue;
                }
                if solid_positions.contains(&neighbor) {
                    surface_area += 1
                } else {
                    open_positions.push(neighbor);
                }
            }
        }
        if open_positions.is_empty() {
            break;
        }
    }

    surface_area
}

fn main() {
    if let Ok(lines) = read_input("inputs/day18.txt") {
        let positions = lines
            .iter()
            .map(<String>::as_str)
            .map(parse_position)
            .map(|p| p.expect("Unable to parse position."))
            .collect_vec();
        let surface_area = compute_surface_area(&positions);
        println!("The surface area is {}", surface_area);

        let surface_area = compute_outer_surface_area(&positions);
        println!("The outer surface area is {}", surface_area);
    } else {
        println!("Couldn't read input.");
    }
}
