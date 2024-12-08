use std::collections::{HashMap, HashSet};

fn main() {
    static INPUT: &str = include_str!("../../../day08.txt");

    let width = INPUT.lines().next().unwrap().len();
    let height = 50; // INPUT.len() / width;
    let mut antennas = HashMap::new();
    for (y, row) in INPUT.lines().enumerate() {
        for (x, antenna) in row.bytes().enumerate() {
            if antenna != b'.' {
                antennas
                    .entry(antenna as char)
                    .or_insert(Vec::new())
                    .push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (kind, nodes) in antennas.iter() {
        for i in 0..nodes.len() - 1 {
            let a = nodes[i];
            for j in i + 1..nodes.len() {
                let b = nodes[j];

                if (0..width as i32).contains(&(2 * a.0 - b.0))
                    && (0..height as i32).contains(&(2 * a.1 - b.1))
                {
                    antinodes.insert((2 * a.0 - b.0, 2 * a.1 - b.1));
                }
                if (0..width as i32).contains(&(2 * b.0 - a.0))
                    && (0..height as i32).contains(&(2 * b.1 - a.1))
                {
                    antinodes.insert((2 * b.0 - a.0, 2 * b.1 - a.1));
                }
            }
        }
    }
    let part1 = antinodes.len();

    println!("{part1}");

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if b == 0 {
                return a;
            }

            a = a % b;
            core::mem::swap(&mut a, &mut b);
        }
    }

    antinodes.clear();
    for (kind, nodes) in antennas.iter() {
        for i in 0..nodes.len() - 1 {
            let a = nodes[i];
            for j in i + 1..nodes.len() {
                let b = nodes[j];

                let x_diff = a.0 - b.0;
                let y_diff = a.1 - b.1;
                let g: i32 = gcd(x_diff.abs(), y_diff.abs());
                let x_diff = x_diff / g;
                let y_diff = y_diff / g;

                let mut x_1 = a.0;
                let mut y_1 = a.1;
                while (0..width as i32).contains(&x_1) && (0..height as i32).contains(&y_1) {
                    x_1 -= x_diff;
                    y_1 -= y_diff;
                }
                x_1 += x_diff;
                y_1 += y_diff;

                while (0..width as i32).contains(&x_1) && (0..height as i32).contains(&y_1) {
                    antinodes.insert((x_1 as i32, y_1 as i32));

                    x_1 += x_diff;
                    y_1 += y_diff;
                }
            }
        }
    }
    let part2 = antinodes.len();

    println!("{part2}");
}
