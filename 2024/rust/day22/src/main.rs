fn main() {
    static INPUT: &str = include_str!("../../../day22.txt");

    let start = std::time::Instant::now();

    let inputs = INPUT
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for n in inputs.iter().copied() {
        /*for _ in 0..2000 {
            n = next_secret(n);
        }*/
        part1 += two_thousand_secret_new(n) as u64;
    }

    let mut all_sequence_prices = [0; 126892];
    for mut n in inputs.into_iter() {
        let mut seen_changes = [false; 126892];

        let mut units = [0; 2001];
        for i in 0..2001 {
            units[i] = (n % 10) as i8;
            n = next_secret(n);
        }
        for xs in units.windows(5) {
            let changes = 65160
                + xs[4] as usize
                + 18 * xs[3] as usize
                + 342 * xs[2] as usize
                + 6498 * xs[1] as usize
                - 6859 * xs[0] as usize;
            if !seen_changes[changes] {
                seen_changes[changes] = true;
                all_sequence_prices[changes] += xs[4] as u16;
            }
        }
    }
    let part2 = all_sequence_prices.into_iter().max().unwrap();

    let time = start.elapsed();
    println!("Part 1: {part1}\nPart 2: {part2}\nTime taken: {time:?}",);

    for i in 0..24 {
        println!(
            "{:024b} == {:024b}: {}",
            BITMASKS[i],
            BITMASKS_2[i],
            BITMASKS[i] == BITMASKS_2[i]
        );
    }
    //bitmasks_2();

    /*let mut out = String::new();

    for bit_i in 0..24 {
        let mask = BITMASKS_2[bit_i];
    }*/
}

const BITMASKS: [u32; 24] = [
    0x61a765, 0xc2f82d, 0x286d53, 0x44f679, 0x4d6be8, 0x118005, 0x5f19f2, 0xf03667, 0xcea653,
    0xafa201, 0xfd0d29, 0x949200, 0x49a994, 0x021673, 0xb4c5bf, 0x1e0aaf, 0x7cab00, 0x95ba48,
    0x49f04c, 0x9a8320, 0xb69d39, 0x6a2085, 0xd13c84, 0x1c9e15,
];
const BITMASKS_2: [u32; 24] = {
    //fn bitmasks_2() {
    let mut n = [0; 24];
    let mut i = 0;
    while i < 24 {
        n[i] = 1 << i;
        i += 1;
    }

    let mut i = 0;
    while i < 2000 {
        n = next_secret_bit_array(n);
        i += 1;
    }

    let mut bitmasks = [0; 24];
    let mut out_bit_i = 0;
    while out_bit_i < 24 {
        /*bitmasks[0] = n[23] & (1 << 23) | n[22] & (1 << 22) | ... | n[0] & (1 << 0);
        bitmasks[1] = n[22] & (1 << 23) | n[21] & (1 << 22) | ... | n[23] & (1 << 0);
        bitmasks[2] = n[21] & (1 << 23) | n[20] & (1 << 22) | ... | n[22] & (1 << 0);*/

        let mut in_bit_i = 0;
        while in_bit_i < 24 {
            bitmasks[out_bit_i] |= n[in_bit_i] & (1 << ((in_bit_i + out_bit_i) % 24));

            in_bit_i += 1;
        }
        out_bit_i += 1;
    }

    //for (i, bitmask) in bitmasks.iter().enumerate() {
    //    println!("{i}: {bitmask:?}");
    //}
    bitmasks
};

fn next_secret(mut n: u32) -> u32 {
    n ^= n << 6;
    n &= 0b1111_1111_1111_1111_1111_1111;
    n ^= n >> 5;
    n ^= n << 11;
    n & 0b1111_1111_1111_1111_1111_1111
}

const fn next_secret_bit_array(mut n: [u32; 24]) -> [u32; 24] {
    let mut i = 0;
    while i < n.len() - 6 {
        n[i] ^= n[i + 6];
        i += 1;
    }

    let mut i = n.len() - 1;
    while i >= 5 {
        n[i] ^= n[i - 5];
        i -= 1;
    }

    let mut i = 0;
    while i < n.len() - 11 {
        n[i] ^= n[i + 11];
        i += 1;
    }

    n
}

const fn two_thousand_secret_new(n: u32) -> u32 {
    let n = n as u64;
    let n = n << 24 | n;
    let mut result = 0;
    let mut right_shift = 0;
    while right_shift < 24 {
        result ^= (n >> right_shift) & BITMASKS[right_shift] as u64;
        right_shift += 1;
    }
    result as u32
}

/*fn next_secret_2(mut n: Vec<u32>) -> Vec<u32> {
    n = n
        .clone()
        .into_iter()
        .skip(6)
        .chain(core::iter::repeat(0))
        .zip(n)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>();

    n = core::iter::repeat(0)
        .take(5)
        .chain(n.clone())
        .zip(n)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>();

    n = n
        .clone()
        .into_iter()
        .skip(11)
        .chain(core::iter::repeat(0))
        .zip(n)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>();
}*/

fn two_thousandth_secret(n: u32) -> u32 {
    (((n >> 0)
        ^ (n >> 6)
        ^ (n >> 7)
        ^ (n >> 8)
        ^ (n >> 9)
        ^ (n >> 10)
        ^ (n >> 13)
        ^ (n >> 16)
        ^ (n >> 18)
        ^ (n >> 19)
        ^ (n >> 21))
        & 1)
        << 23
        | (((n >> 1)
            ^ (n >> 2)
            ^ (n >> 4)
            ^ (n >> 5)
            ^ (n >> 6)
            ^ (n >> 8)
            ^ (n >> 10)
            ^ (n >> 14)
            ^ (n >> 16)
            ^ (n >> 19)
            ^ (n >> 20)
            ^ (n >> 22)
            ^ (n >> 23))
            & 1)
            << 22
        | (((n >> 4)
            ^ (n >> 6)
            ^ (n >> 7)
            ^ (n >> 11)
            ^ (n >> 13)
            ^ (n >> 17)
            ^ (n >> 18)
            ^ (n >> 21)
            ^ (n >> 23))
            & 1)
            << 21
        | (((n >> 1)
            ^ (n >> 2)
            ^ (n >> 3)
            ^ (n >> 6)
            ^ (n >> 7)
            ^ (n >> 10)
            ^ (n >> 11)
            ^ (n >> 12)
            ^ (n >> 13)
            ^ (n >> 15)
            ^ (n >> 16)
            ^ (n >> 18)
            ^ (n >> 19))
            & 1)
            << 20
        | (((n >> 1)
            ^ (n >> 3)
            ^ (n >> 4)
            ^ (n >> 5)
            ^ (n >> 7)
            ^ (n >> 10)
            ^ (n >> 11)
            ^ (n >> 13)
            ^ (n >> 14)
            ^ (n >> 16)
            ^ (n >> 18)
            ^ (n >> 21)
            ^ (n >> 23))
            & 1)
            << 19
        | (((n >> 0)
            ^ (n >> 2)
            ^ (n >> 3)
            ^ (n >> 4)
            ^ (n >> 5)
            ^ (n >> 8)
            ^ (n >> 9)
            ^ (n >> 10)
            ^ (n >> 11)
            ^ (n >> 14)
            ^ (n >> 17)
            ^ (n >> 21)
            ^ (n >> 22))
            & 1)
            << 18
        | (((n >> 1)
            ^ (n >> 2)
            ^ (n >> 6)
            ^ (n >> 8)
            ^ (n >> 12)
            ^ (n >> 13)
            ^ (n >> 14)
            ^ (n >> 18)
            ^ (n >> 23))
            & 1)
            << 17
        | (((n >> 1)
            ^ (n >> 2)
            ^ (n >> 4)
            ^ (n >> 9)
            ^ (n >> 10)
            ^ (n >> 14)
            ^ (n >> 16)
            ^ (n >> 20)
            ^ (n >> 21)
            ^ (n >> 22))
            & 1)
            << 16
        | (((n >> 0)
            ^ (n >> 2)
            ^ (n >> 3)
            ^ (n >> 5)
            ^ (n >> 7)
            ^ (n >> 8)
            ^ (n >> 9)
            ^ (n >> 10)
            ^ (n >> 11)
            ^ (n >> 14)
            ^ (n >> 15)
            ^ (n >> 16)
            ^ (n >> 18)
            ^ (n >> 20)
            ^ (n >> 23))
            & 1)
            << 15
        | (((n >> 4) ^ (n >> 8) ^ (n >> 15) ^ (n >> 16) ^ (n >> 17) ^ (n >> 18)) & 1) << 14
        | (((n >> 1)
            ^ (n >> 5)
            ^ (n >> 6)
            ^ (n >> 7)
            ^ (n >> 10)
            ^ (n >> 11)
            ^ (n >> 13)
            ^ (n >> 14)
            ^ (n >> 15)
            ^ (n >> 16)
            ^ (n >> 17)
            ^ (n >> 20)
            ^ (n >> 21)
            ^ (n >> 22))
            & 1)
            << 13
        | (((n >> 1)
            ^ (n >> 5)
            ^ (n >> 6)
            ^ (n >> 8)
            ^ (n >> 10)
            ^ (n >> 11)
            ^ (n >> 13)
            ^ (n >> 15)
            ^ (n >> 18)
            ^ (n >> 19)
            ^ (n >> 23))
            & 1)
            << 12
        | (((n >> 2)
            ^ (n >> 3)
            ^ (n >> 4)
            ^ (n >> 7)
            ^ (n >> 9)
            ^ (n >> 10)
            ^ (n >> 12)
            ^ (n >> 13)
            ^ (n >> 15)
            ^ (n >> 17)
            ^ (n >> 21)
            ^ (n >> 23))
            & 1)
            << 11
        | (((n >> 0)
            ^ (n >> 6)
            ^ (n >> 8)
            ^ (n >> 9)
            ^ (n >> 10)
            ^ (n >> 12)
            ^ (n >> 13)
            ^ (n >> 17)
            ^ (n >> 18)
            ^ (n >> 20)
            ^ (n >> 23))
            & 1)
            << 10
        | (((n >> 0)
            ^ (n >> 1)
            ^ (n >> 2)
            ^ (n >> 4)
            ^ (n >> 8)
            ^ (n >> 9)
            ^ (n >> 12)
            ^ (n >> 13)
            ^ (n >> 16)
            ^ (n >> 17)
            ^ (n >> 18)
            ^ (n >> 20)
            ^ (n >> 22))
            & 1)
            << 9
        | (((n >> 0)
            ^ (n >> 3)
            ^ (n >> 4)
            ^ (n >> 8)
            ^ (n >> 10)
            ^ (n >> 12)
            ^ (n >> 14)
            ^ (n >> 18)
            ^ (n >> 20)
            ^ (n >> 22))
            & 1)
            << 8
        | (((n >> 4) ^ (n >> 5) ^ (n >> 11) ^ (n >> 13) ^ (n >> 19) ^ (n >> 21) ^ (n >> 22)) & 1)
            << 7
        | (((n >> 0)
            ^ (n >> 6)
            ^ (n >> 8)
            ^ (n >> 9)
            ^ (n >> 10)
            ^ (n >> 12)
            ^ (n >> 13)
            ^ (n >> 14)
            ^ (n >> 19)
            ^ (n >> 23))
            & 1)
            << 6
        | (((n >> 0)
            ^ (n >> 1)
            ^ (n >> 5)
            ^ (n >> 6)
            ^ (n >> 8)
            ^ (n >> 9)
            ^ (n >> 11)
            ^ (n >> 12)
            ^ (n >> 15)
            ^ (n >> 18)
            ^ (n >> 19)
            ^ (n >> 20))
            & 1)
            << 5
        | (((n >> 0)
            ^ (n >> 3)
            ^ (n >> 6)
            ^ (n >> 7)
            ^ (n >> 10)
            ^ (n >> 12)
            ^ (n >> 16)
            ^ (n >> 17)
            ^ (n >> 18))
            & 1)
            << 4
        | (((n >> 4)
            ^ (n >> 6)
            ^ (n >> 7)
            ^ (n >> 13)
            ^ (n >> 17)
            ^ (n >> 18)
            ^ (n >> 20)
            ^ (n >> 21)
            ^ (n >> 23))
            & 1)
            << 3
        | (((n >> 0)
            ^ (n >> 1)
            ^ (n >> 2)
            ^ (n >> 3)
            ^ (n >> 7)
            ^ (n >> 9)
            ^ (n >> 14)
            ^ (n >> 16)
            ^ (n >> 17)
            ^ (n >> 20)
            ^ (n >> 23))
            & 1)
            << 2
        | (((n >> 3) ^ (n >> 7) ^ (n >> 8) ^ (n >> 9) ^ (n >> 14) ^ (n >> 15) ^ (n >> 16)) & 1) << 1
        | (((n >> 0)
            ^ (n >> 1)
            ^ (n >> 2)
            ^ (n >> 3)
            ^ (n >> 5)
            ^ (n >> 7)
            ^ (n >> 8)
            ^ (n >> 9)
            ^ (n >> 10)
            ^ (n >> 13)
            ^ (n >> 14)
            ^ (n >> 15)
            ^ (n >> 20)
            ^ (n >> 21)
            ^ (n >> 23))
            & 1)
}
