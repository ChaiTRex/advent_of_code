fn main() {
    static INPUT: &str = include_str!("../../../day06.txt");

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Spot {
        Empty,
        Visited {
            earliest_visit: u16,
            directions_when_visiting: u8,
        },
        Blockage,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    let mut guard = (1000, 1000);

    let mut map = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            match line.bytes().position(|x| x == b'^') {
                Some(x) => guard = (x, y),
                None => (),
            }

            line.bytes()
                .map(|b| {
                    if b == b'#' {
                        Spot::Blockage
                    } else {
                        Spot::Empty
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = map[0].len();
    let height = map.len();

    fn follow_map(mut map: Vec<Vec<Spot>>, mut guard: (usize, usize)) -> Option<u16> {
        let mut direction = Direction::North;

        let width = map[0].len();
        let height = map.len();

        let mut time = 0;
        while (0..width).contains(&guard.0) && (0..height).contains(&guard.1) {
            match direction {
                Direction::North => {
                    if guard.1 == 0 {
                        break;
                    }
                    if map[guard.1 - 1][guard.0] == Spot::Blockage {
                        direction = Direction::East;
                        continue;
                    }
                    guard.1 -= 1;
                    map[guard.1][guard.0] = match map[guard.1][guard.0] {
                        Spot::Visited {
                            earliest_visit,
                            directions_when_visiting,
                        } => {
                            if directions_when_visiting & 0b1000 != 0 {
                                return None;
                            }
                            let directions_when_visiting = directions_when_visiting | 0b1000;
                            Spot::Visited {
                                earliest_visit,
                                directions_when_visiting,
                            }
                        }
                        _ => Spot::Visited {
                            earliest_visit: time,
                            directions_when_visiting: 0b1000,
                        },
                    };
                }
                Direction::East => {
                    if guard.0 == width - 1 {
                        break;
                    }
                    if map[guard.1][guard.0 + 1] == Spot::Blockage {
                        direction = Direction::South;
                        continue;
                    }
                    guard.0 += 1;
                    map[guard.1][guard.0] = match map[guard.1][guard.0] {
                        Spot::Visited {
                            earliest_visit,
                            directions_when_visiting,
                        } => {
                            if directions_when_visiting & 0b0100 != 0 {
                                return None;
                            }
                            let directions_when_visiting = directions_when_visiting | 0b0100;
                            Spot::Visited {
                                earliest_visit,
                                directions_when_visiting,
                            }
                        }
                        _ => Spot::Visited {
                            earliest_visit: time,
                            directions_when_visiting: 0b0100,
                        },
                    };
                }
                Direction::South => {
                    if guard.1 == height - 1 {
                        break;
                    }
                    if map[guard.1 + 1][guard.0] == Spot::Blockage {
                        direction = Direction::West;
                        continue;
                    }
                    guard.1 += 1;
                    map[guard.1][guard.0] = match map[guard.1][guard.0] {
                        Spot::Visited {
                            earliest_visit,
                            directions_when_visiting,
                        } => {
                            if directions_when_visiting & 0b0010 != 0 {
                                return None;
                            }
                            let directions_when_visiting = directions_when_visiting | 0b0010;
                            Spot::Visited {
                                earliest_visit,
                                directions_when_visiting,
                            }
                        }
                        _ => Spot::Visited {
                            earliest_visit: time,
                            directions_when_visiting: 0b0010,
                        },
                    };
                }
                Direction::West => {
                    if guard.0 == 0 {
                        break;
                    }
                    if map[guard.1][guard.0 - 1] == Spot::Blockage {
                        direction = Direction::North;
                        continue;
                    }
                    guard.0 -= 1;
                    map[guard.1][guard.0] = match map[guard.1][guard.0] {
                        Spot::Visited {
                            earliest_visit,
                            directions_when_visiting,
                        } => {
                            if directions_when_visiting & 0b0001 != 0 {
                                return None;
                            }
                            let directions_when_visiting = directions_when_visiting | 0b0001;
                            Spot::Visited {
                                earliest_visit,
                                directions_when_visiting,
                            }
                        }
                        _ => Spot::Visited {
                            earliest_visit: time,
                            directions_when_visiting: 0b0001,
                        },
                    };
                }
            }
            time += 1;
        }
        let steps_taken = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&spot| matches!(spot, Spot::Visited { .. }) as u16)
                    .sum::<u16>()
            })
            .sum::<u16>();
        Some(steps_taken)
    }

    let part1 = follow_map(map.clone(), guard).unwrap();
    println!("{part1:?}");

    let mut part2 = 0;
    for y in 0..height {
        for x in 0..width {
            if guard == (x, y) || map[y][x] == Spot::Blockage {
                continue;
            }
            let mut map = map.clone();
            map[y][x] = Spot::Blockage;
            part2 += follow_map(map, guard).is_none() as u16;
        }
    }

    println!("{part2}");
}
