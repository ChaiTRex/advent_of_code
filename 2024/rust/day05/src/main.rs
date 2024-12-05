fn main() {
    static INPUT: &str = include_str!("../../../day05.txt");

    let start = std::time::Instant::now();

    let (rules, candidates) = INPUT.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|rule| {
            let (lower, higher) = rule.split_once('|').unwrap();
            let lower = lower.parse::<u32>().unwrap();
            let higher = higher.parse::<u32>().unwrap();
            move |a: u32, b: u32| !(b == lower && a == higher)
        })
        .collect::<Vec<_>>();
    let candidates = candidates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let winners = candidates.iter().filter(|candidate| {
        for i in 0..candidate.len() - 1 {
            for j in i + 1..candidate.len() {
                for rule in &rules {
                    if !rule(candidate[i], candidate[j]) {
                        return false;
                    }
                }
            }
        }
        true
    });

    let mut part1 = 0;
    for winner in winners {
        part1 += winner[winner.len() / 2];
    }

    let mut part2 = 0;
    let losers = candidates
        .into_iter()
        .filter(|candidate| {
            for i in 0..candidate.len() - 1 {
                for j in i + 1..candidate.len() {
                    for rule in &rules {
                        if !rule(candidate[i], candidate[j]) {
                            return true;
                        }
                    }
                }
            }
            false
        })
        .collect::<Vec<_>>();
    for mut loser in losers {
        loser.sort_by(|a, b| {
            if a == b {
                return core::cmp::Ordering::Equal;
            }
            for rule in &rules {
                if !rule(*a, *b) {
                    return core::cmp::Ordering::Greater;
                }
            }
            return core::cmp::Ordering::Less;
        });
        part2 += loser[loser.len() / 2];
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
