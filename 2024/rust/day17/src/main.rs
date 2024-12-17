fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day17.txt");

    let start = std::time::Instant::now();

    let mut a = (INPUT[12] - b'0') as u64;
    let mut i = 13;
    while INPUT[i].is_ascii_digit() {
        a = 10 * a + (INPUT[i] - b'0') as u64;
        i += 1;
    }
    while INPUT[i] != b'R' {
        i += 1;
    }
    i += 12;
    let mut b = (INPUT[i] - b'0') as u64;
    i += 1;
    while INPUT[i].is_ascii_digit() {
        b = 10 * b + (INPUT[i] - b'0') as u64;
        i += 1;
    }
    while INPUT[i] != b'R' {
        i += 1;
    }
    i += 12;
    let mut c = (INPUT[i] - b'0') as u64;
    i += 1;
    while INPUT[i].is_ascii_digit() {
        c = 10 * c + (INPUT[i] - b'0') as u64;
        i += 1;
    }

    while INPUT[i] != b'P' {
        i += 1;
    }
    i += 9;

    let mut machine_code = Vec::with_capacity(16);
    while i < INPUT.len() && INPUT[i].is_ascii_digit() {
        machine_code.push(INPUT[i] - b'0');
        machine_code.push(INPUT[i + 2] - b'0');
        i += 4;
    }
    let mut instructions = Vec::new();
    let mut i = 0;
    while i < machine_code.len() {
        instructions.push(Instruction::new(machine_code[i], machine_code[i + 1]).unwrap());
        i += 2;
    }

    let outputs = execute(&instructions, a, b, c);
    let mut part1 = Vec::with_capacity(2 * outputs.len() - 1);
    part1.push(outputs[0] + b'0');
    let mut i = 1;
    while i < outputs.len() {
        part1.push(b',');
        part1.push(outputs[i] + b'0');
        i += 1;
    }
    // SAFETY: `part1` is filled with ASCII characters, so it's UTF-8 compliant.
    let part1 = unsafe { String::from_utf8_unchecked(part1) };

    let mut part2_candidates = vec![0];
    for i in (0..machine_code.len()).rev() {
        let target = &machine_code[i..];
        part2_candidates = part2_candidates
            .into_iter()
            .flat_map(|candidate| (0..8).map(move |n| 8 * candidate + n))
            .filter(|&candidate| execute(&instructions, candidate, 0, 0) == target)
            .collect::<Vec<_>>();
    }
    let part2 = part2_candidates[0];

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);
}

pub fn execute(instructions: &[Instruction], mut a: u64, mut b: u64, mut c: u64) -> Vec<u8> {
    let mut instruction_pointer = 0;
    let mut outputs = Vec::new();

    while instruction_pointer < instructions.len() {
        let instruction = instructions[instruction_pointer];
        instruction_pointer += 1;
        match instruction {
            Instruction::Adv0 => (),
            Instruction::Adv1 => a >>= 1,
            Instruction::Adv2 => a >>= 2,
            Instruction::Adv3 => a >>= 3,
            Instruction::AdvA => a = 0,
            Instruction::AdvB => a >>= b,
            Instruction::AdvC => a >>= c,
            Instruction::Bxl0 => (),
            Instruction::Bxl1 => b ^= 1,
            Instruction::Bxl2 => b ^= 2,
            Instruction::Bxl3 => b ^= 3,
            Instruction::Bxl4 => b ^= 4,
            Instruction::Bxl5 => b ^= 5,
            Instruction::Bxl6 => b ^= 6,
            Instruction::Bxl7 => b ^= 7,
            Instruction::Bst0 => b = 0,
            Instruction::Bst1 => b = 1,
            Instruction::Bst2 => b = 2,
            Instruction::Bst3 => b = 3,
            Instruction::BstA => b = a & 0b111,
            Instruction::BstB => b &= 0b111,
            Instruction::BstC => b = c & 0b111,
            Instruction::Jnz0 => {
                if a != 0 {
                    instruction_pointer = 0;
                }
            }
            Instruction::Jnz2 => {
                if a != 0 {
                    instruction_pointer = 1;
                }
            }
            Instruction::Jnz4 => {
                if a != 0 {
                    instruction_pointer = 2;
                }
            }
            Instruction::Jnz6 => {
                if a != 0 {
                    instruction_pointer = 3;
                }
            }
            Instruction::Jnz1 | Instruction::Jnz3 | Instruction::Jnz5 | Instruction::Jnz7 => {
                // Don't forget to double the nearby `instruction_pointer =` lines when implementing this.
                unimplemented!("Jumping to an odd address is unsupported.")
            }
            Instruction::Bxc => b ^= c,
            Instruction::Out0 => outputs.push(0),
            Instruction::Out1 => outputs.push(1),
            Instruction::Out2 => outputs.push(2),
            Instruction::Out3 => outputs.push(3),
            Instruction::OutA => outputs.push(a as u8 & 0b111),
            Instruction::OutB => outputs.push(b as u8 & 0b111),
            Instruction::OutC => outputs.push(c as u8 & 0b111),
            Instruction::Bdv0 => b = a,
            Instruction::Bdv1 => b = a >> 1,
            Instruction::Bdv2 => b = a >> 2,
            Instruction::Bdv3 => b = a >> 3,
            Instruction::BdvA => b = 0,
            Instruction::BdvB => b = a >> b,
            Instruction::BdvC => b = a >> c,
            Instruction::Cdv0 => c = a,
            Instruction::Cdv1 => c = a >> 1,
            Instruction::Cdv2 => c = a >> 2,
            Instruction::Cdv3 => c = a >> 3,
            Instruction::CdvA => c = 0,
            Instruction::CdvB => c = a >> b,
            Instruction::CdvC => c = a >> c,
        }
    }

    outputs
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Instruction {
    Adv0 = (0 << 3) | 0,
    Adv1 = (0 << 3) | 1,
    Adv2 = (0 << 3) | 2,
    Adv3 = (0 << 3) | 3,
    AdvA = (0 << 3) | 4,
    AdvB = (0 << 3) | 5,
    AdvC = (0 << 3) | 6,
    Bxl0 = (1 << 3) | 0,
    Bxl1 = (1 << 3) | 1,
    Bxl2 = (1 << 3) | 2,
    Bxl3 = (1 << 3) | 3,
    Bxl4 = (1 << 3) | 4,
    Bxl5 = (1 << 3) | 5,
    Bxl6 = (1 << 3) | 6,
    Bxl7 = (1 << 3) | 7,
    Bst0 = (2 << 3) | 0,
    Bst1 = (2 << 3) | 1,
    Bst2 = (2 << 3) | 2,
    Bst3 = (2 << 3) | 3,
    BstA = (2 << 3) | 4,
    BstB = (2 << 3) | 5,
    BstC = (2 << 3) | 6,
    Jnz0 = (3 << 3) | 0,
    Jnz1 = (3 << 3) | 1,
    Jnz2 = (3 << 3) | 2,
    Jnz3 = (3 << 3) | 3,
    Jnz4 = (3 << 3) | 4,
    Jnz5 = (3 << 3) | 5,
    Jnz6 = (3 << 3) | 6,
    Jnz7 = (3 << 3) | 7,
    Bxc = (4 << 3) | 0,
    Out0 = (5 << 3) | 0,
    Out1 = (5 << 3) | 1,
    Out2 = (5 << 3) | 2,
    Out3 = (5 << 3) | 3,
    OutA = (5 << 3) | 4,
    OutB = (5 << 3) | 5,
    OutC = (5 << 3) | 6,
    Bdv0 = (6 << 3) | 0,
    Bdv1 = (6 << 3) | 1,
    Bdv2 = (6 << 3) | 2,
    Bdv3 = (6 << 3) | 3,
    BdvA = (6 << 3) | 4,
    BdvB = (6 << 3) | 5,
    BdvC = (6 << 3) | 6,
    Cdv0 = (7 << 3) | 0,
    Cdv1 = (7 << 3) | 1,
    Cdv2 = (7 << 3) | 2,
    Cdv3 = (7 << 3) | 3,
    CdvA = (7 << 3) | 4,
    CdvB = (7 << 3) | 5,
    CdvC = (7 << 3) | 6,
}

impl Instruction {
    pub fn new(opcode: u8, operand: u8) -> Result<Self, InvalidInstruction> {
        if (opcode | operand) > 7 {
            Err(InvalidInstruction(opcode, operand))
        } else if opcode == 4 {
            Ok(Self::Bxc)
        } else if (opcode | 2) != 3 && operand == 7 {
            Err(InvalidInstruction(opcode, operand))
        } else {
            // Safety: all invalid inputs have been handled above.
            Ok(unsafe { core::mem::transmute((opcode << 3) | operand) })
        }
    }

    pub fn new_from_ascii(
        ascii_opcode: u8,
        ascii_operand: u8,
    ) -> Result<Self, InvalidInstructionFromAscii> {
        let opcode = ascii_opcode.wrapping_sub(b'0');
        let operand = ascii_operand.wrapping_sub(b'0');
        if (opcode | operand) > 7 {
            Err(InvalidInstructionFromAscii(opcode, operand))
        } else if opcode == 4 {
            Ok(Self::Bxc)
        } else if (opcode | 2) != 3 && operand == 7 {
            Err(InvalidInstructionFromAscii(opcode, operand))
        } else {
            // Safety: all invalid inputs have been handled above.
            Ok(unsafe { core::mem::transmute((opcode << 3) | operand) })
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        static INSTRUCTIONS: [Instruction; 52] = [
            Instruction::Adv0,
            Instruction::Adv1,
            Instruction::Adv2,
            Instruction::Adv3,
            Instruction::AdvA,
            Instruction::AdvB,
            Instruction::AdvC,
            Instruction::Bxl0,
            Instruction::Bxl1,
            Instruction::Bxl2,
            Instruction::Bxl3,
            Instruction::Bxl4,
            Instruction::Bxl5,
            Instruction::Bxl6,
            Instruction::Bxl7,
            Instruction::Bst0,
            Instruction::Bst1,
            Instruction::Bst2,
            Instruction::Bst3,
            Instruction::BstA,
            Instruction::BstB,
            Instruction::BstC,
            Instruction::Jnz0,
            Instruction::Jnz1,
            Instruction::Jnz2,
            Instruction::Jnz3,
            Instruction::Jnz4,
            Instruction::Jnz5,
            Instruction::Jnz6,
            Instruction::Jnz7,
            Instruction::Bxc,
            Instruction::Out0,
            Instruction::Out1,
            Instruction::Out2,
            Instruction::Out3,
            Instruction::OutA,
            Instruction::OutB,
            Instruction::OutC,
            Instruction::Bdv0,
            Instruction::Bdv1,
            Instruction::Bdv2,
            Instruction::Bdv3,
            Instruction::BdvA,
            Instruction::BdvB,
            Instruction::BdvC,
            Instruction::Cdv0,
            Instruction::Cdv1,
            Instruction::Cdv2,
            Instruction::Cdv3,
            Instruction::CdvA,
            Instruction::CdvB,
            Instruction::CdvC,
        ];

        INSTRUCTIONS.into_iter()
    }

    pub fn to_asm(self) -> &'static str {
        static INSTRUCTIONS_IN_ASM: [&str; 63] = [
            "adv 0",
            "adv 1",
            "adv 2",
            "adv 3",
            "adv A",
            "adv B",
            "adv C",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "bxl 0",
            "bxl 1",
            "bxl 2",
            "bxl 3",
            "bxl 4",
            "bxl 5",
            "bxl 6",
            "bxl 7",
            "bst 0",
            "bst 1",
            "bst 2",
            "bst 3",
            "bst A",
            "bst B",
            "bst C",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "jnz 0",
            "jnz 1",
            "jnz 2",
            "jnz 3",
            "jnz 4",
            "jnz 5",
            "jnz 6",
            "jnz 7",
            "bxc",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "out 0",
            "out 1",
            "out 2",
            "out 3",
            "out A",
            "out B",
            "out C",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "bdv 0",
            "bdv 1",
            "bdv 2",
            "bdv 3",
            "bdv A",
            "bdv B",
            "bdv C",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "cdv 0",
            "cdv 1",
            "cdv 2",
            "cdv 3",
            "cdv A",
            "cdv B",
            "cdv C",
        ];

        INSTRUCTIONS_IN_ASM[self as usize]
    }
    pub fn to_rust(self) -> &'static str {
        static INSTRUCTIONS_IN_RUST: [&str; 63] = [
            "()",
            "a >>= 1",
            "a >>= 2",
            "a >>= 3",
            "a = 0",
            "a >>= b",
            "a >>= c",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "()",
            "b ^= 1",
            "b ^= 2",
            "b ^= 3",
            "b ^= 4",
            "b ^= 5",
            "b ^= 6",
            "b ^= 7",
            "b = 0",
            "b = 1",
            "b = 2",
            "b = 3",
            "b = a & 0b111",
            "b &= 0b111",
            "b = c & 0b111",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "if a != 0 { instruction_pointer = 0; }",
            "if a != 0 { instruction_pointer = 1; }",
            "if a != 0 { instruction_pointer = 2; }",
            "if a != 0 { instruction_pointer = 3; }",
            "if a != 0 { instruction_pointer = 4; }",
            "if a != 0 { instruction_pointer = 5; }",
            "if a != 0 { instruction_pointer = 6; }",
            "if a != 0 { instruction_pointer = 7; }",
            "b ^= c",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "outputs.push(0)",
            "outputs.push(1)",
            "outputs.push(2)",
            "outputs.push(3)",
            "outputs.push(a as u8 & 0b111)",
            "outputs.push(b as u8 & 0b111)",
            "outputs.push(c as u8 & 0b111)",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "b = a",
            "b = a >> 1",
            "b = a >> 2",
            "b = a >> 3",
            "b = 0",
            "b = a >> b",
            "b = a >> c",
            "[ERROR: Invalid bit pattern in this `Instruction`]",
            "c = a",
            "c = a >> 1",
            "c = a >> 2",
            "c = a >> 3",
            "c = 0",
            "c = a >> b",
            "c = a >> c",
        ];

        INSTRUCTIONS_IN_RUST[self as usize]
    }
}

impl core::fmt::Display for Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.to_asm())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvalidInstruction(u8, u8);

impl InvalidInstruction {
    pub fn opcode(self) -> u8 {
        self.0
    }

    pub fn operand(self) -> u8 {
        self.1
    }
}

impl core::fmt::Display for InvalidInstruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (self.0, self.1) {
            (0 | 2 | 5 | 6 | 7, operand) => {
                f.write_str("The combo operand was invalid (")?;
                operand.fmt(f)?;
                f.write_str(" is not in `0..=6`).")
            }
            (1 | 3 | 4, operand) => {
                f.write_str("The literal operand was invalid (")?;
                operand.fmt(f)?;
                f.write_str(" is not in `0..=7`).")
            }
            (opcode, 0..=7) => {
                f.write_str("The opcode was invalid (")?;
                opcode.fmt(f)?;
                f.write_str(" is not in `0..=7`).")
            }
            (opcode, operand) => {
                f.write_str("The opcode and operand were invalid (the opcode ")?;
                opcode.fmt(f)?;
                f.write_str(" and the operand ")?;
                operand.fmt(f)?;
                f.write_str(" are not in `0..=7`).")
            }
        }
    }
}

impl core::error::Error for InvalidInstruction {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvalidInstructionFromAscii(u8, u8);

impl InvalidInstructionFromAscii {
    pub fn opcode(self) -> char {
        self.0 as char
    }

    pub fn operand(self) -> char {
        self.1 as char
    }
}

impl core::fmt::Display for InvalidInstructionFromAscii {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (self.0, self.1) {
            (b'0' | b'2' | b'5' | b'6' | b'7', operand) => {
                f.write_str("The combo operand was invalid (")?;
                <char as core::fmt::Debug>::fmt(&(operand as char), f)?;
                f.write_str(" is not in `'0'..='6'`).")
            }
            (b'1' | b'3' | b'4', operand) => {
                f.write_str("The literal operand was invalid (")?;
                <char as core::fmt::Debug>::fmt(&(operand as char), f)?;
                f.write_str(" is not in `'0'..='7'`).")
            }
            (opcode, b'0'..=b'7') => {
                f.write_str("The opcode was invalid (")?;
                <char as core::fmt::Debug>::fmt(&(opcode as char), f)?;
                f.write_str(" is not in `'0'..='7'`).")
            }
            (opcode, operand) => {
                f.write_str("The opcode and operand were invalid (the opcode ")?;
                <char as core::fmt::Debug>::fmt(&(opcode as char), f)?;
                f.write_str(" and the operand ")?;
                <char as core::fmt::Debug>::fmt(&(operand as char), f)?;
                f.write_str(" are not in `'0'..='7'`).")
            }
        }
    }
}

impl core::error::Error for InvalidInstructionFromAscii {}
