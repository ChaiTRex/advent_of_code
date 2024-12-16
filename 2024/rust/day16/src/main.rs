use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day16.txt");
    static WIDTH: usize = {
        let mut i = 0;
        while INPUT[i].is_ascii_graphic() {
            i += 1;
        }
        i
    };
    static LINE_WIDTH: usize = {
        let mut i = WIDTH;
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i + 1
    };
    static HEIGHT: usize = (INPUT.len() + LINE_WIDTH - WIDTH) / LINE_WIDTH;

    static LOWEST_SCORE_SO_FAR: AtomicU64 = AtomicU64::new(u64::MAX);

    let bench_start = std::time::Instant::now();

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    fn f(i: usize, direction: Direction, score_so_far: u64, least_scores: &mut [Option<u64>]) {
        if INPUT[i] == b'#' || score_so_far > LOWEST_SCORE_SO_FAR.load(Ordering::Relaxed) {
            return;
        }
        if least_scores[(i << 2) | direction as usize]
            .map(|score| score <= score_so_far)
            .unwrap_or(false)
        {
            //println!("fail {i} {direction:?}");
            return;
        }
        least_scores[(i << 2) | direction as usize] = Some(score_so_far);

        match INPUT[i] {
            b'#' => panic!("'#' is already handled above"),
            b'E' => {
                LOWEST_SCORE_SO_FAR.fetch_min(score_so_far, Ordering::Relaxed);
            }
            b'.' | b'S' => match direction {
                Direction::North => {
                    f(i - LINE_WIDTH, direction, score_so_far + 1, least_scores);
                    f(i - 1, Direction::West, score_so_far + 1001, least_scores);
                    f(i + 1, Direction::East, score_so_far + 1001, least_scores);
                }
                Direction::East => {
                    f(i + 1, direction, score_so_far + 1, least_scores);
                    f(
                        i - LINE_WIDTH,
                        Direction::North,
                        score_so_far + 1001,
                        least_scores,
                    );
                    f(
                        i + LINE_WIDTH,
                        Direction::South,
                        score_so_far + 1001,
                        least_scores,
                    );
                }
                Direction::South => {
                    f(i + LINE_WIDTH, direction, score_so_far + 1, least_scores);
                    f(i - 1, Direction::West, score_so_far + 1001, least_scores);
                    f(i + 1, Direction::East, score_so_far + 1001, least_scores);
                }
                Direction::West => {
                    f(i - 1, direction, score_so_far + 1, least_scores);
                    f(
                        i - LINE_WIDTH,
                        Direction::North,
                        score_so_far + 1001,
                        least_scores,
                    );
                    f(
                        i + LINE_WIDTH,
                        Direction::South,
                        score_so_far + 1001,
                        least_scores,
                    );
                }
            },
            _ => panic!("Invalid map character ({})", INPUT[i] as char),
        };
    }

    let start = INPUT.iter().copied().position(|spot| spot == b'S').unwrap();
    let end = INPUT.iter().copied().position(|spot| spot == b'E').unwrap();

    let mut least_scores = vec![None; INPUT.len() << 2];

    f(start, Direction::East, 0, &mut least_scores);
    f(start - 1, Direction::West, 2001, &mut least_scores);
    //println!("{end}\n\n{least_scores:?}");
    let part1 = least_scores[(end << 2)..][..4]
        .iter()
        .copied()
        .flatten()
        .min()
        .unwrap();

    fn g(
        i: usize,
        direction: Direction,
        score_so_far: u64,
        least_scores: &[Option<u64>],
        good_spots: &mut HashSet<usize>,
    ) -> bool {
        if INPUT[i] == b'#'
            || least_scores[(i << 2) | direction as usize]
                .map(|least_score| score_so_far != least_score)
                .unwrap_or(true)
        {
            // println!("Reached ({}, {})", i % LINE_WIDTH, i / LINE_WIDTH);
            return false;
        }
        match INPUT[i] {
            b'.' | b'S' => {
                if match direction {
                    Direction::North => {
                        g(
                            i - LINE_WIDTH,
                            direction,
                            score_so_far + 1,
                            least_scores,
                            good_spots,
                        ) | g(
                            i - 1,
                            Direction::West,
                            score_so_far + 1001,
                            least_scores,
                            good_spots,
                        ) | g(
                            i + 1,
                            Direction::East,
                            score_so_far + 1001,
                            least_scores,
                            good_spots,
                        )
                    }
                    Direction::East => {
                        g(i + 1, direction, score_so_far + 1, least_scores, good_spots)
                            | g(
                                i - LINE_WIDTH,
                                Direction::North,
                                score_so_far + 1001,
                                least_scores,
                                good_spots,
                            )
                            | g(
                                i + LINE_WIDTH,
                                Direction::South,
                                score_so_far + 1001,
                                least_scores,
                                good_spots,
                            )
                    }
                    Direction::South => {
                        g(
                            i + LINE_WIDTH,
                            direction,
                            score_so_far + 1,
                            least_scores,
                            good_spots,
                        ) | g(
                            i - 1,
                            Direction::West,
                            score_so_far + 1001,
                            least_scores,
                            good_spots,
                        ) | g(
                            i + 1,
                            Direction::East,
                            score_so_far + 1001,
                            least_scores,
                            good_spots,
                        )
                    }
                    Direction::West => {
                        g(i - 1, direction, score_so_far + 1, least_scores, good_spots)
                            | g(
                                i - LINE_WIDTH,
                                Direction::North,
                                score_so_far + 1001,
                                least_scores,
                                good_spots,
                            )
                            | g(
                                i + LINE_WIDTH,
                                Direction::South,
                                score_so_far + 1001,
                                least_scores,
                                good_spots,
                            )
                    }
                } {
                    good_spots.insert(i);
                    true
                } else {
                    false
                }
            }
            b'E' => {
                if score_so_far
                    == least_scores[(i << 2)..][..4]
                        .iter()
                        .copied()
                        .flatten()
                        .min()
                        .unwrap()
                {
                    good_spots.insert(i);
                    true
                } else {
                    false
                }
            }
            b'#' => unreachable!("'#' already handled above"),
            _ => panic!("Invalid map character ({})", INPUT[i] as char),
        }
    }

    let mut good_spots = HashSet::new();

    g(start, Direction::East, 0, &least_scores, &mut good_spots);
    g(
        start - 1,
        Direction::West,
        2001,
        &least_scores,
        &mut good_spots,
    );

    let part2 = good_spots.len();

    let time = bench_start.elapsed();

    let mut i = 0;
    for _ in 0..HEIGHT {
        for _ in 0..WIDTH {
            if good_spots.contains(&i) {
                print!("O");
            } else {
                print!("{}", if INPUT[i] == b'.' { b' ' } else { INPUT[i] } as char);
            }
            i += 1;
        }
        println!();
        i += LINE_WIDTH - WIDTH;
    }

    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
