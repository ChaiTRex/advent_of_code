use std::collections::HashSet;

/// Gives the greatest common denominator of the two inputs, unless that's 2⁷.
/// 2⁷ doesn't fit in an `i8`, so it returns -2⁷, which does.
pub fn gcd(u: i8, v: i8) -> i8 {
    let mut v = v.unsigned_abs();
    if u == 0 {
        return v as i8;
    }
    let mut u = u.unsigned_abs();
    if v == 0 {
        return u as i8;
    }

    // `|` is bitwise OR. `trailing_zeros` quickly counts a binary number's
    // trailing zeros, giving its prime factorization's exponent on two.
    let gcd_exponent_on_two = (u | v).trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in
    // the left variable. `u` divided by its prime factorization's power of two
    // turns it odd.
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right.
    (u << gcd_exponent_on_two) as i8
}

fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day08.txt");
    const WIDTH: i8 = {
        let mut i = 0;

        while INPUT[i] != b'\n' {
            i += 1;
        }

        i as i8
    };
    const LINE_WIDTH: i8 = WIDTH + 1;
    const HEIGHT: i8 = (INPUT.len() / LINE_WIDTH as usize) as i8;

    let start = std::time::Instant::now();

    const ORIGINAL_ANTENNAS_LEN: usize = (b'z' - b'0' + 1) as usize;
    let mut antennas = vec![Vec::new(); ORIGINAL_ANTENNAS_LEN];

    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    while y < HEIGHT {
        while x < WIDTH {
            let value = INPUT[i];
            if value != b'.' {
                antennas[(value - b'0') as usize].push((x, y));
            }
            i += 1;
            x += 1;
        }
        i += const { (LINE_WIDTH - WIDTH) as usize };
        x = 0;
        y += 1;
    }
    antennas.retain(|nodes| !nodes.is_empty());

    let mut antinodes = HashSet::new();
    for nodes in &antennas {
        for i in 0..nodes.len() - 1 {
            let a = nodes[i];
            for j in i + 1..nodes.len() {
                let b = nodes[j];

                let x = 2 * a.0 - b.0;
                let y = 2 * a.1 - b.1;
                if (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y) {
                    antinodes.insert((x, y));
                }
                let x = 2 * b.0 - a.0;
                let y = 2 * b.1 - a.1;
                if (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y) {
                    antinodes.insert((x, y));
                }
            }
        }
    }
    let part1 = antinodes.len();

    antinodes.clear();
    for nodes in &antennas {
        for i in 0..nodes.len() - 1 {
            let a = nodes[i];
            for j in i + 1..nodes.len() {
                let b = nodes[j];

                let x_diff = a.0 - b.0;
                let y_diff = a.1 - b.1;
                let g: i8 = gcd(x_diff, y_diff);
                let x_diff = x_diff / g;
                let y_diff = y_diff / g;

                let mut x = a.0;
                let mut y = a.1;
                while (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y) {
                    antinodes.insert((x, y));
                    x -= x_diff;
                    y -= y_diff;
                }

                x = a.0 + x_diff;
                y = a.1 + y_diff;
                while (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y) {
                    antinodes.insert((x, y));
                    x += x_diff;
                    y += y_diff;
                }
            }
        }
    }
    let part2 = antinodes.len();

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
