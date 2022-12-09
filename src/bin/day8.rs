use itertools::enumerate;
use std::slice::Iter;
use utilities::read_input;

type Value = usize;
type Row = Vec<Value>;

#[derive(Debug, PartialEq)]
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn from(rows: Vec<String>) -> Grid {
        let mut grid_rows = Vec::new();
        for row in rows.iter().map(parse_row) {
            grid_rows.push(row)
        }
        Grid { rows: grid_rows }
    }
    fn row_iter(&self) -> Iter<Row> {
        self.rows.iter()
    }

    fn column_iter(&self) -> ColumnIterator {
        let row_iters: Vec<_> = self.rows.iter().map(|r| r.iter()).collect();
        ColumnIterator {
            position: row_iters,
        }
    }

    fn is_empty(&self) -> bool {
        if self.rows.is_empty() {
            return true;
        }

        if self.rows.first().unwrap().is_empty() {
            return true;
        }
        false
    }

    fn is_visible_in_line<'a, T>(iter: T, index: usize) -> bool
    where
        T: Iterator<Item = &'a usize>,
    {
        if index == 0 {
            return true;
        }
        let values = iter.collect::<Vec<_>>();
        let target = *values.get(index).expect("Invalid index!");
        for (current_index, value) in enumerate(values) {
            if current_index == index {
                return true;
            }
            if value >= target {
                return false;
            }
        }
        false
    }

    fn is_visible(&self, row: usize, column: usize) -> Option<bool> {
        if self.is_empty() {
            return None;
        }

        let column_count = self.rows.first().unwrap().len();
        let row_count = self.rows.len();

        if column == 0
            || column == column_count.saturating_sub(1)
            || row == 0
            || row == row_count.saturating_sub(1)
        {
            return Some(true);
        }

        let row_iter = (*self.row_iter().nth(row)?).iter();

        if Grid::is_visible_in_line(row_iter.clone(), column)
            || Grid::is_visible_in_line(
                row_iter.rev(),
                column_count.saturating_sub(1).saturating_sub(column),
            )
        {
            return Some(true);
        }
        let column_values = self.column_iter().nth(column)?;
        let column_iter = column_values.iter().map(|v| *v);

        if Grid::is_visible_in_line(column_iter.clone(), row)
            || Grid::is_visible_in_line(
                column_iter.rev(),
                row_count.saturating_sub(1).saturating_sub(row),
            )
        {
            return Some(true);
        }

        Some(false)
    }

    fn count_visible(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let mut visible_count = 0;
        for column in 0..self.rows.first().unwrap().len() {
            for row in 0..self.rows.len() {
                if self.is_visible(row, column)? {
                    visible_count += 1;
                }
            }
        }

        Some(visible_count)
    }
}

struct ColumnIterator<'a> {
    position: Vec<Iter<'a, Value>>,
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = Vec<&'a Value>;

    fn next(&mut self) -> Option<Self::Item> {
        let column: Vec<_> = self.position.iter_mut().filter_map(|r| r.next()).collect();
        if column.is_empty() {
            None
        } else {
            Some(column)
        }
    }
}

impl<'a> DoubleEndedIterator for ColumnIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let column: Vec<_> = self
            .position
            .iter_mut()
            .filter_map(|r| r.next_back())
            .collect();
        if column.is_empty() {
            None
        } else {
            Some(column)
        }
    }
}

fn parse_row<T>(line: T) -> Row
where
    T: Into<String>,
{
    line.into()
        .chars()
        .map(|c| {
            c.to_string()
                .parse::<Value>()
                .expect("Couldn't parse number")
        })
        .collect()
}

fn main() {
    if let Ok(lines) = read_input("inputs/day8.txt") {
        let grid = Grid::from(lines);
        println!("There are {:?} visible trees!", grid.count_visible());
    } else {
        println!("Couldn't read input.")
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_row("30373"), vec![3, 0, 3, 7, 3]);
        assert_eq!(parse_row("25512"), vec![2, 5, 5, 1, 2]);
        assert_eq!(parse_row("65332"), vec![6, 5, 3, 3, 2]);
        assert_eq!(parse_row("33549"), vec![3, 3, 5, 4, 9]);
        assert_eq!(parse_row("35390"), vec![3, 5, 3, 9, 0]);
    }

    fn grid_test_value() -> Grid {
        Grid {
            rows: vec![vec![0, 1], vec![2, 3]],
        }
    }

    #[test]
    fn test_grid() {
        let lines = vec!["01", "23"].iter().map(<&str>::to_string).collect();
        let grid = Grid::from(lines);
        assert_eq!(grid, grid_test_value());
    }

    #[test]
    fn test_grid_row_iter() {
        let grid = grid_test_value();
        let mut row_iter = grid.row_iter();
        let first_row = row_iter.next();
        let second_row = row_iter.next();
        let third_row = row_iter.next();

        assert!(first_row.is_some());
        assert!(second_row.is_some());
        assert_eq!(third_row, None);

        assert_eq!(*first_row.unwrap(), vec![0, 1]);
        assert_eq!(*second_row.unwrap(), vec![2, 3]);
    }

    #[test]
    fn test_grid_row_iter_rev() {
        let grid = grid_test_value();
        let mut row_iter = grid.row_iter().rev();
        let first_row = row_iter.next();
        let second_row = row_iter.next();
        let third_row = row_iter.next();

        assert!(first_row.is_some());
        assert!(second_row.is_some());
        assert_eq!(third_row, None);

        assert_eq!(*first_row.unwrap(), vec![2, 3]);
        assert_eq!(*second_row.unwrap(), vec![0, 1]);
    }

    #[test]
    fn test_grid_column_iter() {
        let grid = grid_test_value();
        let mut column_iter = grid.column_iter();
        let first_column = column_iter.next();
        let second_column = column_iter.next();
        let third_column = column_iter.next();

        assert!(first_column.is_some());
        assert!(second_column.is_some());
        assert_eq!(third_column, None);

        assert_eq!(
            first_column
                .unwrap()
                .iter()
                .map(|c| **c)
                .collect::<Vec<_>>(),
            vec![0, 2]
        );
        assert_eq!(
            second_column
                .unwrap()
                .iter()
                .map(|c| **c)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
    }
    #[test]
    fn test_grid_column_iter_rev() {
        let grid = grid_test_value();
        let mut column_iter = grid.column_iter().rev();
        let first_column = column_iter.next();
        let second_column = column_iter.next();
        let third_column = column_iter.next();

        assert!(first_column.is_some());
        assert!(second_column.is_some());
        assert_eq!(third_column, None);

        assert_eq!(
            first_column
                .unwrap()
                .iter()
                .map(|c| **c)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
        assert_eq!(
            second_column
                .unwrap()
                .iter()
                .map(|c| **c)
                .collect::<Vec<_>>(),
            vec![0, 2]
        );
    }

    #[test]
    fn test_is_visible_in_line() {
        assert_eq!(Grid::is_visible_in_line(vec![0, 5].iter(), 1), true);
        assert_eq!(Grid::is_visible_in_line(vec![2, 5].iter(), 1), true);
        assert_eq!(Grid::is_visible_in_line(vec![3, 5].iter(), 1), true);
        assert_eq!(Grid::is_visible_in_line(vec![2, 1].iter(), 1), false);
        assert_eq!(Grid::is_visible_in_line(vec![2, 1].iter(), 0), true);
        assert_eq!(Grid::is_visible_in_line(vec![7, 1].iter(), 1), false);
        assert_eq!(Grid::is_visible_in_line(vec![7, 1].iter(), 0), true);

        assert_eq!(
            Grid::is_visible_in_line(vec![6, 5, 3, 3, 2].iter(), 3),
            false
        );
        assert_eq!(
            Grid::is_visible_in_line(vec![6, 5, 3, 3, 2].iter().rev(), 1),
            true
        );

        assert_eq!(
            Grid::is_visible_in_line(vec![2, 5, 5, 1, 2].iter(), 3),
            false
        );
        assert_eq!(
            Grid::is_visible_in_line(vec![2, 5, 5, 1, 2].iter().rev(), 1),
            false
        );

        assert_eq!(
            Grid::is_visible_in_line(vec![7, 1, 3, 4, 9].iter(), 1),
            false
        );
        assert_eq!(
            Grid::is_visible_in_line(vec![7, 1, 3, 4, 9].iter().rev(), 3),
            false
        );

        assert_eq!(
            Grid::is_visible_in_line(vec![6, 5, 3, 3, 2].iter(), 1),
            false
        );
        assert_eq!(
            Grid::is_visible_in_line(vec![6, 5, 3, 3, 2].iter().rev(), 3),
            true
        );
    }

    fn example_grid() -> Grid {
        Grid {
            rows: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        }
    }

    #[test]
    fn test_is_visible() {
        let grid = example_grid();

        for i in 0..4 {
            assert_eq!(grid.is_visible(i, 0), Some(true));
            assert_eq!(grid.is_visible(i, 4), Some(true));
            assert_eq!(grid.is_visible(0, i), Some(true));
            assert_eq!(grid.is_visible(4, i), Some(true));
        }

        assert_eq!(grid.is_visible(1, 1), Some(true));
        assert_eq!(grid.is_visible(1, 2), Some(true));
        assert_eq!(grid.is_visible(1, 3), Some(false));
        assert_eq!(grid.is_visible(2, 1), Some(true));
        assert_eq!(grid.is_visible(2, 2), Some(false));
        assert_eq!(grid.is_visible(2, 3), Some(true));
        assert_eq!(grid.is_visible(3, 1), Some(false));
        assert_eq!(grid.is_visible(3, 2), Some(true));
        assert_eq!(grid.is_visible(3, 3), Some(false));
    }

    #[test]
    fn test_count_visible() {
        assert_eq!(example_grid().count_visible(), Some(21));
    }
}
