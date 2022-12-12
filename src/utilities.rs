use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_input<P>(file_name: P) -> Result<Vec<String>, String>
where
    P: AsRef<Path>,
{
    let file = File::open(&file_name);
    match file {
        Ok(file) => {
            let mut lines: Vec<String> = Vec::new();
            for line in io::BufReader::new(file).lines() {
                match line {
                    Ok(line) => lines.push(line),
                    Err(err) => {
                        return Err(format!(
                            "Unable to read line from file {:?}: {}",
                            file_name.as_ref().to_str(),
                            err
                        ))
                    }
                }
            }
            Ok(lines)
        }
        Err(err) => Err(format!(
            "Unable to open file {:?}: {}",
            file_name.as_ref().to_str(),
            err
        )),
    }
}

// from: https://rosettacode.org/wiki/Least_common_multiple#Rust
pub fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}