use itertools::Itertools;
use utilities::*;

fn contains_no_duplicates<T>(input: &[T]) -> bool
where
    T: PartialEq,
{
    !(1..input.len()).any(|i| input[i..].contains(&input[i - 1]))
}

fn find_packet_marker(input: &String, length: usize) -> Result<usize, String> {
    match input
        .chars()
        .collect::<Vec<char>>()
        .windows(length)
        .find_position(|window| contains_no_duplicates(window))
    {
        Some((index, _)) => Ok(index + length),
        None => Err("Couldn't find start-of-packet marker".to_string()),
    }
}

enum MarkerType {
    PacketStart = 4,
    MessageStart = 14,
}

fn scan_for_marker(input: &String, marker_type: MarkerType) {
    match find_packet_marker(&input, marker_type as usize) {
        Ok(marker_index) => println!("Found marker at {}", marker_index),
        Err(msg) => println!("Couldn't find marker: {}", msg),
    }
}

fn main() {
    match read_input("inputs/day6.txt") {
        Ok(lines) => match lines.get(0) {
            Some(line) => {
                scan_for_marker(&line.clone(), MarkerType::PacketStart);
                scan_for_marker(&line, MarkerType::MessageStart);
            }

            None => println!("Expecting one line of input. Got none."),
        },
        Err(msg) => println!("Couldn't read input: {}", msg),
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_containts_no_duplicates() {
        assert_eq!(contains_no_duplicates(&[0, 1]), true);
        assert_eq!(contains_no_duplicates(&[0, 0]), false);
        assert_eq!(contains_no_duplicates(&['m', 'j', 'q', 'j']), false);
        assert_eq!(contains_no_duplicates(&['j', 'q', 'j', 'p']), false);
        assert_eq!(contains_no_duplicates(&['q', 'j', 'p', 'q']), false);
        assert_eq!(contains_no_duplicates(&['j', 'p', 'q', 'm']), true);
    }

    #[test]
    fn test_find_packet_marker() {
        // packet start markers of length 4
        assert_eq!(
            find_packet_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4),
            Ok(5)
        );
        assert_eq!(
            find_packet_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4),
            Ok(6)
        );
        assert_eq!(
            find_packet_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4),
            Ok(10)
        );
        assert_eq!(
            find_packet_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),
            Ok(11)
        );
        // message markers of length 14
        assert_eq!(
            find_packet_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),
            Ok(19)
        );
        assert_eq!(
            find_packet_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
            Ok(23)
        );
        assert_eq!(
            find_packet_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),
            Ok(23)
        );
        assert_eq!(
            find_packet_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),
            Ok(29)
        );
        assert_eq!(
            find_packet_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),
            Ok(26)
        );
    }
}
