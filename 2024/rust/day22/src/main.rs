use std::collections::{HashMap, HashSet};

fn main() {
    static INPUT: &str = include_str!("../../../day22.txt");

    let mut part1 = 0;
    for mut n in INPUT.lines().map(|n| n.parse::<u32>().unwrap()) {
        //let orig = n;
        for _ in 0..2000 {
            n = next_secret(n);
        }
        part1 += n as u64;
        //println!("{orig}: {n}");
    }
    println!("{part1}");

    let mut all_sequences = HashSet::new();
    let mut all_sequence_prices = Vec::new();
    for mut n in INPUT.lines().map(|n| n.parse::<u32>().unwrap()) {
        let mut sequence_prices = HashMap::new();

        let mut units = Vec::with_capacity(2000);
        for _ in 0..2001 {
            units.push(n as i32 % 10);
            n = next_secret(n);
        }
        for xs in units.windows(5) {
            let changes = [xs[1] - xs[0], xs[2] - xs[1], xs[3] - xs[2], xs[4] - xs[3]];
            all_sequences.insert(changes);
            sequence_prices.entry(changes).or_insert(xs[4]);
        }

        all_sequence_prices.push(sequence_prices);
    }
    let part2 = all_sequences
        .into_iter()
        .map(|sequence| {
            all_sequence_prices
                .iter()
                .flat_map(|sequence_prices| sequence_prices.get(&sequence).map(|&n| n as i64))
                .sum::<i64>()
        })
        .max()
        .unwrap();

    println!("{part2}");
}

fn next_secret(mut n: u32) -> u32 {
    n = (n ^ (n << 6)) & 0b1111_1111_1111_1111_1111_1111;
    n = (n ^ (n >> 5)) & 0b1111_1111_1111_1111_1111_1111;
    n = (n ^ (n << 11)) & 0b1111_1111_1111_1111_1111_1111;
    n
}
