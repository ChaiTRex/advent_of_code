fn main() {
    static INPUT: &str = include_str!("../../../day04.txt");

    let start = std::time::Instant::now();

    let input = INPUT
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();

    let mut part1 = 0;
    for x in 0..width - 3 {
        for y in 0..height {
            if input[y][x] == b'X'
                && input[y][x + 1] == b'M'
                && input[y][x + 2] == b'A'
                && input[y][x + 3] == b'S'
            {
                part1 += 1;
            }
            if input[y][x] == b'S'
                && input[y][x + 1] == b'A'
                && input[y][x + 2] == b'M'
                && input[y][x + 3] == b'X'
            {
                part1 += 1;
            }
        }
    }
    for x in 0..width {
        for y in 0..height - 3 {
            if input[y][x] == b'X'
                && input[y + 1][x] == b'M'
                && input[y + 2][x] == b'A'
                && input[y + 3][x] == b'S'
            {
                part1 += 1;
            }
            if input[y][x] == b'S'
                && input[y + 1][x] == b'A'
                && input[y + 2][x] == b'M'
                && input[y + 3][x] == b'X'
            {
                part1 += 1;
            }
        }
    }
    for x in 0..width - 3 {
        for y in 0..height - 3 {
            if input[y][x] == b'X'
                && input[y + 1][x + 1] == b'M'
                && input[y + 2][x + 2] == b'A'
                && input[y + 3][x + 3] == b'S'
            {
                part1 += 1;
            }
            if input[y][x] == b'S'
                && input[y + 1][x + 1] == b'A'
                && input[y + 2][x + 2] == b'M'
                && input[y + 3][x + 3] == b'X'
            {
                part1 += 1;
            }
        }
    }
    for x in 0..width - 3 {
        for y in 0..height - 3 {
            if input[y][x + 3] == b'X'
                && input[y + 1][x + 2] == b'M'
                && input[y + 2][x + 1] == b'A'
                && input[y + 3][x] == b'S'
            {
                part1 += 1;
            }
            if input[y][x + 3] == b'S'
                && input[y + 1][x + 2] == b'A'
                && input[y + 2][x + 1] == b'M'
                && input[y + 3][x] == b'X'
            {
                part1 += 1;
            }
        }
    }

    let mut part2 = 0;
    for x in 0..width - 2 {
        for y in 0..height - 2 {
            if input[y + 1][x + 1] == b'A' {
                if (input[y][x] == b'M' && input[y + 2][x + 2] == b'S')
                    || (input[y][x] == b'S' && input[y + 2][x + 2] == b'M')
                {
                    if (input[y + 2][x] == b'M' && input[y][x + 2] == b'S')
                        || (input[y + 2][x] == b'S' && input[y][x + 2] == b'M')
                    {
                        part2 += 1;
                    }
                }
            }
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
