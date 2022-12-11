use std::collections::HashMap;
use utilities::read_input;

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn direction_to_offset(input: &Direction) -> (i32, i32) {
    match input {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Motion {
    direction: Direction,
    distance: usize,
}

fn parse_direction(input: &str) -> Option<Direction> {
    match input {
        "U" => Some(Direction::Up),
        "D" => Some(Direction::Down),
        "L" => Some(Direction::Left),
        "R" => Some(Direction::Right),
        _ => None,
    }
}

fn parse_motion<T>(line: T) -> Option<Motion>
where
    T: Into<String>,
{
    let line_string: String = line.into();
    let mut split = line_string.split(" ");
    let direction = parse_direction(split.next()?)?;
    let distance = split
        .next()?
        .parse::<usize>()
        .expect("Couldn't parse distance!");
    Some(Motion {
        direction,
        distance,
    })
}

type Position = (i32, i32);

struct Snake {
    knot_positions: Vec<Position>,
    visited_positions: HashMap<Position, usize>,
}

fn add_offset(a: Position, offset: Position) -> Position {
    (a.0 + offset.0, a.1 + offset.1)
}

fn are_neighbors(a: &Position, b: &Position) -> bool {
    let distance = (b.0 - a.0, b.1 - a.1);
    let neighbor_range = -1..2;
    neighbor_range.contains(&distance.0) && neighbor_range.contains(&distance.1)
}

fn follow_direction(follower: &Position, leader: &Position) -> Position {
    let offset = (leader.0 - follower.0, leader.1 - follower.1);
    (offset.0.signum(), offset.1.signum())
}

impl Snake {
    fn head(&self) -> &Position {
        self.knot_positions.first().expect("got no head?")
    }

    fn head_mut(&mut self) -> &mut Position {
        self.knot_positions.first_mut().expect("got no head?")
    }

    fn tail(&self) -> &Position {
        self.knot_positions.last().expect("got no tail?")
    }

    fn new(elements: usize) -> Snake {
        let mut knot_positions = Vec::new();
        knot_positions.resize(elements, (0, 0));
        Snake {
            knot_positions,
            visited_positions: HashMap::new(),
        }
    }

    fn apply_motion(&mut self, motion: Motion) {
        let movement_dir = direction_to_offset(&motion.direction);
        for _i in 0..motion.distance {
            *self.head_mut() = add_offset(*self.head(), movement_dir);
            let mut last_position = *self.head();

            for knot in self.knot_positions.iter_mut().skip(1) {
                if !are_neighbors(&last_position, &knot) {
                    *knot = add_offset(*knot, follow_direction(knot, &last_position));
                }
                last_position = *knot;
            }
            *self.visited_positions.entry(*self.tail()).or_default() += 1;
        }
    }
}

fn main() {
    if let Ok(lines) = read_input("inputs/day9.txt") {
        let motions = lines
            .iter()
            .map(parse_motion)
            .map(<Option<Motion>>::unwrap)
            .collect::<Vec<_>>();
        let mut snake = Snake::new(2);
        for motion in motions.clone() {
            snake.apply_motion(motion);
        }
        let visited_position_count = snake.visited_positions.keys().count();
        println!("The tail visied {:?} positions.", visited_position_count);

        let mut snake = Snake::new(10);
        for motion in motions {
            snake.apply_motion(motion);
        }
        let visited_position_count = snake.visited_positions.keys().count();
        println!("The tail visied {:?} positions.", visited_position_count);
    } else {
        println!("Unable to read input!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_motion() {
        assert_eq!(
            parse_motion("R 4"),
            Some(Motion {
                direction: Direction::Right,
                distance: 4
            })
        );
        assert_eq!(
            parse_motion("U 42"),
            Some(Motion {
                direction: Direction::Up,
                distance: 42
            })
        );
        assert_eq!(
            parse_motion("L 10"),
            Some(Motion {
                direction: Direction::Left,
                distance: 10
            })
        );
        assert_eq!(
            parse_motion("D 1"),
            Some(Motion {
                direction: Direction::Down,
                distance: 1
            })
        );
    }

    #[test]
    fn test_snake() {
        let mut snake = Snake::new(2);
        snake.apply_motion(Motion {
            direction: Direction::Right,
            distance: 4,
        });

        assert_eq!(snake.head(), &(4, 0));
        assert_eq!(snake.tail(), &(3, 0));

        assert!(snake.visited_positions.contains_key(&(0, 0)));
        assert!(snake.visited_positions.contains_key(&(1, 0)));
        assert!(snake.visited_positions.contains_key(&(2, 0)));
        assert!(snake.visited_positions.contains_key(&(3, 0)));

        snake.apply_motion(Motion {
            direction: Direction::Up,
            distance: 4,
        });

        assert_eq!(snake.head(), &(4, 4));
        assert_eq!(snake.tail(), &(4, 3));

        assert!(snake.visited_positions.contains_key(&(4, 1)));
        assert!(snake.visited_positions.contains_key(&(4, 2)));
        assert!(snake.visited_positions.contains_key(&(4, 3)));

        snake.apply_motion(Motion {
            direction: Direction::Left,
            distance: 3,
        });

        assert_eq!(snake.head(), &(1, 4));
        assert_eq!(snake.tail(), &(2, 4));

        snake.apply_motion(Motion {
            direction: Direction::Down,
            distance: 1,
        });

        assert_eq!(snake.head(), &(1, 3));
        assert_eq!(snake.tail(), &(2, 4));

        snake.apply_motion(Motion {
            direction: Direction::Right,
            distance: 4,
        });

        assert_eq!(snake.head(), &(5, 3));
        assert_eq!(snake.tail(), &(4, 3));

        snake.apply_motion(Motion {
            direction: Direction::Down,
            distance: 1,
        });

        assert_eq!(snake.head(), &(5, 2));
        assert_eq!(snake.tail(), &(4, 3));

        snake.apply_motion(Motion {
            direction: Direction::Left,
            distance: 5,
        });

        assert_eq!(snake.head(), &(0, 2));
        assert_eq!(snake.tail(), &(1, 2));

        snake.apply_motion(Motion {
            direction: Direction::Right,
            distance: 2,
        });

        assert_eq!(snake.head(), &(2, 2));
        assert_eq!(snake.tail(), &(1, 2));
    }
}
