use aoc_2021::days::day2::{part_1, part_2};

fn main() {
    pretty_env_logger::init();
    let first = part_1();
    let second = part_2();

    println!("first: {} and second: {}", first, second);
}
