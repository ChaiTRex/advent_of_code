use core::fmt::Write;

fn main() {
    static INPUT: &str = include_str!("../../../day17.txt");

    let start = std::time::Instant::now();

    let (registers, instructions) = INPUT.split_once("\n\n").unwrap();
    let registers = registers
        .lines()
        .map(|line| line[12..].parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let registers = [registers[0], registers[1], registers[2]];
    let original_machine_code = instructions[9..].trim_end();
    let mut iter = original_machine_code.split(',');
    let original_machine_code = iter
        .clone()
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut instructions = Vec::new();
    loop {
        match (iter.next(), iter.next()) {
            (Some(opcode), Some(operand)) => instructions.push((
                Opcode::try_from(opcode.parse::<u8>().unwrap()).unwrap(),
                operand.parse::<u8>().unwrap(),
            )),
            (None, None) => break,
            _ => panic!(),
        }
    }

    let outputs = execute(&instructions, registers);

    let mut part1 = String::new();
    write!(part1, "{}", outputs[0]).unwrap();
    for output in &outputs[1..] {
        write!(part1, ",").unwrap();
        write!(part1, "{output}").unwrap();
    }

    let mut part2_candidates = vec![0];
    for i in (0..original_machine_code.len()).rev() {
        let target = &original_machine_code[i..];
        part2_candidates = part2_candidates
            .into_iter()
            .flat_map(|candidate| (0..8).map(move |n| 8 * candidate + n))
            .filter(|&candidate| execute(&instructions, [candidate, 0, 0]) == target)
            .collect::<Vec<_>>();
    }
    let part2 = part2_candidates.iter().min().unwrap();

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}

fn execute(instructions: &[(Opcode, u8)], mut registers: [i64; 3]) -> Vec<u8> {
    let mut instruction_pointer = 0;
    let mut outputs = Vec::new();
    while instruction_pointer < instructions.len() {
        let (opcode, operand) = instructions[instruction_pointer];
        match opcode {
            Opcode::Adv => registers[0] /= 1 << combo_operand(operand, &registers),
            Opcode::Bxl => registers[1] ^= operand as i64,
            Opcode::Bst => registers[1] = combo_operand(operand, &registers) % 8,
            Opcode::Jnz => {
                if registers[0] == 0 {
                    instruction_pointer += 1;
                } else {
                    instruction_pointer = operand as usize;
                }
            }
            Opcode::Bxc => registers[1] ^= registers[2],
            Opcode::Out => outputs.push((combo_operand(operand, &registers) % 8) as u8),
            Opcode::Bdv => registers[1] = registers[0] / (1 << combo_operand(operand, &registers)),
            Opcode::Cdv => registers[2] = registers[0] / (1 << combo_operand(operand, &registers)),
        }
        if opcode != Opcode::Jnz {
            instruction_pointer += 1
        };
    }

    outputs
}

fn combo_operand(operand: u8, registers: &[i64]) -> i64 {
    match operand {
        x @ 0..=3 => x as i64,
        i @ 4..=6 => registers[i as usize - 4],
        7 => panic!("Combo operand 7 is invalid"),
        _ => panic!("Operand isn't in 0..=7"),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

fn to_asm(opcode: Opcode, operand: u8) -> String {
    fn combo_operand(operand: u8) -> &'static str {
        match operand {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "A",
            5 => "B",
            6 => "C",
            7 => panic!("Combo operand 7 is invalid"),
            _ => panic!("Operand isn't in 0..=7"),
        }
    }
    match opcode {
        Opcode::Adv => format!("Adv {}", combo_operand(operand)),
        Opcode::Bxl => format!("Bxl {operand}"),
        Opcode::Bst => format!("Bst {}", combo_operand(operand)),
        Opcode::Jnz => format!("Jnz {operand}"),
        Opcode::Bxc => format!("Bxc"),
        Opcode::Out => format!("Out {}", combo_operand(operand)),
        Opcode::Bdv => format!("Bdv {}", combo_operand(operand)),
        Opcode::Cdv => format!("Cdv {}", combo_operand(operand)),
    }
}

impl TryFrom<u8> for Opcode {
    type Error = InvalidOpcode;

    fn try_from(opcode: u8) -> Result<Opcode, InvalidOpcode> {
        match opcode {
            0 => Ok(Opcode::Adv),
            1 => Ok(Opcode::Bxl),
            2 => Ok(Opcode::Bst),
            3 => Ok(Opcode::Jnz),
            4 => Ok(Opcode::Bxc),
            5 => Ok(Opcode::Out),
            6 => Ok(Opcode::Bdv),
            7 => Ok(Opcode::Cdv),
            _ => Err(InvalidOpcode),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct InvalidOpcode;
