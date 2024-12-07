fn ilog10(val: u16) -> usize {
    // Specialized-to-this-problem version of the Rust language
    // standard library's `u8::ilog10` function.
    //
    // https://github.com/rust-lang/rust/blob/404e9c5e3ad75057b6bbb3bcd44fe60480e50294/library/core/src/num/int_log10.rs#L6-L23

    // For better performance, avoid branches by assembling the solution
    // in the bits above the low 10 bits.

    // Adding c1 to val gives 10 in the top bits for val < 10, 11 for val >= 10
    const C1: u16 = 0b11_0000000000 - 10; // 3062
    // Adding c2 to val gives 01 in the top bits for val < 100, 10 for val >= 100
    const C2: u16 = 0b10_0000000000 - 100; // 1948

    // Value of top bits:
    //            +c1  +c2  1&2
    //     1..=9   10   01   00 = 0
    //   10..=99   11   01   01 = 1
    // 100..=999   11   10   10 = 2
    (((val + C1) & (val + C2)) >> 10) as usize
}

fn main() {
    static INPUT: &str = include_str!("../../../day07.txt");

    fn f(correct_result: u64, current_result: u64, operands: &[u16]) -> bool {
        match *operands {
            [] => correct_result == current_result,
            [operand, ref operands @ ..] => {
                f(correct_result, current_result + operand as u64, operands)
                    || f(correct_result, current_result * operand as u64, operands)
            }
        }
    }

    fn g(correct_result: u64, current_result: u64, operands: &[u16]) -> bool {
        match *operands {
            [] => correct_result == current_result,
            [operand, ref operands @ ..] => {
                g(correct_result, current_result + operand as u64, operands)
                    || g(correct_result, current_result * operand as u64, operands)
                    || g(
                        correct_result,
                        [10, 100, 1000][ilog10(operand)] * current_result + operand as u64,
                        operands,
                    )
            }
        }
    }

    let start = std::time::Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut operands = Vec::with_capacity(11);
    for line in INPUT.lines() {
        let (correct_result, operands_str) = line.split_once(": ").unwrap();
        let correct_result = correct_result.parse::<u64>().unwrap();
        let mut iter = operands_str
            .split(' ')
            .map(|operand| operand.parse::<u16>().unwrap());
        let operand = iter.next().unwrap() as u64;
        operands.extend(iter);

        if f(correct_result, operand, &operands) {
            part1 += correct_result;
            part2 += correct_result;
        } else if g(correct_result, operand, &operands) {
            part2 += correct_result;
        }

        operands.clear();
    }

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}
