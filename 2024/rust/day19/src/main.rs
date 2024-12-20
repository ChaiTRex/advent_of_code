use std::collections::HashMap;

fn main() {
    static INPUT: &str = include_str!("../../../day19.txt");

    let start = std::time::Instant::now();

    let towels = INPUT
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;
    for design in INPUT.lines().skip(2) {
        let mut memoizer = HashMap::new();
        memoizer.insert(design.len(), 1);
        let different_ways = f(&mut memoizer, 0, design, &towels);
        if different_ways != 0 {
            part1 += 1;
            part2 += different_ways;
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}

fn f(memoizer: &mut HashMap<usize, usize>, i: usize, design: &str, towels: &[&str]) -> usize {
    if memoizer.contains_key(&i) {
        *memoizer.get(&i).unwrap()
    } else {
        let count = towels
            .into_iter()
            .filter(|&towel| design[i..].starts_with(towel))
            .map(|towel| f(memoizer, i + towel.len(), design, towels))
            .sum::<usize>();
        memoizer.insert(i, count);
        count
    }
}
