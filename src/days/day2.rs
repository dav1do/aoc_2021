use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiveDirection {
    Forward,
    Up,
    Down,
}

impl std::str::FromStr for DiveDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            s => Err(format!("{} is unknown direction", s)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiveCommand {
    direction: DiveDirection,
    distance: i32,
}

impl DiveCommand {
    pub fn try_new(dir: &str, distance: &str) -> Result<Self, String> {
        let direction = DiveDirection::from_str(dir)?;
        let distance = match distance.parse::<i32>() {
            Ok(d) => d,
            Err(e) => return Err(format!("{} is not a valid distance: {:?}", distance, e)),
        };
        Ok(Self {
            direction,
            distance,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct NaiveDiveLocation {
    x: i32,
    depth: i32,
}

pub trait SubmarineCommandProcessor {
    fn accept_command(&mut self, cmd: DiveCommand);
    fn multipled(&self) -> i32;
    fn x(&self) -> i32;
    fn depth(&self) -> i32;
}

impl SubmarineCommandProcessor for NaiveDiveLocation {
    fn accept_command(&mut self, cmd: DiveCommand) {
        match cmd.direction {
            DiveDirection::Forward => self.x += cmd.distance,
            DiveDirection::Up => self.depth -= cmd.distance,
            DiveDirection::Down => self.depth += cmd.distance,
        }
    }

    fn multipled(&self) -> i32 {
        self.x * self.depth
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn depth(&self) -> i32 {
        self.depth
    }
}

fn parse_input(input: Vec<&str>) -> Vec<DiveCommand> {
    let parsed = input
        .iter()
        .map(|v| {
            let input: Vec<&str> = v.split(" ").collect();
            let command = DiveCommand::try_new(input.first().unwrap(), input.get(1).unwrap());
            command.unwrap()
        })
        .collect::<Vec<DiveCommand>>();
    parsed
}

fn day_2_internal(
    mut location: impl SubmarineCommandProcessor,
    commands: Vec<DiveCommand>,
) -> impl SubmarineCommandProcessor {
    for command in commands.into_iter() {
        location.accept_command(command)
    }
    location
}

/*
--- Day 2: Dive! ---
Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

forward X increases the horizontal position by X units.
down X increases the depth by X units.
up X decreases the depth by X units.
Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2
Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

forward 5 adds 5 to your horizontal position, a total of 5.
down 5 adds 5 to your depth, resulting in a value of 5.
forward 8 adds 8 to your horizontal position, a total of 13.
up 3 decreases your depth by 3, resulting in a value of 2.
down 8 adds 8 to your depth, resulting in a value of 10.
forward 2 adds 2 to your horizontal position, a total of 15.
After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

*/
pub fn part_1() -> i32 {
    let data = include_str!("../inputs/2.txt");
    let data = data.split("\n").collect::<Vec<&str>>();
    let input = parse_input(data);
    let location = day_2_internal(NaiveDiveLocation::default(), input);
    println!(
        "horizontal={}, depth={}, multipled={}",
        location.x(),
        location.depth(),
        location.multipled()
    );
    location.multipled()
}

#[derive(Clone, Debug, Default)]
struct DiveLocation {
    x: i32,
    depth: i32,
    aim: i32,
}

impl SubmarineCommandProcessor for DiveLocation {
    fn accept_command(&mut self, cmd: DiveCommand) {
        match cmd.direction {
            DiveDirection::Forward => {
                self.x += cmd.distance;
                self.depth += self.aim * cmd.distance
            }
            DiveDirection::Up => self.aim -= cmd.distance,
            DiveDirection::Down => self.aim += cmd.distance,
        }
    }

    fn multipled(&self) -> i32 {
        self.depth * self.x
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn depth(&self) -> i32 {
        self.depth
    }
}

/*
--- Part Two ---
Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.

In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:

down X increases your aim by X units.
up X decreases your aim by X units.
forward X does two things:
It increases your horizontal position by X units.
It increases your depth by your aim multiplied by X.
Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.

Now, the above example does something different:

forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
down 5 adds 5 to your aim, resulting in a value of 5.
forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
up 3 decreases your aim by 3, resulting in a value of 2.
down 8 adds 8 to your aim, resulting in a value of 10.
forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)

Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
*/
pub fn part_2() -> i32 {
    let data = include_str!("../inputs/2.txt");
    let data = data.split("\n").collect::<Vec<&str>>();
    let input = parse_input(data);
    let location = day_2_internal(DiveLocation::default(), input);
    println!(
        "horizontal={}, depth={}, multipled={}",
        location.x(),
        location.depth(),
        location.multipled()
    );
    location.multipled()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "forward 5,down 5,forward 8,up 3,down 8,forward 2";

    #[test]
    fn part1() {
        let data = TEST_INPUT.split(",").collect::<Vec<&str>>();
        let input = parse_input(data);
        let location = day_2_internal(NaiveDiveLocation::default(), input);
        assert_eq!(15, location.x());
        assert_eq!(10, location.depth());
        assert_eq!(150, location.multipled())
    }

    #[test]
    fn part2() {
        let data = TEST_INPUT.split(",").collect::<Vec<&str>>();
        let input = parse_input(data);
        let location = day_2_internal(DiveLocation::default(), input);
        assert_eq!(15, location.x());
        assert_eq!(60, location.depth());
        assert_eq!(900, location.multipled())
    }
}
