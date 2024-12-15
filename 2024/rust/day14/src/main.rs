fn main() {
    let start = std::time::Instant::now();

    let mut robots = INPUT
        .lines()
        .map(|mut line| {
            line = &line[2..];
            let (x, line) = line.split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let (y, line) = line.split_once(" v=").unwrap();
            let y = y.parse::<i32>().unwrap();
            let (vx, vy) = line.split_once(',').unwrap();
            let vx = vx.parse::<i32>().unwrap();
            let vy = vy.parse::<i32>().unwrap();

            ((x, y), (vx, vy))
        })
        .collect::<Vec<_>>();

    let mut upper_left = 0;
    let mut upper_right = 0;
    let mut lower_left = 0;
    let mut lower_right = 0;
    for &((mut x, mut y), (vx, vy)) in &robots {
        x = (x + 100 * vx).rem_euclid(WIDTH as i32);
        y = (y + 100 * vy).rem_euclid(HEIGHT as i32);
        if x < WIDTH as i32 / 2 {
            if y < HEIGHT as i32 / 2 {
                upper_left += 1;
            } else if y > HEIGHT as i32 / 2 {
                lower_left += 1;
            }
        } else if x > WIDTH as i32 / 2 {
            if y < HEIGHT as i32 / 2 {
                upper_right += 1;
            } else if y > HEIGHT as i32 / 2 {
                lower_right += 1;
            }
        }
    }
    let part1 = upper_left * upper_right * lower_left * lower_right;

    let mut horizontal_band = 0;
    let mut vertical_band = 0;
    let mut i = 0;
    loop {
        if horizontal_band != 0 && vertical_band != 0 {
            break;
        }
        let mut row_counts = [0; HEIGHT];
        let mut column_counts = [0; WIDTH];
        let mut map = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
        for ((x, y), _) in robots.iter().cloned() {
            row_counts[y as usize] += 1;
            column_counts[x as usize] += 1;
            map[y as usize][x as usize] = '*';
        }

        if horizontal_band == 0
            && i < HEIGHT
            && row_counts.windows(CHRISTMAS_TREE_HEIGHT).any(|slice| {
                slice
                    .into_iter()
                    .zip(CHRISTMAS_TREE_ROW_COUNTS.iter())
                    .all(|(map_count, tree_count)| map_count >= tree_count)
            })
        {
            horizontal_band = i;
        }
        if vertical_band == 0
            && i < WIDTH
            && column_counts.windows(CHRISTMAS_TREE_HEIGHT).any(|slice| {
                slice
                    .into_iter()
                    .zip(CHRISTMAS_TREE_COLUMN_COUNTS.iter())
                    .all(|(map_count, tree_count)| map_count >= tree_count)
            })
        {
            vertical_band = i;
        }

        for ((x, y), (vx, vy)) in &mut robots {
            *x = (*x + *vx).rem_euclid(WIDTH as i32);
            *y = (*y + *vy).rem_euclid(HEIGHT as i32);
        }
        i += 1;
    }

    // Chinese remainder theorem with coprime moduluses
    let z1 = modular_inverse(HEIGHT as isize, WIDTH as isize).rem_euclid(WIDTH as isize) as usize;
    let z2 = modular_inverse(WIDTH as isize, HEIGHT as isize).rem_euclid(HEIGHT as isize) as usize;
    let part2 = (vertical_band * HEIGHT * z1 + horizontal_band * WIDTH * z2) % (WIDTH * HEIGHT);

    let time = start.elapsed();

    let mut map = [[b' '; WIDTH]; HEIGHT];
    for ((mut x, mut y), (vx, vy)) in robots {
        x = (x + vx * ((part2 - i) as i32)).rem_euclid(WIDTH as i32);
        y = (y + vy * ((part2 - i) as i32)).rem_euclid(HEIGHT as i32);
        map[y as usize][x as usize] = b'*';
    }
    print!("+");
    for _ in 0..WIDTH {
        print!("-");
    }
    println!("+");
    for y in 0..HEIGHT {
        let row = map[y];
        print!("|");
        for x in 0..WIDTH {
            print!("{}", row[x] as char);
        }
        println!("|");
    }
    print!("+");
    for _ in 0..WIDTH {
        print!("-");
    }
    println!("+");
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}");
}

static INPUT: &str = include_str!("../../../day14.txt");
const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const CHRISTMAS_TREE_WIDTH: usize = CHRISTMAS_TREE[0].len();
const CHRISTMAS_TREE_HEIGHT: usize = CHRISTMAS_TREE.len();
const CHRISTMAS_TREE: [[u8; 31]; 33] = [
    *b"*******************************",
    *b"*                             *",
    *b"*                             *",
    *b"*                             *",
    *b"*                             *",
    *b"*              *              *",
    *b"*             ***             *",
    *b"*            *****            *",
    *b"*           *******           *",
    *b"*          *********          *",
    *b"*            *****            *",
    *b"*           *******           *",
    *b"*          *********          *",
    *b"*         ***********         *",
    *b"*        *************        *",
    *b"*          *********          *",
    *b"*         ***********         *",
    *b"*        *************        *",
    *b"*       ***************       *",
    *b"*      *****************      *",
    *b"*        *************        *",
    *b"*       ***************       *",
    *b"*      *****************      *",
    *b"*     *******************     *",
    *b"*    *********************    *",
    *b"*             ***             *",
    *b"*             ***             *",
    *b"*             ***             *",
    *b"*                             *",
    *b"*                             *",
    *b"*                             *",
    *b"*                             *",
    *b"*******************************",
];
static CHRISTMAS_TREE_ROW_COUNTS: [u8; CHRISTMAS_TREE_HEIGHT] = row_counts(CHRISTMAS_TREE);
static CHRISTMAS_TREE_COLUMN_COUNTS: [u8; CHRISTMAS_TREE_WIDTH] = column_counts(CHRISTMAS_TREE);

const fn row_counts<const W: usize, const H: usize>(rows: [[u8; W]; H]) -> [u8; H] {
    let mut result = [0; H];

    let mut y = 0;
    while y < H {
        let row = rows[y];
        let mut x = 0;
        while x < W {
            if row[x] == b'*' {
                result[y] += 1;
            }
            x += 1;
        }
        y += 1;
    }

    result
}

const fn column_counts<const W: usize, const H: usize>(rows: [[u8; W]; H]) -> [u8; W] {
    let mut result = [0; W];

    let mut y = 0;
    while y < H {
        let row = rows[y];
        let mut x = 0;
        while x < W {
            if row[x] == b'*' {
                result[x] += 1;
            }
            x += 1;
        }
        y += 1;
    }

    result
}

const fn extended_gcd(a: isize, b: isize) -> (isize, (isize, isize)) {
    if b == 0 {
        return (a, (1, 0));
    }
    let mut q = [0, 0];
    let mut r = [a, b];
    let mut s = [1, 0];
    let mut t = [0, 1];

    loop {
        q[0] = q[1];
        q[1] = r[0] / r[1];
        let mut tmp = r[0] % r[1];
        if tmp == 0 {
            return (r[1], (s[1], t[1]));
        }
        r[0] = r[1];
        r[1] = tmp;
        tmp = s[0] - q[1] * s[1];
        s[0] = s[1];
        s[1] = tmp;
        tmp = t[0] - q[1] * t[1];
        t[0] = t[1];
        t[1] = tmp;
    }
}

const fn modular_inverse(x: isize, m: isize) -> isize {
    extended_gcd(x, m).1.0
}
