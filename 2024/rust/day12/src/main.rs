fn main() {
    static INPUT: &str = include_str!("../../../day12.txt");
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();

    let map = INPUT
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; width]; height];

    fn f(
        map: &[Vec<u8>],
        visited: &mut [Vec<bool>],
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        north_walls: &mut Vec<(usize, usize)>,
        east_walls: &mut Vec<(usize, usize)>,
        south_walls: &mut Vec<(usize, usize)>,
        west_walls: &mut Vec<(usize, usize)>,
        crop_type: u8,
    ) -> (u64, u64) {
        visited[y][x] = true;

        let mut perimeter = 0;
        let mut area = 1;
        if y > 0 && map[y - 1][x] == crop_type {
            if !visited[y - 1][x] {
                let (up_perimeter, up_area) = f(
                    map,
                    visited,
                    width,
                    height,
                    x,
                    y - 1,
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
        if x > 0 && map[y][x - 1] == crop_type {
            if !visited[y][x - 1] {
                let (left_perimeter, left_area) = f(
                    map,
                    visited,
                    width,
                    height,
                    x - 1,
                    y,
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
        if y < height - 1 && map[y + 1][x] == crop_type {
            if !visited[y + 1][x] {
                let (down_perimeter, down_area) = f(
                    map,
                    visited,
                    width,
                    height,
                    x,
                    y + 1,
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
        if x < width - 1 && map[y][x + 1] == crop_type {
            if !visited[y][x + 1] {
                let (right_perimeter, right_area) = f(
                    map,
                    visited,
                    width,
                    height,
                    x + 1,
                    y,
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

    let mut part1 = 0;
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let mut north_walls = Vec::new();
                let mut east_walls = Vec::new();
                let mut south_walls = Vec::new();
                let mut west_walls = Vec::new();

                let (perimeter, area) = f(
                    &map,
                    &mut visited,
                    width,
                    height,
                    x,
                    y,
                    &mut north_walls,
                    &mut east_walls,
                    &mut south_walls,
                    &mut west_walls,
                    map[y][x],
                );
                part1 += perimeter * area;
            }
        }
    }
    println!("{part1}");

    let mut visited = vec![vec![false; width]; height];

    let mut part2 = 0;
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let mut north_walls = Vec::new();
                let mut east_walls = Vec::new();
                let mut south_walls = Vec::new();
                let mut west_walls = Vec::new();

                let (_, area) = f(
                    &map,
                    &mut visited,
                    width,
                    height,
                    x,
                    y,
                    &mut north_walls,
                    &mut east_walls,
                    &mut south_walls,
                    &mut west_walls,
                    map[y][x],
                );
                north_walls.sort();
                east_walls.sort();
                west_walls.sort();
                south_walls.sort();
                let mut side_count = 0;
                while !north_walls.is_empty() {
                    let (x1, y1) = north_walls.remove(0);
                    println!("North wall starting at {:?}", (x1, y1));
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
                    println!("{side_count}");
                }
                while !east_walls.is_empty() {
                    let (x1, y1) = east_walls.remove(0);
                    println!("East wall starting at {:?}", (x1, y1));
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
                    println!("{side_count}");
                }
                while !south_walls.is_empty() {
                    let (x1, y1) = south_walls.remove(0);
                    println!("South wall starting at {:?}", (x1, y1));
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
                    println!("{side_count}");
                }
                while !west_walls.is_empty() {
                    let (x1, y1) = west_walls.remove(0);
                    println!("West wall starting at {:?}", (x1, y1));
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
                    println!("{side_count}");
                }
                dbg!(map[y][x] as char, side_count, area);
                part2 += side_count * area;
            }
        }
    }
    println!("{part2}");
}
