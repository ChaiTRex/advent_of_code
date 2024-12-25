fn main() {
    static INPUT: &str = include_str!("../../../day25.txt");

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for entry in INPUT.split("\n\n") {
        let mut lines = entry.lines();
        if lines.next().unwrap() == "#####" {
            let mut heights = [0, 0, 0, 0, 0];
            for (height, line) in (1..=5).zip(lines) {
                for (i, ch) in (0..5).zip(line.bytes()) {
                    if ch == b'#' {
                        heights[i] = height;
                    }
                }
            }
            locks.push(heights);
        } else {
            let mut heights = [0, 0, 0, 0, 0];
            for (height, line) in (1..=5).zip(lines.rev().skip(1)) {
                for (i, ch) in (0..5).zip(line.bytes()) {
                    if ch == b'#' {
                        heights[i] = height;
                    }
                }
            }
            keys.push(heights);
        }
    }

    let mut part1 = 0;
    for lock in &locks {
        for key in &keys {
            if lock
                .into_iter()
                .copied()
                .zip(key.into_iter().copied())
                .map(|(lock_height, key_height)| lock_height + key_height)
                .all(|height| height <= 5)
            {
                part1 += 1;
            }
        }
    }

    println!("{part1}");
}
