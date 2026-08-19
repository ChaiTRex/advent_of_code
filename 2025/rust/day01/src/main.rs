// Improved with the assistance of Godbolt.org and Gemini 3.1 Pro.

fn main() {
    let input = include_bytes!("../../../day01.txt");

    let mut pos = 50;
    let mut leftward_goes_up = false;
    let mut part_1: u32 = 0;
    let mut part_2 = 0;

    let mut i = 0;
    while i < input.len() {
        if (input[i] == b'L') ^ leftward_goes_up {
            leftward_goes_up = !leftward_goes_up;
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
