fn main() {
    static INPUT: &str = include_str!("../../../day04.txt");

    let start = std::time::Instant::now();

    macro_rules! xmas_count_stage {
        (
            $input:ident,
            $width:ident,
            $height:ident,
            $counter:ident,
            $width_dec:literal,
            $height_dec:literal,
            $x_start:literal,
            $x_step:literal,
            $y_start:literal,
            $y_step:literal
        ) => {
            for y in 0..$height - $height_dec {
                for x in 0..$width - $width_dec {
                    const X_0: usize = $x_start;
                    const X_1: usize = X_0.wrapping_add_signed($x_step);
                    const X_2: usize = X_1.wrapping_add_signed($x_step);
                    const X_3: usize = X_2.wrapping_add_signed($x_step);
                    const Y_0: usize = $y_start;
                    const Y_1: usize = Y_0.wrapping_add_signed($y_step);
                    const Y_2: usize = Y_1.wrapping_add_signed($y_step);
                    const Y_3: usize = Y_2.wrapping_add_signed($y_step);

                    if ($input[y + Y_0][x + X_0] == b'X'
                        && $input[y + Y_1][x + X_1] == b'M'
                        && $input[y + Y_2][x + X_2] == b'A'
                        && $input[y + Y_3][x + X_3] == b'S')
                        || ($input[y + Y_0][x + X_0] == b'S'
                            && $input[y + Y_1][x + X_1] == b'A'
                            && $input[y + Y_2][x + X_2] == b'M'
                            && $input[y + Y_3][x + X_3] == b'X')
                    {
                        $counter += 1;
                    }
                }
            }
        };
    }

    let mut input = INPUT
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();

    let mut part1 = 0;
    xmas_count_stage!(input, width, height, part1, 3, 0, 0, 1, 0, 0);
    xmas_count_stage!(input, width, height, part1, 0, 3, 0, 0, 0, 1);
    xmas_count_stage!(input, width, height, part1, 3, 3, 0, 1, 0, 1);
    xmas_count_stage!(input, width, height, part1, 3, 3, 3, -1, 0, 1);

    let mut part2 = 0;
    for x in 0..width - 2 {
        for y in 0..height - 2 {
            if input[y + 1][x + 1] == b'A' {
                if ((input[y][x] == b'M' && input[y + 2][x + 2] == b'S')
                    || (input[y][x] == b'S' && input[y + 2][x + 2] == b'M'))
                    && ((input[y + 2][x] == b'M' && input[y][x + 2] == b'S')
                        || (input[y + 2][x] == b'S' && input[y][x + 2] == b'M'))
                {
                    part2 += 1;
                }
            }
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
