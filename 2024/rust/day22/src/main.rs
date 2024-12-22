fn main() {
    static INPUT: &str = include_str!("../../../day22.txt");

    let start = std::time::Instant::now();

    let inputs = INPUT
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for mut n in inputs.iter().copied() {
        for _ in 0..2000 {
            n = next_secret(n);
        }
        part1 += n as u64;
    }

    let mut all_sequence_prices = [0; 126892];
    for mut n in inputs.into_iter() {
        let mut seen_changes = [false; 126892];

        let mut units = [0; 2001];
        for i in 0..2001 {
            units[i] = (n % 10) as i8;
            n = next_secret(n);
        }
        for xs in units.windows(5) {
            let changes = 65160
                + xs[4] as usize
                + 18 * xs[3] as usize
                + 342 * xs[2] as usize
                + 6498 * xs[1] as usize
                - 6859 * xs[0] as usize;
            if !seen_changes[changes] {
                seen_changes[changes] = true;
                all_sequence_prices[changes] += xs[4] as u16;
            }
        }
    }
    let part2 = all_sequence_prices.into_iter().max().unwrap();

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}

fn next_secret(mut n: u32) -> u32 {
    n ^= n << 6;
    n &= 0b1111_1111_1111_1111_1111_1111;
    n ^= n >> 5;
    n ^= n << 11;
    n & 0b1111_1111_1111_1111_1111_1111
}
