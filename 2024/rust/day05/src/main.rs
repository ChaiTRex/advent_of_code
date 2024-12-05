use std::collections::HashSet;

macro_rules! parse_two_digits {
    ($tens:expr, $ones:expr) => {
        10 * ($tens - b'0') + $ones - b'0'
    };
}

fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day05.txt");

    let start = std::time::Instant::now();

    let mut violations = HashSet::with_capacity(1176);
    let mut i = 0;
    while INPUT[i] != b'\n' {
        violations.insert((
            parse_two_digits!(INPUT[i + 3], INPUT[i + 4]),
            parse_two_digits!(INPUT[i], INPUT[i + 1]),
        ));
        i += 6;
    }
    i += 1;

    let mut part1 = 0;
    let mut part2 = 0;

    let mut pages = Vec::with_capacity(23);
    let mut sorted_pages = Vec::with_capacity(23);
    while i < INPUT.len() {
        loop {
            pages.push(parse_two_digits!(INPUT[i], INPUT[i + 1]));
            i += 3;
            if INPUT[i - 1] == b'\n' {
                break;
            }
        }
        if pages.is_empty() {
            break;
        }
        for page in pages.iter().copied() {
            sorted_pages.push(page);
        }
        sorted_pages.sort_unstable_by(|&a, &b| {
            if a == b {
                core::cmp::Ordering::Equal
            } else if violations.contains(&(a, b)) {
                core::cmp::Ordering::Greater
            } else {
                core::cmp::Ordering::Less
            }
        });

        let middle_element = sorted_pages[sorted_pages.len() / 2] as u16;
        if pages == sorted_pages {
            part1 += middle_element;
        } else {
            part2 += middle_element;
        }

        pages.clear();
        sorted_pages.clear();
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
