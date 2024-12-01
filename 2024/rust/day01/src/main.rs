fn main() {
    static INPUT: &str = include_str!("../../../day01.txt");

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<u32>().unwrap(),
                iter.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let part1 = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();

    println!("{part1}");

    let part2 = left
        .iter()
        .map(|x| x * right.iter().filter(|&y| x == y).count() as u32)
        .sum::<u32>();

    println!("{part2}");
}
