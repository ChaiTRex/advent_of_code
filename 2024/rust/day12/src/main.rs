fn main() {
    let start = std::time::Instant::now();

    let mut unvisited = vec![true; FARM_SIZE];
    let mut perimeters = vec![0_u8; FARM_SIZE];
    let mut corner_counts = vec![0_u8; FARM_SIZE];
    let mut i = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let up_is_fence = y == 0 || INPUT[i] != INPUT[i - LINE_WIDTH];
            let left_is_fence = x == 0 || INPUT[i] != INPUT[i - 1];
            let right_is_fence = x == WIDTH - 1 || INPUT[i] != INPUT[i + 1];
            let down_is_fence = y == HEIGHT - 1 || INPUT[i] != INPUT[i + LINE_WIDTH];

            perimeters[i] = up_is_fence as u8
                + left_is_fence as u8
                + right_is_fence as u8
                + down_is_fence as u8;

            macro_rules! outward_corners {
                (
                    $input:ident, $corner_counts:ident, $i:ident :
                    $({ $is_fence_a:ident, $is_fence_b:ident, $cond_1:expr, $cond_2:expr, $i_corner:expr, $i_1:expr, $i_2:expr };)+
                ) => {
                    $(
                        if $is_fence_a & $is_fence_b {
                            $corner_counts[$i] += 1;
                            if $cond_1 && $cond_2 {
                                let other_crop = $input[$i_1];
                                if other_crop == $input[$i_corner] && other_crop == $input[$i_2] {
                                    $corner_counts[$i_corner] += 1;
                                }
                            }
                        }
                    )*
                };
            }

            outward_corners!(
                INPUT, corner_counts, i:
                { up_is_fence, left_is_fence, x != 0, y != 0, i - LINE_WIDTH - 1, i - LINE_WIDTH, i - 1 };
                { up_is_fence, right_is_fence, x != WIDTH - 1, y != 0, i - LINE_WIDTH + 1, i - LINE_WIDTH, i + 1 };
                { left_is_fence, down_is_fence, x != 0, y != HEIGHT - 1, i + LINE_WIDTH - 1, i + LINE_WIDTH, i - 1 };
                { right_is_fence, down_is_fence, x != WIDTH - 1, y != HEIGHT - 1, i + LINE_WIDTH + 1, i + LINE_WIDTH, i + 1 };
            );

            i += 1;
        }
        i += LINE_WIDTH - WIDTH;
    }

    let mut part1 = 0;
    let mut part2 = 0;
    let mut i = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if unvisited[i] {
                let (perimeter, corner_count, area) =
                    visit_region(&mut unvisited, &perimeters, &corner_counts, x, y, i);
                part1 += perimeter * area;
                part2 += corner_count * area;
            }
            i += 1;
        }
        i += LINE_WIDTH - WIDTH;
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}

fn visit_region(
    unvisited: &mut [bool],
    perimeters: &[u8],
    corner_counts: &[u8],
    x: usize,
    y: usize,
    i: usize,
) -> (u64, u64, u64) {
    unvisited[i] = false;

    let mut corner_count = corner_counts[i] as u64;
    let mut perimeter = perimeters[i] as u64;
    let mut area = 1;

    macro_rules! visit {
        (
            $input:ident, $unvisited:ident, $perimeters:ident, $corner_counts:ident, $perimeter:ident, $corner_count:ident, $area:ident :
            $({ $cond:expr, $i:ident, $i_after_move:expr, $x_after_move:expr, $y_after_move:expr };)+
        ) => {
            $(
                let i_after_move = $i_after_move;
                if $cond && $input[$i] == $input[i_after_move] && $unvisited[i_after_move] {
                    let (added_perimeter, added_corner_count, added_area) = visit_region($unvisited, $perimeters, $corner_counts, $x_after_move, $y_after_move, i_after_move);
                    $perimeter += added_perimeter;
                    $corner_count += added_corner_count;
                    $area += added_area;
                }
            )*
        };
    }

    visit!(
        INPUT, unvisited, perimeters, corner_counts, perimeter, corner_count, area :
        { y > 0, i, i - LINE_WIDTH, x, y - 1 };
        { x > 0, i, i - 1, x - 1, y };
        { x < WIDTH - 1, i, i + 1, x + 1, y };
        { y < HEIGHT - 1, i, i + LINE_WIDTH, x, y + 1 };
    );

    (perimeter, corner_count, area)
}

static INPUT: &[u8] = include_bytes!("../../../day12.txt");
const WIDTH: usize = {
    let mut i = 0;
    while INPUT[i].is_ascii_uppercase() {
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
const FARM_SIZE: usize = LINE_WIDTH * (HEIGHT - 1) + WIDTH;
