use std::str::FromStr;

enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{} is an invalid direction", s),
                )
            )
        }
    }
}

struct Command {
    direction: Direction,
    distance: u32,
}

// https://fettblog.eu/rust-enums-wrapping-errors/
enum CommandParseError {
    ParseError(std::num::ParseIntError),
    IoError(std::io::Error),
}

impl From<std::num::ParseIntError> for CommandParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        CommandParseError::ParseError(err)
    }
}

impl From<std::io::Error> for CommandParseError {
    fn from(err: std::io::Error) -> Self {
        CommandParseError::IoError(err)
    }
}


impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(" ").collect();
        Ok(Command {
            direction: input[0].parse::<Direction>()?,
            distance: input[1].parse::<u32>()?,
        })
    }
}

pub fn execute() {
    let input = include_str!("input.txt");
    let commands: Vec<Command> = input.lines().filter_map(|s| s.parse::<Command>().ok()).collect();
    println!("Distance travelled: {0}", distance_travelled(&commands));
}


fn distance_travelled(commands: &[Command]) -> u32 {
    let mut depth = 0;
    let mut distance = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => distance += command.distance,
            Direction::Up => depth -= command.distance,
            Direction::Down => depth += command.distance
        }
    }

    return depth * distance;
}