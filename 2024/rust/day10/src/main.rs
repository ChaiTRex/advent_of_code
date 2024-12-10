use std::collections::HashSet;
fn main() {
    static INPUT: &str = include_str!("../../../day10.txt");

    fn trailhead_score(
        map: &[Vec<u8>],
        height: usize,
        width: usize,
        (x, y): (usize, usize),
        trail_height: u8,
        trail_ends: &mut HashSet<(usize, usize)>,
    ) {
        if map[y][x] == 9 {
            trail_ends.insert((x, y));
            return;
        }
        if y > 0 && map[y - 1][x] == trail_height + 1 {
            trailhead_score(map, height, width, (x, y - 1), trail_height + 1, trail_ends);
        }
        if x > 0 && map[y][x - 1] == trail_height + 1 {
            trailhead_score(map, height, width, (x - 1, y), trail_height + 1, trail_ends);
        }
        if y < height - 1 && map[y + 1][x] == trail_height + 1 {
            trailhead_score(map, height, width, (x, y + 1), trail_height + 1, trail_ends);
        }
        if x < width - 1 && map[y][x + 1] == trail_height + 1 {
            trailhead_score(map, height, width, (x + 1, y), trail_height + 1, trail_ends);
        }
    }

    let map = INPUT
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().copied().enumerate() {
            if height == 0 {
                let mut exits = HashSet::new();
                trailhead_score(&map, map.len(), row.len(), (x, y), 0, &mut exits);
                part1 += exits.len();
            }
        }
    }

    fn trailhead_rating(
        map: &[Vec<u8>],
        height: usize,
        width: usize,
        (x, y): (usize, usize),
        trail_height: u8,
    ) -> usize {
        if map[y][x] == 9 {
            return 1;
        }
        let mut rating = 0;
        if y > 0 && map[y - 1][x] == trail_height + 1 {
            rating += trailhead_rating(map, height, width, (x, y - 1), trail_height + 1);
        }
        if x > 0 && map[y][x - 1] == trail_height + 1 {
            rating += trailhead_rating(map, height, width, (x - 1, y), trail_height + 1);
        }
        if y < height - 1 && map[y + 1][x] == trail_height + 1 {
            rating += trailhead_rating(map, height, width, (x, y + 1), trail_height + 1);
        }
        if x < width - 1 && map[y][x + 1] == trail_height + 1 {
            rating += trailhead_rating(map, height, width, (x + 1, y), trail_height + 1);
        }
        rating
    }

    let mut part2 = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().copied().enumerate() {
            if height == 0 {
                part2 += trailhead_rating(&map, map.len(), row.len(), (x, y), 0);
            }
        }
    }

    println!("{part1}");
    println!("{part2}");
}
