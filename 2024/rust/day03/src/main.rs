fn main() {
    static INPUT: &str = include_str!("../../../day03.txt");

    let start = std::time::Instant::now();

    let part1 = INPUT
        .split("mul(")
        .map(|thingy| {
            let (a, rest) = thingy.split_once(',')?;
            let (b, _) = rest.split_once(')')?;
            let a = a.parse::<u32>().ok()?;
            let b = b.parse::<u32>().ok()?;
            Some(a * b)
        })
        .flatten()
        .sum::<u32>();

    let part2 = INPUT
        .split("do()")
        .map(|thingy| {
            let (blah, _) = thingy.split_once("don't()").unwrap_or((thingy, ""));
            blah.split("mul(")
                .map(|thingy| {
                    let (a, rest) = thingy.split_once(',')?;
                    let (b, _) = rest.split_once(')')?;
                    let a = a.parse::<u32>().ok()?;
                    let b = b.parse::<u32>().ok()?;
                    Some(a * b)
                })
                .flatten()
                .sum::<u32>()
        })
        .sum::<u32>();

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
