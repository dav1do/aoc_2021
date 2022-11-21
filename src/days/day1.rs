
pub fn part_1() -> u32 {
    let input = include_str!("../inputs/1.txt");
    let input = input.split("\n").collect::<Vec<&str>>();
    let increases = count_increases(input);
    increases
}

pub fn part_2() -> u32 {
    let input = include_str!("../inputs/1.txt");
    let input = input.split("\n").collect::<Vec<&str>>();
    let increases = count_sliding_3_window(input);
    increases
}

fn count_increases(input: Vec<&str>) -> u32 {
    let mut increases = 0;
    for (i, elem) in input.iter().enumerate() {
        if let Some(next) = input.get(i + 1).as_ref().map(|v| v.parse::<u32>().unwrap()) {
            let current = elem.parse::<u32>().unwrap();

            log::debug!("{} > {} => {}", next, current, next > current);
            if next > current {
                increases += 1;
            }
        }
    }
    increases
}

fn count_sliding_3_window(input: Vec<&str>) -> u32 {
    let mut increases = 0;
    for (i, _) in input.iter().enumerate() {
        let first = get_window(&input, i, 3);
        let next = get_window(&input, i + 1, 3);
        if next.is_none() {
            break;
        }

        log::debug!("first: {:?} and next: {:?}", first, next);
        let f_sum: u32 = first.map_or(0, |i| i.iter().sum());
        let s_sum: u32 = next.map_or(0, |i| i.iter().sum());
        log::debug!("{} > {} => {}", s_sum, f_sum, s_sum > f_sum);
        if s_sum > f_sum {
            increases += 1;
        }
    }

    increases
}

fn get_window(input: &[&str], start: usize, length: usize) -> Option<Vec<u32>> {
    let iter = input.iter().skip(start).take(length);

    if iter.len() == length {
        let mut window = Vec::new();
        for elem in iter {
            let current = elem.parse::<u32>().unwrap();
            window.push(current);
        }

        Some(window)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{count_increases, count_sliding_3_window};

    const STATIC_INPUT: &'static str = "199,200,208,210,200,207,240,269,260,263";

    #[test]
    fn part1() {
        let input = STATIC_INPUT.split(",").collect::<Vec<&str>>();
        assert_eq!(7, count_increases(input));
    }

    #[test]
    fn part2() {
        let input = STATIC_INPUT.split(",").collect::<Vec<&str>>();
        assert_eq!(5, count_sliding_3_window(input));
    }
}
