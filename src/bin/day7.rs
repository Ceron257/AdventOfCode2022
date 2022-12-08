use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::slice::Iter;
use utilities::*;

#[derive(PartialEq, Debug)]
struct Command {
    command: String,
    argument: Option<String>,
}

fn parse_command_line(input: &String) -> Option<Command> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\$\s(?P<command>cd|ls)\s?(?P<argument>.*)?").unwrap();
    }
    let captures = RE.captures(input)?;
    let (command, argument) = (captures.name("command")?, captures.name("argument")?);
    Some(Command {
        command: command.as_str().to_string(),
        argument: Some(argument.as_str().to_string()),
    })
}

fn parse_dir_line(input: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(dir)\s(?P<name>.*)").unwrap();
    }
    let matches = RE.captures(input)?;
    let name = matches.name("name")?;
    Some(name.as_str().to_string())
}

#[derive(PartialEq, Debug)]
struct File {
    name: String,
    size: usize,
}

fn parse_file_line(input: &String) -> Option<File> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<size>\d*)\s(?P<name>.*)").unwrap();
    }
    let matches = RE.captures(input)?;
    let (name, size) = (matches.name("name")?, matches.name("size")?);
    Some(File {
        name: name.as_str().to_string(),
        size: if let Ok(size) = size.as_str().parse::<usize>() {
            size
        } else {
            return None;
        },
    })
}

fn append_path(input: &mut String, item: String) {
    if input != "/" {
        input.push_str("/");
    }
    input.push_str(item.as_str());
}

fn parse_directory_tree(mut iter: Iter<String>) -> HashMap<String, usize> {
    let mut directory_sizes: HashMap<String, usize> = HashMap::new();
    directory_sizes.entry("/".to_string()).or_default();
    let mut path: Vec<String> = Vec::new();

    while let Some(line) = iter.next() {
        if let Some(command) = parse_command_line(&line.clone()) {
            match command.command.as_str() {
                "cd" => {
                    if let Some(argument) = command.argument {
                        match argument.as_str() {
                            ".." => match path.pop() {
                                Some(_) => (),
                                None => panic!("There is not parent directory for '/'!"),
                            },
                            "/" => (),
                            _ => path.push(argument),
                        };
                    } else {
                        panic!("cd needs an argument!");
                    }
                }
                "ls" => (),
                _ => panic!("Unknown command: {}", command.command),
            };
        }

        if let Some(directory) = parse_dir_line(&line.clone()) {
            let mut path = path.join("/");
            path.insert(0, '/');
            append_path(&mut path, directory);
            directory_sizes.entry(path).or_default();
        }

        if let Some(file) = parse_file_line(line) {
            let mut path = path.join("/");
            path.insert(0, '/');
            *directory_sizes.entry(path.clone()).or_insert(0) += file.size;
        }
    }

    let mut accumulated_sizes = HashMap::new();

    for dir in directory_sizes.keys() {
        for (path, size) in directory_sizes.iter() {
            if path.starts_with(dir) {
                *accumulated_sizes.entry(dir.clone()).or_insert(0) += size;
            }
        }
    }
    accumulated_sizes
}

fn main() {
    if let Ok(input) = read_input("inputs/day7.txt") {
        let directory_sizes = parse_directory_tree(input.iter());
        let accumulated_directory_size =
            directory_sizes
                .iter()
                .fold(0 as usize, |accumulator, element| {
                    if element.1 <= &100000 {
                        accumulator + element.1
                    } else {
                        accumulator
                    }
                });
        println!(
            "All directories' sizes lower than 100000 sum up to = {}",
            accumulated_directory_size
        );

        let required_space = directory_sizes.get("/").unwrap() - (70000000 - 30000000);
        match directory_sizes
            .values()
            .sorted()
            .find(|element| element >= &&required_space)
        {
            Some(size) => println!("The size of the to be deleted directory is {}", size),
            None => println!("Unable to find directory to delete!"),
        }
    } else {
        println!("Unable to read input.");
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_parse_command_line() {
        assert_eq!(
            parse_command_line(&"$ cd /".to_string()),
            Some(Command {
                command: "cd".to_string(),
                argument: Some("/".to_string())
            })
        );
        assert_eq!(
            parse_command_line(&"$ ls".to_string()),
            Some(Command {
                command: "ls".to_string(),
                argument: Some("".to_string())
            })
        );
        assert_eq!(
            parse_command_line(&"$ cd a".to_string()),
            Some(Command {
                command: "cd".to_string(),
                argument: Some("a".to_string())
            })
        );
        assert_eq!(
            parse_command_line(&"$ cd a123bz".to_string()),
            Some(Command {
                command: "cd".to_string(),
                argument: Some("a123bz".to_string())
            })
        );
        assert_eq!(
            parse_command_line(&"$ cd ..".to_string()),
            Some(Command {
                command: "cd".to_string(),
                argument: Some("..".to_string())
            })
        );
        assert_eq!(parse_command_line(&"dir a123bz".to_string()), None);
        assert_eq!(parse_command_line(&"29116 f".to_string()), None);
    }

    #[test]
    fn parse_parse_dir_line() {
        assert_eq!(parse_dir_line(&"$ cd a123bz".to_string()), None);
        assert_eq!(
            parse_dir_line(&"dir a123bz".to_string()),
            Some("a123bz".to_string())
        );
        assert_eq!(parse_dir_line(&"29116 f".to_string()), None);
    }

    #[test]
    fn parse_parse_file_line() {
        assert_eq!(parse_file_line(&"$ cd a123bz".to_string()), None);
        assert_eq!(parse_file_line(&"dir a123bz".to_string()), None);
        assert_eq!(
            parse_file_line(&"29116 f".to_string()),
            Some(File {
                name: "f".to_string(),
                size: 29116
            })
        );
        assert_eq!(
            parse_file_line(&"5626152 d.ext".to_string()),
            Some(File {
                name: "d.ext".to_string(),
                size: 5626152
            })
        );
    }
}
