use std::collections::HashSet;

fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day10.txt");
    const WIDTH: usize = {
        let mut i = 0;
        while INPUT[i].is_ascii_digit() {
            i += 1;
        }
        i
    };
    const LINE_WIDTH: usize = {
        let mut i = WIDTH;
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i + 1
    };
    const HEIGHT: usize = INPUT.len() / LINE_WIDTH;

    fn solve(
        i: usize,
        x: usize,
        y: usize,
        trail_height: u8,
        trail_ends: &mut HashSet<(usize, usize)>,
    ) -> usize {
        if trail_height == b'9' {
            trail_ends.insert((x, y));
            return 1;
        }
        let mut rating = 0;
        if y > 0 && INPUT[i - LINE_WIDTH] == trail_height + 1 {
            rating += solve(i - LINE_WIDTH, x, y - 1, trail_height + 1, trail_ends);
        }
        if x > 0 && INPUT[i - 1] == trail_height + 1 {
            rating += solve(i - 1, x - 1, y, trail_height + 1, trail_ends);
        }
        if y < HEIGHT - 1 && INPUT[i + LINE_WIDTH] == trail_height + 1 {
            rating += solve(i + LINE_WIDTH, x, y + 1, trail_height + 1, trail_ends);
        }
        if x < WIDTH - 1 && INPUT[i + 1] == trail_height + 1 {
            rating += solve(i + 1, x + 1, y, trail_height + 1, trail_ends);
        }
        rating
    }

    let start = std::time::Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut trail_ends = HashSet::with_capacity(7);
    let mut i = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if INPUT[i] == b'0' {
                part2 += solve(i, x, y, b'0', &mut trail_ends);
                part1 += trail_ends.len();
                trail_ends.clear();
            }
            i += 1;
        }
        i += LINE_WIDTH - WIDTH;
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
