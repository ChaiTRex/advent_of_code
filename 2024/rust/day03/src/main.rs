fn main() {
    static INPUT: &str = include_str!("../../../day03.txt");

    let start = std::time::Instant::now();

    macro_rules! parse {
        ($iter:ident: $($ch:literal => $block:tt),*) => {
            match $iter.peek().copied() {
                $(
                    Some($ch) => {
                        $iter.next();
                        $block
                    }
                ),*
                Some(_) => {
                    continue;
                }
                None => break,
            }
        };
    }

    macro_rules! parse_number {
        ($outer:lifetime $iter:ident $result:ident: $terminator:expr => $block:tt) => {
            match $iter.peek().copied() {
                Some(ch @ b'0'..=b'9') => {
                    $iter.next();
                    let mut $result = (ch - b'0') as u32;
                    loop {
                        match $iter.peek().copied() {
                            Some(ch @ b'0'..=b'9') => {
                                $iter.next();
                                $result = 10 * $result + (ch - b'0') as u32;
                            }
                            Some($terminator) => {
                                $iter.next();
                                break;
                            }
                            Some(_) => continue $outer,
                            None => break $outer,
                        }
                    }
                    $block
                }
                Some(_) => continue $outer,
                None => break $outer,
            }
        };
    }

    let mut part1 = 0;
    let mut part2 = 0;
    let mut on = true;
    let mut iter = INPUT.bytes().peekable();

    'outer: loop {
        match iter.next() {
            Some(b'd') => {
                parse!(iter: b'o' => {
                    parse!(iter:
                        b'(' => {
                            parse!(iter: b')' => {
                                on = true;
                            })
                        },
                        b'n' => {
                            parse!(iter: b'\'' => {
                                parse!(iter: b't' => {
                                    parse!(iter: b'(' => {
                                        parse!(iter: b')' =>  {
                                            on = false;
                                        })
                                    })
                                })
                            })
                        }
                    )
                })
            }
            Some(b'm') => {
                parse!(iter: b'u' => {
                    parse!(iter: b'l' => {
                        parse!(iter: b'(' => {
                            parse_number!('outer iter a: b',' => {
                                parse_number!('outer iter b: b')' => {
                                    let product = a * b;
                                    part1 += product;
                                    if on {
                                        part2 += product;
                                    }
                                    continue 'outer;
                                })
                            })
                        })
                    })
                })
            }
            Some(_) => continue,
            None => break,
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
