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
    let part1 = f(0, (0, 0), &mut best, &unusable).unwrap();

    let mut part2 = String::from("ERROR");
    for (x, y) in drops[1024..].iter().copied() {
        unusable[y as usize][x as usize] = true;
        let mut best = [[usize::MAX; 71]; 71];
        if f(0, (0, 0), &mut best, &unusable).is_none() {
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
    best: &mut [[usize; 71]; 71],
    unusable: &[[bool; 71]; 71],
) -> Option<usize> {
    /*println!(
        "steps: {} at ({x}, {y}) with best {} and unusability is {}",
        steps + 1,
        best[y][x],
        unusable[y][x]
    );*/
    if unusable[y][x] {
        return None;
    }
    if steps >= best[y][x] {
        return None;
    }
    best[y][x] = steps;
    if (x, y) == (70, 70) {
        return Some(steps);
    }
    steps += 1;
    [
        (y > 0).then(|| f(steps, (x, y - 1), best, unusable)),
        (x < 70).then(|| f(steps, (x + 1, y), best, unusable)),
        (y < 70).then(|| f(steps, (x, y + 1), best, unusable)),
        (x > 0).then(|| f(steps, (x - 1, y), best, unusable)),
    ]
    .into_iter()
    .flatten()
    .flatten()
    .min()
}
