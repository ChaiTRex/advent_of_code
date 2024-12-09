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

    fn f(
        visited: &mut [Vec<bool>],
        x: usize,
        y: usize,
        i: usize,
        north_walls: &mut Vec<(usize, usize)>,
        east_walls: &mut Vec<(usize, usize)>,
        south_walls: &mut Vec<(usize, usize)>,
        west_walls: &mut Vec<(usize, usize)>,
        crop_type: u8,
    ) -> (u64, u64) {
        visited[y][x] = true;

        let mut perimeter = 0;
        let mut area = 1;
        if y > 0 && INPUT[LINE_WIDTH * (y - 1) + x] == crop_type {
            if !visited[y - 1][x] {
                let (up_perimeter, up_area) = f(
                    visited,
                    x,
                    y - 1,
                    i - LINE_WIDTH,
                    north_walls,
                    east_walls,
                    south_walls,
                    west_walls,
                    crop_type,
                );
                perimeter += up_perimeter;
                area += up_area;
            }
        } else {
            perimeter += 1;
            north_walls.push((x, y));
        }
        if x > 0 && INPUT[LINE_WIDTH * y + x - 1] == crop_type {
            if !visited[y][x - 1] {
                let (left_perimeter, left_area) = f(
                    visited,
                    x - 1,
                    y,
                    i - 1,
                    north_walls,
                    east_walls,
                    south_walls,
                    west_walls,
                    crop_type,
                );
                perimeter += left_perimeter;
                area += left_area;
            }
        } else {
            perimeter += 1;
            west_walls.push((x, y));
        }
        if y < HEIGHT - 1 && INPUT[LINE_WIDTH * (y + 1) + x] == crop_type {
            if !visited[y + 1][x] {
                let (down_perimeter, down_area) = f(
                    visited,
                    x,
                    y + 1,
                    i + LINE_WIDTH,
                    north_walls,
                    east_walls,
                    south_walls,
                    west_walls,
                    crop_type,
                );
                perimeter += down_perimeter;
                area += down_area;
            }
        } else {
            perimeter += 1;
            south_walls.push((x, y));
        }
        if x < WIDTH - 1 && INPUT[LINE_WIDTH * y + x + 1] == crop_type {
            if !visited[y][x + 1] {
                let (right_perimeter, right_area) = f(
                    visited,
                    x + 1,
                    y,
                    i + 1,
                    north_walls,
                    east_walls,
                    south_walls,
                    west_walls,
                    crop_type,
                );
                perimeter += right_perimeter;
                area += right_area;
            }
        } else {
            perimeter += 1;
            east_walls.push((x, y));
        }

        (perimeter, area)
    }

    let mut visited = vec![vec![false; WIDTH]; HEIGHT];

    let mut part1 = 0;
    let mut part2 = 0;
    let mut i = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if !visited[y][x] {
                let mut north_walls = Vec::new();
                let mut east_walls = Vec::new();
                let mut south_walls = Vec::new();
                let mut west_walls = Vec::new();

                let (perimeter, area) = f(
                    &mut visited,
                    x,
                    y,
                    i,
                    &mut north_walls,
                    &mut east_walls,
                    &mut south_walls,
                    &mut west_walls,
                    INPUT[i],
                );
                part1 += perimeter * area;

                north_walls.sort();
                east_walls.sort();
                west_walls.sort();
                south_walls.sort();
                let mut side_count = 0;
                while !north_walls.is_empty() {
                    let (x1, y1) = north_walls.remove(0);
                    side_count += 1;
                    let mut x2 = x1 + 1;
                    loop {
                        if let Some(i) = north_walls.iter().position(|&(x, y)| x == x2 && y == y1) {
                            north_walls.remove(i);
                            x2 += 1;
                        } else {
                            break;
                        }
                    }
                }
                while !east_walls.is_empty() {
                    let (x1, y1) = east_walls.remove(0);
                    side_count += 1;
                    let mut y2 = y1 + 1;
                    loop {
                        if let Some(i) = east_walls.iter().position(|&(x, y)| x == x1 && y == y2) {
                            east_walls.remove(i);
                            y2 += 1;
                        } else {
                            break;
                        }
                    }
                }
                while !south_walls.is_empty() {
                    let (x1, y1) = south_walls.remove(0);
                    side_count += 1;
                    let mut x2 = x1 + 1;
                    loop {
                        if let Some(i) = south_walls.iter().position(|&(x, y)| x == x2 && y == y1) {
                            south_walls.remove(i);
                            x2 += 1;
                        } else {
                            break;
                        }
                    }
                }
                while !west_walls.is_empty() {
                    let (x1, y1) = west_walls.remove(0);
                    side_count += 1;
                    let mut y2 = y1 + 1;
                    loop {
                        if let Some(i) = west_walls.iter().position(|&(x, y)| x == x1 && y == y2) {
                            west_walls.remove(i);
                            y2 += 1;
                        } else {
                            break;
                        }
                    }
                }
                part2 += side_count * area;
            }
            i += 1;
        }
        i += const { LINE_WIDTH - WIDTH };
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
