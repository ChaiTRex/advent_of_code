use std::collections::{HashMap, HashSet};

fn main() {
    let mut paths = HashMap::new();

    for start in [
        NumericKey::Number(0),
        NumericKey::Number(1),
        NumericKey::Number(2),
        NumericKey::Number(3),
        NumericKey::Number(4),
        NumericKey::Number(5),
        NumericKey::Number(6),
        NumericKey::Number(7),
        NumericKey::Number(8),
        NumericKey::Number(9),
        NumericKey::Activate,
    ] {
        paths.insert((start, start), vec![DirectionalKey::Activate]);
        let mut states_achieved = HashSet::new();

        let mut states = vec![(Vec::new(), State {
            pad3: start,
            ..State::default()
        })];
        while paths.len() % 11 != 0 {
            let iter = states.into_iter();
            states = Vec::new();
            for (past_buttons, state) in iter {
                if states_achieved.contains(&state) {
                    continue;
                }
                for direction in [
                    DirectionalKey::Up,
                    DirectionalKey::Right,
                    DirectionalKey::Down,
                    DirectionalKey::Left,
                ] {
                    if let Some((next_state, _)) = state.step(direction) {
                        let mut buttons = past_buttons.clone();
                        buttons.push(direction);
                        states.push((buttons, next_state));
                    }
                }
                match state.step(DirectionalKey::Activate) {
                    Some((_, Some(button_pressed))) => {
                        let mut buttons = past_buttons.clone();
                        buttons.push(DirectionalKey::Activate);
                        paths.entry((start, button_pressed)).or_insert(buttons);
                    }
                    Some((next_state, None)) => {
                        let mut buttons = past_buttons.clone();
                        buttons.push(DirectionalKey::Activate);
                        states.push((buttons, next_state));
                    }
                    _ => (),
                }
                states_achieved.insert(state);
            }
        }
    }

    let mut part1 = 0;
    let mut i = 0;
    for _ in 0..HEIGHT {
        let a = INPUT[i] - b'0';
        let b = INPUT[i + 1] - b'0';
        let c = INPUT[i + 2] - b'0';

        let path_len = paths
            .get(&(NumericKey::Activate, NumericKey::Number(a)))
            .unwrap()
            .len() as u64
            + paths
                .get(&(NumericKey::Number(a), NumericKey::Number(b)))
                .unwrap()
                .len() as u64
            + paths
                .get(&(NumericKey::Number(b), NumericKey::Number(c)))
                .unwrap()
                .len() as u64
            + paths
                .get(&(NumericKey::Number(c), NumericKey::Activate))
                .unwrap()
                .len() as u64;
        let number = 100 * a as u64 + 10 * b as u64 + c as u64;

        part1 += path_len * number;

        i += LINE_WIDTH;
    }
    println!("{part1}");

    let mut numeric_paths = HashMap::new();

    for start in [
        NumericKey::Number(0),
        NumericKey::Number(1),
        NumericKey::Number(2),
        NumericKey::Number(3),
        NumericKey::Number(4),
        NumericKey::Number(5),
        NumericKey::Number(6),
        NumericKey::Number(7),
        NumericKey::Number(8),
        NumericKey::Number(9),
        NumericKey::Activate,
    ] {
        for end in [
            NumericKey::Number(0),
            NumericKey::Number(1),
            NumericKey::Number(2),
            NumericKey::Number(3),
            NumericKey::Number(4),
            NumericKey::Number(5),
            NumericKey::Number(6),
            NumericKey::Number(7),
            NumericKey::Number(8),
            NumericKey::Number(9),
            NumericKey::Activate,
        ] {
            let mut solutions = vec![(start, vec![])];

            while !solutions.iter().any(|&(location, _)| location == end) {
                let mut iter = solutions.into_iter();
                solutions = Vec::new();

                for (location, path) in iter {
                    for (step, direction) in [
                        NumericKey::up,
                        NumericKey::right,
                        NumericKey::down,
                        NumericKey::left,
                    ]
                    .into_iter()
                    .zip([
                        DirectionalKey::Up,
                        DirectionalKey::Right,
                        DirectionalKey::Down,
                        DirectionalKey::Left,
                    ]) {
                        if let Some(new_location) = step(location) {
                            let mut path = path.clone();
                            path.push(direction);
                            solutions.push((new_location, path));
                        }
                    }
                }
            }

            let solutions = solutions
                .into_iter()
                .filter(|&(location, _)| location == end)
                .map(|(_, mut path)| {
                    path.push(DirectionalKey::Activate);
                    path
                })
                .collect::<Vec<_>>();

            numeric_paths.insert((start, end), solutions);
        }
    }

    let mut directional_paths = HashMap::new();

    for start in [
        DirectionalKey::Up,
        DirectionalKey::Right,
        DirectionalKey::Down,
        DirectionalKey::Left,
        DirectionalKey::Activate,
    ] {
        for end in [
            DirectionalKey::Up,
            DirectionalKey::Right,
            DirectionalKey::Down,
            DirectionalKey::Left,
            DirectionalKey::Activate,
        ] {
            let mut solutions = vec![(start, vec![])];

            while !solutions.iter().any(|&(location, _)| location == end) {
                let mut iter = solutions.into_iter();
                solutions = Vec::new();

                for (location, path) in iter {
                    for (step, direction) in [
                        DirectionalKey::up,
                        DirectionalKey::right,
                        DirectionalKey::down,
                        DirectionalKey::left,
                    ]
                    .into_iter()
                    .zip([
                        DirectionalKey::Up,
                        DirectionalKey::Right,
                        DirectionalKey::Down,
                        DirectionalKey::Left,
                    ]) {
                        if let Some(new_location) = step(location) {
                            let mut path = path.clone();
                            path.push(direction);
                            solutions.push((new_location, path));
                        }
                    }
                }
            }

            let mut solutions = solutions
                .into_iter()
                .filter(|&(location, _)| location == end)
                .flat_map(|(_, mut path)| {
                    path.push(DirectionalKey::Activate);
                    path
                })
                .collect::<Vec<_>>();
            solutions.sort();

            directional_paths.insert((start, end), solutions);
        }
    }
    for (ends, solutions) in &directional_paths {
        println!("{ends:?}: {solutions:?}");
    }

    /*let mut xs: Vec<Vec<DirectionalKey>> = directional_paths.values().cloned().collect::<Vec<_>>();
    xs.sort_unstable();
    xs.dedup();
    println!("{}, {xs:?}", xs.len());*/

    let mut part2 = 0;
    let mut i = 0;
    for _ in 0..HEIGHT {
        let a = INPUT[i] - b'0';
        let b = INPUT[i + 1] - b'0';
        let c = INPUT[i + 2] - b'0';

        let number = 100 * a as u64 + 10 * b as u64 + c as u64;

        //part1 += path_len * number;

        i += LINE_WIDTH;
    }
    println!("{part2}");
}

static INPUT: &[u8] = include_bytes!("../../../day21.txt");

const WIDTH: usize = 4;
const LINE_WIDTH: usize = {
    let mut i = WIDTH;
    while !INPUT[i].is_ascii_alphanumeric() {
        i += 1;
    }
    i
};
const HEIGHT: usize = INPUT.len() / LINE_WIDTH;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State {
    pad1: DirectionalKey,
    pad2: DirectionalKey,
    pad3: NumericKey,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State2 {
    dirs: [DirectionalKey; 25],
    nums: NumericKey,
}

impl State {
    pub fn step(mut self, input: DirectionalKey) -> Option<(Self, Option<NumericKey>)> {
        let mut key3_pressed = false;
        match input {
            DirectionalKey::Up => {
                self.pad1 = self.pad1.up()?;
            }
            DirectionalKey::Right => {
                self.pad1 = self.pad1.right()?;
            }
            DirectionalKey::Down => {
                self.pad1 = self.pad1.down()?;
            }
            DirectionalKey::Left => {
                self.pad1 = self.pad1.left()?;
            }
            DirectionalKey::Activate => match self.pad1 {
                DirectionalKey::Up => {
                    self.pad2 = self.pad2.up()?;
                }
                DirectionalKey::Right => {
                    self.pad2 = self.pad2.right()?;
                }
                DirectionalKey::Down => {
                    self.pad2 = self.pad2.down()?;
                }
                DirectionalKey::Left => {
                    self.pad2 = self.pad2.left()?;
                }
                DirectionalKey::Activate => match self.pad2 {
                    DirectionalKey::Up => {
                        self.pad3 = self.pad3.up()?;
                    }
                    DirectionalKey::Right => {
                        self.pad3 = self.pad3.right()?;
                    }
                    DirectionalKey::Down => {
                        self.pad3 = self.pad3.down()?;
                    }
                    DirectionalKey::Left => {
                        self.pad3 = self.pad3.left()?;
                    }
                    DirectionalKey::Activate => {
                        key3_pressed = true;
                    }
                },
            },
        }

        Some((self, key3_pressed.then_some(self.pad3)))
    }
}

impl State2 {
    pub fn step(mut self, i: usize, input: DirectionalKey) -> Option<(Self, Option<NumericKey>)> {
        let mut key3_pressed = false;
        match input {
            DirectionalKey::Up => {
                self.dirs[i] = self.dirs[i].up()?;
            }
            DirectionalKey::Right => {
                self.dirs[i] = self.dirs[i].right()?;
            }
            DirectionalKey::Down => {
                self.dirs[i] = self.dirs[i].down()?;
            }
            DirectionalKey::Left => {
                self.dirs[i] = self.dirs[i].left()?;
            }
            DirectionalKey::Activate => {
                if i < self.dirs.len() - 1 {
                    return self.step(i + 1, self.dirs[i]);
                } else {
                    match self.dirs[i] {
                        DirectionalKey::Up => {
                            self.nums = self.nums.up()?;
                        }
                        DirectionalKey::Right => {
                            self.nums = self.nums.right()?;
                        }
                        DirectionalKey::Down => {
                            self.nums = self.nums.down()?;
                        }
                        DirectionalKey::Left => {
                            self.nums = self.nums.left()?;
                        }
                        DirectionalKey::Activate => {
                            key3_pressed = true;
                        }
                    }
                }
            }
        }

        Some((self, key3_pressed.then_some(self.nums)))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NumericKey {
    Number(u8),
    #[default]
    Activate,
}

impl NumericKey {
    pub fn up(self) -> Option<Self> {
        match self {
            Self::Number(0) => Some(Self::Number(2)),
            Self::Number(1) => Some(Self::Number(4)),
            Self::Number(2) => Some(Self::Number(5)),
            Self::Number(3) => Some(Self::Number(6)),
            Self::Number(4) => Some(Self::Number(7)),
            Self::Number(5) => Some(Self::Number(8)),
            Self::Number(6) => Some(Self::Number(9)),
            Self::Activate => Some(Self::Number(3)),
            _ => None,
        }
    }

    pub fn right(self) -> Option<Self> {
        match self {
            Self::Number(0) => Some(Self::Activate),
            Self::Number(1) => Some(Self::Number(2)),
            Self::Number(2) => Some(Self::Number(3)),
            Self::Number(4) => Some(Self::Number(5)),
            Self::Number(5) => Some(Self::Number(6)),
            Self::Number(7) => Some(Self::Number(8)),
            Self::Number(8) => Some(Self::Number(9)),
            _ => None,
        }
    }

    pub fn down(self) -> Option<Self> {
        match self {
            Self::Number(2) => Some(Self::Number(0)),
            Self::Number(3) => Some(Self::Activate),
            Self::Number(4) => Some(Self::Number(1)),
            Self::Number(5) => Some(Self::Number(2)),
            Self::Number(6) => Some(Self::Number(3)),
            Self::Number(7) => Some(Self::Number(4)),
            Self::Number(8) => Some(Self::Number(5)),
            Self::Number(9) => Some(Self::Number(6)),
            _ => None,
        }
    }

    pub fn left(self) -> Option<Self> {
        match self {
            Self::Number(2) => Some(Self::Number(1)),
            Self::Number(3) => Some(Self::Number(2)),
            Self::Number(5) => Some(Self::Number(4)),
            Self::Number(6) => Some(Self::Number(5)),
            Self::Number(8) => Some(Self::Number(7)),
            Self::Number(9) => Some(Self::Number(8)),
            Self::Activate => Some(Self::Number(0)),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DirectionalKey {
    Up,
    Right,
    Down,
    Left,
    #[default]
    Activate,
}

impl DirectionalKey {
    pub fn up(self) -> Option<Self> {
        match self {
            Self::Right => Some(Self::Activate),
            Self::Down => Some(Self::Up),
            _ => None,
        }
    }

    pub fn right(self) -> Option<Self> {
        match self {
            Self::Up => Some(Self::Activate),
            Self::Down => Some(Self::Right),
            Self::Left => Some(Self::Down),
            _ => None,
        }
    }

    pub fn down(self) -> Option<Self> {
        match self {
            Self::Up => Some(Self::Down),
            Self::Activate => Some(Self::Right),
            _ => None,
        }
    }

    pub fn left(self) -> Option<Self> {
        match self {
            Self::Right => Some(Self::Down),
            Self::Down => Some(Self::Left),
            Self::Activate => Some(Self::Up),
            _ => None,
        }
    }
}
