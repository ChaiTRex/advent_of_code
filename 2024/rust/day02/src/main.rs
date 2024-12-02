fn main() {
    static INPUT: &str = include_str!("../../../day02.txt");

    let start = std::time::Instant::now();

    let inputs = INPUT
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    'outer: for input in &inputs {
        if input[0] < input[1] {
            for (a, b) in input.iter().zip(input.iter().skip(1)) {
                if a >= b || !(1..=3).contains(&(b - a)) {
                    continue 'outer;
                }
            }
            part1 += 1;
        } else if input[0] > input[1] {
            for (a, b) in input.iter().zip(input.iter().skip(1)) {
                if a <= b || !(1..=3).contains(&(a - b)) {
                    continue 'outer;
                }
            }
            part1 += 1;
        }
    }

    let mut part2 = 0;
    'outer: for input in &inputs {
        'remover: for i in 0..input.len() {
            let mut input = input.clone();
            input.remove(i);
            if input[0] < input[1] {
                for (a, b) in input.iter().zip(input.iter().skip(1)) {
                    if a >= b || !(1..=3).contains(&(b - a)) {
                        continue 'remover;
                    }
                }
                part2 += 1;
                continue 'outer;
            } else if input[0] > input[1] {
                for (a, b) in input.iter().zip(input.iter().skip(1)) {
                    if a <= b || !(1..=3).contains(&(a - b)) {
                        continue 'remover;
                    }
                }
                part2 += 1;
                continue 'outer;
            }
        }
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
