use std::collections::HashSet;

fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day15.txt");
    const MAP_WIDTH: usize = {
        let mut i = 0;
        while INPUT[i].is_ascii_graphic() {
            i += 1;
        }
        i
    };
    const MAP_LINE_WIDTH: usize = {
        let mut i = MAP_WIDTH;
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i + 1
    };
    const MAP_HEIGHT: usize = {
        let mut i = MAP_LINE_WIDTH;
        while INPUT[i].is_ascii_graphic() {
            i += MAP_LINE_WIDTH;
        }
        i / MAP_LINE_WIDTH
    };
    const START_OF_MOVEMENTS: usize = MAP_HEIGHT * MAP_LINE_WIDTH + 1;

    let mut map = INPUT[..MAP_LINE_WIDTH * (MAP_HEIGHT - 1) + MAP_WIDTH].to_vec();
    let mut robot_position = map.iter().position(|&spot| spot == b'@').unwrap();
    let mut move_i = START_OF_MOVEMENTS;
    while move_i < INPUT.len() {
        /*println!(
            "\nStep {} ({})\n{}",
            move_i - START_OF_MOVEMENTS,
            INPUT[move_i] as char,
            map.iter().map(|&ch| ch as char).collect::<String>(),
        );
        if !map[MAP_LINE_WIDTH * (MAP_HEIGHT - 1)..][..MAP_WIDTH]
            .iter()
            .all(|&x| x == b'#')
        {
            println!("broken");
            return;
        }*/
        match INPUT[move_i] {
            b'^' => {
                let mut box_count = 0;
                let mut i = robot_position - MAP_LINE_WIDTH;
                loop {
                    match map[i] {
                        b'O' => {
                            box_count += 1;
                            i -= MAP_LINE_WIDTH;
                        }
                        b'#' => {
                            box_count = 0;
                            break;
                        }
                        b'.' => {
                            break;
                        }
                        x => panic!("Moved off map or something ({})", x as char),
                    }
                }
                if box_count != 0 {
                    map[i] = b'O';
                    map[robot_position] = b'.';
                    robot_position -= MAP_LINE_WIDTH;
                    map[robot_position] = b'@';
                } else if map[robot_position - MAP_LINE_WIDTH] == b'.' {
                    map[robot_position] = b'.';
                    robot_position -= MAP_LINE_WIDTH;
                    map[robot_position] = b'@';
                }
            }
            b'<' => {
                let mut box_count = 0;
                let mut i = robot_position - 1;
                loop {
                    match map[i] {
                        b'O' => {
                            box_count += 1;
                            i -= 1;
                        }
                        b'#' => {
                            box_count = 0;
                            break;
                        }
                        b'.' => {
                            break;
                        }
                        x => panic!("Moved off map or something ({})", x as char),
                    }
                }
                if box_count != 0 {
                    map[robot_position] = b'.';
                    robot_position -= 1;
                    map[i] = b'O';
                    map[robot_position] = b'@';
                } else if map[robot_position - 1] == b'.' {
                    map[robot_position] = b'.';
                    robot_position -= 1;
                    map[robot_position] = b'@';
                }
            }
            b'>' => {
                let mut box_count = 0;
                let mut i = robot_position + 1;
                loop {
                    match map[i] {
                        b'O' => {
                            box_count += 1;
                            i += 1;
                        }
                        b'#' => {
                            box_count = 0;
                            break;
                        }
                        b'.' => {
                            break;
                        }
                        x => panic!("Moved off map or something ({})", x as char),
                    }
                }
                if box_count != 0 {
                    map[robot_position] = b'.';
                    robot_position += 1;
                    map[i] = b'O';
                    map[robot_position] = b'@';
                } else if map[robot_position + 1] == b'.' {
                    map[robot_position] = b'.';
                    robot_position += 1;
                    map[robot_position] = b'@';
                }
            }
            b'v' => {
                let mut box_count = 0;
                let mut i = robot_position + MAP_LINE_WIDTH;
                loop {
                    match map[i] {
                        b'O' => {
                            box_count += 1;
                            i += MAP_LINE_WIDTH;
                        }
                        b'#' => {
                            box_count = 0;
                            break;
                        }
                        b'.' => {
                            break;
                        }
                        x => panic!("Moved off map or something ({})", x as char),
                    }
                }
                if box_count != 0 {
                    map[robot_position] = b'.';
                    robot_position += MAP_LINE_WIDTH;
                    map[i] = b'O';
                    map[robot_position] = b'@';
                } else if map[robot_position + MAP_LINE_WIDTH] == b'.' {
                    map[robot_position] = b'.';
                    robot_position += MAP_LINE_WIDTH;
                    map[robot_position] = b'@';
                }
            }
            _ => (),
        }
        move_i += 1;
    }

    let mut part1 = 0;
    let mut i = 0;
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if map[i] == b'O' {
                part1 += x + 100 * y;
            }
            i += 1;
        }
        i += MAP_LINE_WIDTH - MAP_WIDTH;
    }
    println!("{part1}");

    // =============================================================================

    const WIDE_MAP_WIDTH: usize = 2 * MAP_WIDTH;
    const WIDE_MAP_LINE_WIDTH: usize = WIDE_MAP_WIDTH + MAP_LINE_WIDTH - MAP_WIDTH;
    const WIDE_MAP_HEIGHT: usize = MAP_HEIGHT;

    let mut map = INPUT[..MAP_LINE_WIDTH * (MAP_HEIGHT - 1) + MAP_WIDTH]
        .iter()
        .copied()
        .flat_map(|spot| match spot {
            b'O' => b"[]".as_slice(),
            b'@' => b"@.",
            b'.' => b"..",
            b'#' => b"##",
            b'\r' => b"\r",
            b'\n' => b"\n",
            _ => panic!("Invalid item"),
        })
        .copied()
        .collect::<Vec<_>>();

    let mut robot_position = map.iter().position(|&spot| spot == b'@').unwrap();
    let mut move_i = START_OF_MOVEMENTS;
    while move_i < INPUT.len() {
        assert_eq!(
            robot_position,
            map.iter().position(|&spot| spot == b'@').unwrap()
        );

        println!(
            "\nStep {} ({})\n{}",
            move_i - START_OF_MOVEMENTS,
            if move_i == START_OF_MOVEMENTS {
                String::from("Nothing")
            } else {
                (INPUT[move_i - 1] as char).to_string()
            },
            map.iter().map(|&ch| ch as char).collect::<String>(),
        );
        if !map[WIDE_MAP_LINE_WIDTH * (WIDE_MAP_HEIGHT - 1)..][..WIDE_MAP_WIDTH]
            .iter()
            .all(|&x| x == b'#')
        {
            println!("broken");
            return;
        }
        match INPUT[move_i] {
            b'^' => 'outer: {
                match map[robot_position - WIDE_MAP_LINE_WIDTH] {
                    b'[' | b']' => {
                        let mut box_half_rows = Vec::new();
                        let mut robot_row = HashSet::new();
                        robot_row.insert(robot_position);
                        box_half_rows.push(robot_row);
                        'check_blockages: loop {
                            let box_halves_last_row = box_half_rows.last().unwrap();
                            let mut box_halves_this_row = HashSet::new();
                            for position in box_halves_last_row.iter().copied() {
                                let next_row_position = position - WIDE_MAP_LINE_WIDTH;
                                match map[next_row_position] {
                                    b'[' => {
                                        box_halves_this_row.insert(next_row_position);
                                        box_halves_this_row.insert(next_row_position + 1);
                                    }
                                    b']' => {
                                        box_halves_this_row.insert(next_row_position - 1);
                                        box_halves_this_row.insert(next_row_position);
                                    }
                                    b'#' => {
                                        break 'outer;
                                    }
                                    b'.' => (),
                                    x => panic!("What? ({})", x as char),
                                }
                            }
                            if box_halves_this_row.is_empty() {
                                break 'check_blockages;
                            }
                            box_half_rows.push(box_halves_this_row);
                        }
                        for row in box_half_rows.into_iter().rev() {
                            for position in row {
                                map[position - WIDE_MAP_LINE_WIDTH] = map[position];
                                map[position] = b'.';
                            }
                        }
                        robot_position -= WIDE_MAP_LINE_WIDTH;
                    }
                    b'#' => (),
                    b'.' => {
                        map[robot_position] = b'.';
                        robot_position -= WIDE_MAP_LINE_WIDTH;
                        map[robot_position] = b'@';
                    }
                    x => panic!("Moved off map or something ({})", x as char),
                }
            }
            b'<' => 'outer: {
                println!("<");
                match map[robot_position - 1] {
                    b'[' => panic!("Moved left into the left side of a box"),
                    b']' => {
                        let mut half_box_count = 0;
                        let mut i = robot_position - 1;
                        loop {
                            match map[i] {
                                b'[' | b']' => {
                                    half_box_count += 1;
                                }
                                b'#' => {
                                    break 'outer;
                                }
                                b'.' => {
                                    break;
                                }
                                _ => panic!("What? ({})", map[i] as char),
                            }

                            i -= 1;
                        }
                        assert_eq!(half_box_count % 2, 0);
                        map[robot_position] = b'.';
                        for box_pos in (i..robot_position - 1).step_by(2) {
                            map[box_pos] = b'[';
                            map[box_pos + 1] = b']';
                        }
                        robot_position -= 1;
                        map[robot_position] = b'@';
                    }
                    b'#' => break 'outer,
                    b'.' => {
                        println!("<.");
                        map[robot_position] = b'.';
                        robot_position -= 1;
                        map[robot_position] = b'@';
                    }
                    x => panic!("Moved off map or something ({})", x as char),
                }
            }
            b'>' => 'outer: {
                match map[robot_position + 1] {
                    b'[' => {
                        let mut half_box_count = 0;
                        let mut i = robot_position + 1;
                        loop {
                            match map[i] {
                                b'[' | b']' => {
                                    half_box_count += 1;
                                }
                                b'#' => {
                                    break 'outer;
                                }
                                b'.' => {
                                    break;
                                }
                                _ => panic!("What? ({})", map[i] as char),
                            }

                            i += 1;
                        }
                        assert_eq!(half_box_count % 2, 0);
                        map[robot_position] = b'.';
                        for box_pos in (robot_position + 2..=i).step_by(2) {
                            map[box_pos] = b'[';
                            map[box_pos + 1] = b']';
                        }
                        robot_position += 1;
                        map[robot_position] = b'@';
                    }
                    b']' => panic!("Moved right into the right side of a box"),
                    b'#' => break 'outer,
                    b'.' => {
                        map[robot_position] = b'.';
                        robot_position += 1;
                        map[robot_position] = b'@';
                    }
                    x => panic!("Moved off map or something ({})", x as char),
                }
            }
            b'v' => 'outer: {
                match map[robot_position + WIDE_MAP_LINE_WIDTH] {
                    b'[' | b']' => {
                        let mut box_half_rows = Vec::new();
                        let mut robot_row = HashSet::new();
                        robot_row.insert(robot_position);
                        box_half_rows.push(robot_row);
                        'check_blockages: loop {
                            let box_halves_last_row = box_half_rows.last().unwrap();
                            let mut box_halves_this_row = HashSet::new();
                            for position in box_halves_last_row.iter().copied() {
                                let next_row_position = position + WIDE_MAP_LINE_WIDTH;
                                match map[next_row_position] {
                                    b'[' => {
                                        box_halves_this_row.insert(next_row_position);
                                        box_halves_this_row.insert(next_row_position + 1);
                                    }
                                    b']' => {
                                        box_halves_this_row.insert(next_row_position - 1);
                                        box_halves_this_row.insert(next_row_position);
                                    }
                                    b'#' => {
                                        break 'outer;
                                    }
                                    b'.' => (),
                                    x => panic!("What? ({})", x as char),
                                }
                            }
                            if box_halves_this_row.is_empty() {
                                break 'check_blockages;
                            }
                            box_half_rows.push(box_halves_this_row);
                        }
                        for row in box_half_rows.into_iter().rev() {
                            for position in row {
                                map[position + WIDE_MAP_LINE_WIDTH] = map[position];
                                map[position] = b'.';
                            }
                        }
                        robot_position += WIDE_MAP_LINE_WIDTH;
                    }
                    b'#' => (),
                    b'.' => {
                        map[robot_position] = b'.';
                        robot_position += WIDE_MAP_LINE_WIDTH;
                        map[robot_position] = b'@';
                    }
                    x => panic!("Moved off map or something ({})", x as char),
                }
            }
            _ => (),
        }
        move_i += 1;
    }

    let mut part2 = 0;
    let mut i = 0;
    for y in 0..WIDE_MAP_HEIGHT {
        for x in 0..WIDE_MAP_WIDTH {
            if map[i] == b'[' {
                part2 += x + 100 * y;
            }
            i += 1;
        }
        i += WIDE_MAP_LINE_WIDTH - WIDE_MAP_WIDTH;
    }
    println!("Part 2: {part2}");
}
