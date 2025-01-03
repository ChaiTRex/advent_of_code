fn main() {
    static INPUT: &str = include_str!("../../../day01.txt");

    let start = std::time::Instant::now();

    let mut left = [0_u32; 1000];
    let mut right = [0_u32; 1000];
    for (i, (l, r)) in INPUT
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .enumerate()
    {
        left[i] = l;
        right[i] = r;
    }
    left.sort_unstable();
    right.sort_unstable();

    let part1 = left
        .iter()
        .copied()
        .zip(right.iter().copied())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u32>();

    let mut part2 = 0;

    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();

    'left_loop: while let Some(l) = left.next() {
        let mut left_product = l;
        while left.peek() == Some(&l) {
            left.next();
            left_product += l;
        }

        loop {
            match right.peek() {
                Some(&r) if r < l => {
                    right.next();
                }
                Some(_) => break,
                None => break 'left_loop,
            }
        }

        loop {
            match right.peek() {
                Some(&r) if r == l => {
                    right.next();
                    part2 += left_product;
                }
                Some(_) => break,
                None => break 'left_loop,
            }
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
