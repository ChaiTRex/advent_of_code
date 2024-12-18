fn main() {
    static INPUT: &str = include_str!("../../../day18.txt");

    let start = std::time::Instant::now();

    let drops = INPUT
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<u8>().unwrap();
            let y = y.parse::<u8>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let mut unusable = [[false; 71]; 71];
    for (x, y) in drops.iter().copied().take(1024) {
        unusable[y as usize][x as usize] = true;
    }
    let mut best = [[usize::MAX; 71]; 71];
    let mut global_minimum = usize::MAX;
    let part1 = f(0, (0, 0), &mut global_minimum, &mut best, &unusable).unwrap();

    let mut part2 = String::from("ERROR");
    for (x, y) in drops[1024..].iter().copied() {
        unusable[y as usize][x as usize] = true;
        let mut visited_or_unusable = unusable;
        if !g((0, 0), &mut visited_or_unusable) {
            part2 = format!("{x},{y}");
            break;
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}

fn f(
    mut steps: usize,
    (x, y): (usize, usize),
    global_minimum: &mut usize,
    best: &mut [[usize; 71]; 71],
    unusable: &[[bool; 71]; 71],
) -> Option<usize> {
    if unusable[y][x] {
        return None;
    }
    if steps >= best[y][x] || steps > *global_minimum {
        return None;
    }
    best[y][x] = steps;
    if (x, y) == (70, 70) {
        *global_minimum = steps.min(*global_minimum);
        return Some(steps);
    }
    steps += 1;
    [
        (y > 0).then(|| f(steps, (x, y - 1), global_minimum, best, unusable)),
        (x < 70).then(|| f(steps, (x + 1, y), global_minimum, best, unusable)),
        (y < 70).then(|| f(steps, (x, y + 1), global_minimum, best, unusable)),
        (x > 0).then(|| f(steps, (x - 1, y), global_minimum, best, unusable)),
    ]
    .into_iter()
    .flatten()
    .flatten()
    .min()
}

fn g((x, y): (usize, usize), visited_or_unusable: &mut [[bool; 71]; 71]) -> bool {
    if visited_or_unusable[y][x] {
        return false;
    }
    visited_or_unusable[y][x] = true;
    if (x, y) == (70, 70) {
        return true;
    }

    (x, y) == (70, 70)
        || (x < 70 && g((x + 1, y), visited_or_unusable))
        || (y < 70 && g((x, y + 1), visited_or_unusable))
        || (y > 0 && g((x, y - 1), visited_or_unusable))
        || (x > 0 && g((x - 1, y), visited_or_unusable))
}
