use std::collections::HashMap;

fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day11.txt");

    let start = std::time::Instant::now();

    let mut stone_counts = HashMap::with_capacity(4000);
    let mut stone_counts_in_loop = HashMap::with_capacity(4000);

    let mut i = 0;
    let mut stone = 0;
    loop {
        if i == INPUT.len() || INPUT[i] == b'\n' {
            *stone_counts.entry(stone).or_default() += 1;
            break;
        } else if INPUT[i] == b' ' {
            *stone_counts.entry(stone).or_default() += 1;
            stone = 0;
        } else {
            stone = 10 * stone + (INPUT[i] - b'0') as u64;
        }
        i += 1;
    }

    let mut part1 = 0;
    for i in 0..75 {
        if i == 25 {
            part1 = stone_counts.values().sum::<u64>();
        }

        for (&stone, &count) in stone_counts.iter() {
            if stone == 0 {
                *stone_counts_in_loop.entry(1).or_default() += count;
            } else {
                let digit_count_minus_one = stone.ilog10();
                if digit_count_minus_one % 2 != 0 {
                    let splitting_power_of_ten = 10_u64.pow((digit_count_minus_one + 1) / 2);
                    *stone_counts_in_loop
                        .entry(stone / splitting_power_of_ten)
                        .or_default() += count;
                    *stone_counts_in_loop
                        .entry(stone % splitting_power_of_ten)
                        .or_default() += count;
                } else {
                    *stone_counts_in_loop.entry(2024 * stone).or_default() += count;
                }
            }
        }
        core::mem::swap(&mut stone_counts, &mut stone_counts_in_loop);
        stone_counts_in_loop.clear();
    }
    let part2 = stone_counts.values().sum::<u64>();

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
