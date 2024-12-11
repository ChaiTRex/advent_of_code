use std::collections::HashMap;

fn main() {
    static INPUT: &str = include_str!("../../../day11.txt");

    let start = std::time::Instant::now();

    let mut counts = HashMap::new();
    for stone in INPUT
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
    {
        *counts.entry(stone).or_insert(0) += 1;
    }

    let mut part1 = 0;
    for i in 0..75 {
        if i == 25 {
            part1 = counts.values().sum::<u64>();
        }
        counts = {
            let mut tmp = HashMap::new();
            for (stone, count) in counts {
                if stone == 0 {
                    *tmp.entry(1).or_insert(0) += count;
                } else {
                    let digit_count = stone.ilog10() + 1;
                    if digit_count & 1 == 0 {
                        let splitting_power_of_ten = 10_u64.pow(digit_count / 2);
                        *tmp.entry(stone / splitting_power_of_ten).or_insert(0) += count;
                        *tmp.entry(stone % splitting_power_of_ten).or_insert(0) += count;
                    } else {
                        *tmp.entry(2024 * stone).or_insert(0) += count;
                    }
                }
            }
            tmp
        };
    }
    let part2 = counts.values().sum::<u64>();

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
