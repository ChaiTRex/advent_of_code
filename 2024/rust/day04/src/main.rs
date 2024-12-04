fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day04.txt");
    const WIDTH: usize = {
        let mut i = 0;
        while !INPUT[i].is_ascii_whitespace() {
            i += 1;
        }
        i
    };
    const LINE_WIDTH: usize = {
        let mut i = WIDTH;
        while INPUT[i].is_ascii_whitespace() {
            i += 1;
        }
        i
    };
    const HEIGHT: usize = INPUT.len() / LINE_WIDTH;

    let start = std::time::Instant::now();

    macro_rules! xmas_count_stage {
        ($counter:ident, $input:ident, $line_width:ident, $width:ident, $height:ident :
            { $($x_start_offset:literal, $x_end_offset:literal, $y_end_offset:literal,
            $neighbor_offset:expr;)* }) => {
            $(
                for y in (0..$line_width * ($height - $y_end_offset)).step_by($line_width) {
                    for x in $x_start_offset..$width - $x_end_offset {
                        let i = x + y;

                        const I_1: isize = $neighbor_offset;
                        const I_2: isize = I_1 + $neighbor_offset;
                        const I_3: isize = I_2 + $neighbor_offset;
                        if ($input[i] == b'X'
                            && $input[i.wrapping_add_signed(I_1)] == b'M'
                            && $input[i.wrapping_add_signed(I_2)] == b'A'
                            && $input[i.wrapping_add_signed(I_3)] == b'S')
                            || ($input[i.wrapping_add_signed(I_3)] == b'X'
                                && $input[i.wrapping_add_signed(I_2)] == b'M'
                                && $input[i.wrapping_add_signed(I_1)] == b'A'
                                && $input[i] == b'S')
                        {
                            $counter += 1;
                        }
                    }
                }
            )*
        };
    }

    let mut part1 = 0;
    xmas_count_stage!(part1, INPUT, LINE_WIDTH, WIDTH, HEIGHT: {
        0, 3, 0, 1; // horizontal
        0, 0, 3, LINE_WIDTH as isize; // vertical
        0, 3, 3, LINE_WIDTH as isize + 1; // diagonal \
        3, 0, 3, LINE_WIDTH as isize - 1; // diagonal /
    });

    let mut part2 = 0;
    for y in (LINE_WIDTH..LINE_WIDTH * (HEIGHT - 1)).step_by(LINE_WIDTH) {
        for x in 1..WIDTH - 1 {
            let i = x + y;
            if INPUT[i] == b'A'
                && ((INPUT[i - LINE_WIDTH - 1] == b'M' && INPUT[i + LINE_WIDTH + 1] == b'S')
                    || (INPUT[i + LINE_WIDTH + 1] == b'M' && INPUT[i - LINE_WIDTH - 1] == b'S'))
                && ((INPUT[i - LINE_WIDTH + 1] == b'M' && INPUT[i + LINE_WIDTH - 1] == b'S')
                    || (INPUT[i + LINE_WIDTH - 1] == b'M' && INPUT[i - LINE_WIDTH + 1] == b'S'))
            {
                part2 += 1;
            }
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
