fn main() {
    static INPUT: &[u8] = include_bytes!("../../../day09.txt");

    let mut disk = Vec::new();
    let mut i = 0;
    let mut file = 0_i16;
    loop {
        if INPUT[i] == b'\n' {
            break;
        }

        let count = (INPUT[i] - b'0') as usize;
        for _ in 0..count {
            disk.push(file);
        }
        file += 1;
        i += 1;

        if INPUT[i] == b'\n' {
            break;
        }

        let count = (INPUT[i] - b'0') as usize;
        for _ in 0..count {
            disk.push(-1);
        }
        i += 1;
    }

    let mut i = 0;
    let mut j = disk.len() - 1;
    loop {
        while disk[i] != -1 {
            i += 1;
        }
        if i >= j {
            break;
        }
        disk.swap(i, j);
        j -= 1;
    }

    let part1 = disk
        .iter()
        .take_while(|&&file| file != -1)
        .map(|&file| file as u64)
        .enumerate()
        .map(|(i, file)| i as u64 * file)
        .sum::<u64>();

    println!("{part1}");

    let mut disk = Vec::new();
    let mut i = 0;
    let mut file = 0_i16;
    loop {
        if INPUT[i] == b'\n' {
            break;
        }

        disk.push((file, INPUT[i] - b'0'));
        file += 1;
        i += 1;

        if INPUT[i] == b'\n' {
            break;
        }

        disk.push((-1, INPUT[i] - b'0'));
        i += 1;
    }

    let mut j = disk.len() - 1;
    'outer: loop {
        while disk[j].0 == -1 {
            if j == 0 {
                break 'outer;
            }
            j -= 1
        }
        let size = disk[j].1;

        match disk
            .iter()
            .take(j)
            .position(|&(file, size_b)| file == -1 && size_b >= size)
        {
            None => (),
            Some(i) => {
                disk[i].1 -= size;

                if disk[i].1 == 0 {
                    disk[i] = disk[j];
                } else {
                    disk.insert(i, disk[j]);
                    j += 1;
                }

                disk[j] = (-1, size);
            }
        }

        if j == 0 {
            break;
        }
        j -= 1;
    }

    let part2 = disk
        .iter()
        .flat_map(|(file, size)| vec![file; *size as usize])
        .enumerate()
        .filter(|&(_, file)| *file != -1)
        .map(|(i, file)| i as u64 * *file as u64)
        .sum::<u64>();
    println!("{part2}");
}
