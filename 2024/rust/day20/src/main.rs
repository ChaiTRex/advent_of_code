fn main() {
    let start_position = INPUT.iter().copied().position(|spot| spot == b'S').unwrap();
    let mut times = vec![usize::MAX; INPUT.len()];

    let mut i = start_position;
    let mut time = 0;
    loop {
        times[i] = time;
        time += 1;
        if INPUT[i] == b'E' {
            break;
        } else if INPUT[i - LINE_WIDTH] != b'#' && times[i - LINE_WIDTH] > time {
            i -= LINE_WIDTH;
        } else if INPUT[i - 1] != b'#' && times[i - 1] > time {
            i -= 1;
        } else if INPUT[i + 1] != b'#' && times[i + 1] > time {
            i += 1;
        } else if INPUT[i + LINE_WIDTH] != b'#' && times[i + LINE_WIDTH] > time {
            i += LINE_WIDTH;
        } else {
            panic!(
                "{i} {:?}: {} with time {}",
                (i / LINE_WIDTH, i % LINE_WIDTH),
                INPUT[i] as char,
                times[i]
            );
        }
    }

    let mut part1 = 0;
    for i in LINE_WIDTH + 1..LINE_WIDTH * (HEIGHT - 1) + WIDTH {
        let time_1 = times[i];
        if time_1 != usize::MAX {
            if i > 2 * LINE_WIDTH {
                let time_2 = times[i - 2 * LINE_WIDTH];
                if time_2 != usize::MAX {
                    let time_diff = time_1.abs_diff(time_2);
                    if time_diff >= 102 {
                        part1 += 1;
                    }
                }
            }
            let time_2 = times[i - LINE_WIDTH + 1];
            if time_2 != usize::MAX {
                let time_diff = time_1.abs_diff(time_2);
                if time_diff >= 102 {
                    part1 += 1;
                }
            }
            let time_2 = times[i + 2];
            if time_2 != usize::MAX {
                let time_diff = time_1.abs_diff(time_2);
                if time_diff >= 102 {
                    part1 += 1;
                }
            }
            let time_2 = times[i + LINE_WIDTH - 1];
            if time_2 != usize::MAX {
                let time_diff = time_1.abs_diff(time_2);
                if time_diff >= 102 {
                    part1 += 1;
                }
            }
        }
    }
    println!("{part1}");

    let mut part2 = 0;
    for i in 0..times.len() {
        let time_1 = times[i];
        if time_1 != usize::MAX {
            let (x1, y1) = (i % LINE_WIDTH, i / LINE_WIDTH);
            for j in i..times.len() {
                let time_2 = times[j];
                if time_2 != usize::MAX {
                    let (x2, y2) = (j % LINE_WIDTH, j / LINE_WIDTH);
                    let dist = x1.abs_diff(x2) + y1.abs_diff(y2);
                    /*println!(
                        "time_1: {time_1}, time_2: {time_2}, ({x1}, {y1}) to ({x2}, {y2}) (distance of {dist})"
                    );*/
                    if dist <= 20 {
                        if time_2.abs_diff(time_1) >= 100 + dist {
                            part2 += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{part2}");
}

static INPUT: &[u8] = include_bytes!("../../../day20.txt");

const WIDTH: usize = {
    let mut i = 0;
    while INPUT[i].is_ascii_graphic() {
        i += 1;
    }
    i
};
const LINE_WIDTH: usize = {
    let mut i = WIDTH;
    while !INPUT[i].is_ascii_graphic() {
        i += 1;
    }
    i
};
const HEIGHT: usize = INPUT.len() / LINE_WIDTH;
