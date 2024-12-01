use core::cmp::Ordering;

static TIMING: bool = false;

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

    if TIMING {
        core::hint::black_box(part1);
    } else {
        println!("{part1}");
    }

    fn catch_up(
        values: &mut core::iter::Peekable<impl Iterator<Item = u32>>,
        target_value: u32,
    ) -> bool {
        loop {
            match values.peek().copied() {
                None => return false,
                Some(x) => {
                    if x >= target_value {
                        return true;
                    } else {
                        values.next();
                    }
                }
            }
        }
    }

    let mut part2 = 0;

    let mut left_values = left.into_iter().peekable();
    let mut right_values = right.into_iter().peekable();
    'outer: loop {
        match (left_values.peek().copied(), right_values.peek().copied()) {
            (None, _) => break 'outer,
            (_, None) => break 'outer,
            (Some(l), Some(r)) => match l.cmp(&r) {
                Ordering::Equal => {
                    let mut left_count = 0;
                    while left_values.peek().copied() == Some(l) {
                        left_values.next();
                        left_count += 1;
                    }
                    let mut right_count = 0;
                    while right_values.peek().copied() == Some(r) {
                        right_values.next();
                        right_count += 1;
                    }
                    part2 += l * left_count * right_count;
                }
                Ordering::Less => {
                    if !catch_up(&mut left_values, r) {
                        break 'outer;
                    }
                }
                Ordering::Greater => {
                    if !catch_up(&mut right_values, l) {
                        break 'outer;
                    }
                }
            },
        }
    }

    if TIMING {
        core::hint::black_box(part2);
        let end = std::time::Instant::now();
        println!("{:?}", end - start);
    } else {
        println!("{part2}");
    }
}
