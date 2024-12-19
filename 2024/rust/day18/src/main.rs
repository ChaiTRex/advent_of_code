fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day18.txt");
    const LINEBREAK_WIDTH: usize = {
        let mut i = 0;
        while INPUT[i].is_ascii_graphic() {
            i += 1;
        }
        let start = i;
        while !INPUT[i].is_ascii_graphic() {
            i += 1;
        }
        i - start
    };

    let start = std::time::Instant::now();

    let mut drops = Vec::with_capacity(3450);
    let mut i = 0;
    while i + 1 < INPUT.len() {
        let x;
        if INPUT[i + 1].is_ascii_digit() {
            x = 10 * (INPUT[i] - b'0') + INPUT[i + 1] - b'0';
            i += 3;
        } else {
            x = INPUT[i] - b'0';
            i += 2;
        }
        let y;
        if INPUT[i + 1].is_ascii_digit() {
            y = 10 * (INPUT[i] - b'0') + INPUT[i + 1] - b'0';
            i += 2 + LINEBREAK_WIDTH;
        } else {
            y = INPUT[i] - b'0';
            i += 1 + LINEBREAK_WIDTH;
        }
        drops.push((x, y));
    }

    let mut unusable = [[false; 71]; 71];
    for (x, y) in drops.iter().copied().take(1024) {
        unusable[y as usize][x as usize] = true;
    }
    let mut best = [[u16::MAX; 71]; 71];
    let mut global_minimum = u16::MAX;
    let part1 = f(0, (0, 0), &mut global_minimum, &mut best, &unusable).unwrap();

    let mut part2 = String::new();
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
    mut steps: u16,
    (x, y): (u8, u8),
    global_minimum: &mut u16,
    best: &mut [[u16; 71]; 71],
    unusable: &[[bool; 71]; 71],
) -> Option<u16> {
    if unusable[y as usize][x as usize] {
        return None;
    }
    if steps >= best[y as usize][x as usize] || steps > *global_minimum {
        return None;
    }
    best[y as usize][x as usize] = steps;
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

    (x, y) == (70, 70)
        || (x < 70 && g((x + 1, y), visited_or_unusable))
        || (y < 70 && g((x, y + 1), visited_or_unusable))
        || (y > 0 && g((x, y - 1), visited_or_unusable))
        || (x > 0 && g((x - 1, y), visited_or_unusable))
}
