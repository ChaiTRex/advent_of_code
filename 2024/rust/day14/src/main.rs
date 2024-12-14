use std::io::stdin;

fn main() {
    static INPUT: &str = include_str!("../../../day14.txt");
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

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
    println!("{robots:?}");

    for i in 0..usize::MAX {
        if i == 100 {
            let mut upper_left = 0;
            let mut upper_right = 0;
            let mut lower_left = 0;
            let mut lower_right = 0;
            for &((x, y), _) in &robots {
                if x < WIDTH / 2 {
                    if y < HEIGHT / 2 {
                        upper_left += 1;
                    } else if y > HEIGHT / 2 {
                        lower_left += 1;
                    }
                } else if x > WIDTH / 2 {
                    if y < HEIGHT / 2 {
                        upper_right += 1;
                    } else if y > HEIGHT / 2 {
                        lower_right += 1;
                    }
                }
            }
            let part1 = upper_left * upper_right * lower_left * lower_right;
            println!("{part1}");
        }

        let mut map = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
        for ((x, y), _) in &robots {
            map[*y as usize][*x as usize] = '*';
        }
        if i % 101 == 12 {
            println!("{i}");
            print!("+");
            for _ in 0..WIDTH {
                print!("-");
            }
            println!("+");
            for row in map.iter() {
                print!("|");
                for spot in row.iter() {
                    print!("{spot}");
                }
                println!("|");
            }
            print!("+");
            for _ in 0..WIDTH {
                print!("-");
            }
            println!("+");
            let _ = stdin().read_line(&mut String::new());
        }
        for ((x, y), (vx, vy)) in &mut robots {
            map[*y as usize][*x as usize] = '*';
            *x = (*x + *vx).rem_euclid(WIDTH);
            *y = (*y + *vy).rem_euclid(HEIGHT);
        }
    }
}
