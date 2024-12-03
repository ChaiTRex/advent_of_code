fn main() {
    static INPUT: &str = include_str!("../../../day03.txt");

    let start = std::time::Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut on = true;
    let mut iter = INPUT.bytes();

    'outer: loop {
        match iter.next() {
            None => break,
            Some(b'd') => match iter.next() {
                None => break,
                Some(b'o') => match iter.next() {
                    None => break,
                    Some(b'(') => match iter.next() {
                        None => break,
                        Some(b')') => on = true,
                        _ => (),
                    },
                    Some(b'n') => match iter.next() {
                        None => break,
                        Some(b'\'') => match iter.next() {
                            None => break,
                            Some(b't') => match iter.next() {
                                None => break,
                                Some(b'(') => match iter.next() {
                                    None => break,
                                    Some(b')') => on = false,
                                    _ => (),
                                },
                                _ => (),
                            },
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            Some(b'm') => match iter.next() {
                None => break,
                Some(b'u') => match iter.next() {
                    None => break,
                    Some(b'l') => match iter.next() {
                        None => break,
                        Some(b'(') => match iter.next() {
                            None => break,
                            Some(ch @ b'0'..=b'9') => {
                                let mut a = (ch - b'0') as u32;
                                loop {
                                    match iter.next() {
                                        None => break 'outer,
                                        Some(ch @ b'0'..=b'9') => {
                                            a = 10 * a + (ch - b'0') as u32;
                                        }
                                        Some(b',') => break,
                                        _ => continue 'outer,
                                    }
                                }
                                match iter.next() {
                                    None => break,
                                    Some(ch @ b'0'..=b'9') => {
                                        let mut b = (ch - b'0') as u32;
                                        loop {
                                            match iter.next() {
                                                None => break 'outer,
                                                Some(ch @ b'0'..=b'9') => {
                                                    b = 10 * b + (ch - b'0') as u32;
                                                }
                                                Some(b')') => {
                                                    let product = a * b;
                                                    part1 += product;
                                                    if on {
                                                        part2 += product;
                                                    }
                                                    break;
                                                }
                                                _ => continue 'outer,
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
