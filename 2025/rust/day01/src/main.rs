// Improved with the assistance of Godbolt.org and Gemini 3.1 Pro.
// Simplified direction change handling, inspired by /u/Turilas at
//     https://www.reddit.com/r/adventofcode/comments/1pb3y8p/_/nrrk1qf/.

fn main() {
    let input = include_bytes!("../../../day01.txt");

    let mut pos = 50;
    let mut last_direction = b'L';
    let mut part_1: u32 = 0;
    let mut part_2 = 0;

    let mut i = 0;
    while i < input.len() {
        if input[i] != last_direction {
            last_direction = input[i];
            if pos != 0 {
                pos = 100 - pos;
            }
        }
        i += 1;
    
        let mut movement = 0;
        while i < input.len() && input[i].wrapping_sub(b'0') <= 9 {
            movement *= 10;
            movement += (input[i] - b'0') as u32;
            i += 1;
        }
        pos += movement;
        part_2 += pos / 100;
        pos %= 100;
        if pos == 0 {
            part_1 += 1;
        }

        while i < input.len() && input[i] < b' ' {
            i += 1;
        }
    }
    
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
