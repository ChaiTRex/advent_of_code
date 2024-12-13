fn main() {
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

    let start = std::time::Instant::now();

    let mut corner_counts = vec![0_u8; LINE_WIDTH * HEIGHT];
    let mut perimeters = vec![0_u8; LINE_WIDTH * HEIGHT];
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
                    $input:ident,
                    $corner_counts:ident,
                    $i:ident :
                    $({
                        $is_fence_a:ident,
                        $is_fence_b:ident,
                        $cond_1:expr,
                        $cond_2:expr,
                        $i_corner:expr,
                        $i_1:expr,
                        $i_2:expr
                    };)+
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

    fn f(
        visited: &mut [bool],
        perimeters: &[u8],
        corner_counts: &[u8],
        x: usize,
        y: usize,
        i: usize,
        crop_type: u8,
    ) -> (u64, u64, u64) {
        visited[i] = true;

        let mut corner_count = corner_counts[i] as u64;
        let mut perimeter = perimeters[i] as u64;
        let mut area = 1;
        let up_i = i - LINE_WIDTH;
        if y > 0 && INPUT[up_i] == crop_type && !visited[up_i] {
            let (up_perimeter, up_corner_count, up_area) = f(
                visited,
                perimeters,
                corner_counts,
                x,
                y - 1,
                up_i,
                crop_type,
            );
            perimeter += up_perimeter;
            corner_count += up_corner_count;
            area += up_area;
        }
        let left_i = i - 1;
        if x > 0 && INPUT[left_i] == crop_type && !visited[left_i] {
            let (left_perimeter, left_corner_count, left_area) = f(
                visited,
                perimeters,
                corner_counts,
                x - 1,
                y,
                left_i,
                crop_type,
            );
            perimeter += left_perimeter;
            corner_count += left_corner_count;
            area += left_area;
        }
        let right_i = i + 1;
        if x < WIDTH - 1 && INPUT[right_i] == crop_type && !visited[right_i] {
            let (right_perimeter, right_corner_count, right_area) = f(
                visited,
                perimeters,
                corner_counts,
                x + 1,
                y,
                right_i,
                crop_type,
            );
            perimeter += right_perimeter;
            corner_count += right_corner_count;
            area += right_area;
        }
        let down_i = i + LINE_WIDTH;
        if y < HEIGHT - 1 && INPUT[down_i] == crop_type && !visited[down_i] {
            let (down_perimeter, down_corner_count, down_area) = f(
                visited,
                perimeters,
                corner_counts,
                x,
                y + 1,
                down_i,
                crop_type,
            );
            perimeter += down_perimeter;
            corner_count += down_corner_count;
            area += down_area;
        }

        (perimeter, corner_count, area)
    }

    let mut visited = vec![false; LINE_WIDTH * HEIGHT];

    let mut part1 = 0;
    let mut part2 = 0;
    let mut i = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if !visited[i] {
                let (perimeter, corner_count, area) =
                    f(&mut visited, &perimeters, &corner_counts, x, y, i, INPUT[i]);
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
