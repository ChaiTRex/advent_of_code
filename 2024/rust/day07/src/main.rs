fn main() {
    static INPUT: &str = include_str!("../../../day07.txt");

    fn f(correct_result: u64, operands: &[u64], current_result: u64) -> bool {
        if operands.is_empty() {
            return correct_result == current_result;
        }
        f(correct_result, &operands[1..], current_result + operands[0])
            || f(correct_result, &operands[1..], current_result * operands[0])
            || f(
                correct_result,
                &operands[1..],
                (current_result.to_string() + &operands[0].to_string())
                    .parse::<u64>()
                    .unwrap(),
            )
    }

    let mut part1 = 0;

    for line in INPUT.lines() {
        let (result, operands) = line.split_once(": ").unwrap();
        let correct_result = result.parse::<u64>().unwrap();
        let mut operands = operands
            .split(' ')
            .map(|operand| operand.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        if f(correct_result, &operands[1..], operands[0]) {
            part1 += correct_result;
        }
    }

    println!("{part1}");
}
