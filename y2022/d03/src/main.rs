use std::{fs::File, io::Read, time::SystemTime};

pub struct Compartment {
    chars: u64,
}

impl From<&str> for Compartment {
    fn from(line: &str) -> Self {
        let mut chars: u64 = 0;
        for chr in line.chars().map(get_char_val) {
            chars |= 1 << chr;
        }
        Self { chars }
    }
}

pub struct RuckSack(Compartment, Compartment);

impl RuckSack {
    fn shared(&self) -> u64 {
        self.0.chars & self.1.chars
    }

    fn joined(&self) -> u64 {
        self.0.chars | self.1.chars
    }
}

impl From<&str> for RuckSack {
    fn from(line: &str) -> Self {
        let (left, right) = line.split_at(line.len() / 2);
        Self(left.into(), right.into())
    }
}

pub struct Group(RuckSack, RuckSack, RuckSack);

impl Group {
    fn shared(self) -> u64 {
        let first = self.0.joined();
        let second = self.1.joined();
        let third = self.2.joined();

        first & second & third
    }
}

#[inline(always)]
fn get_char_val(chr: char) -> i8 {
    (chr as i8 - 96).rem_euclid(58)
}

#[inline(always)]
fn get_set_item(set: u64) -> u8 {
    set.trailing_zeros() as u8
}

fn main() {
    let start = SystemTime::now();

    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut total_p1: u32 = 0;

    let mut rucksacks = (None, None);
    let mut total_p2: u32 = 0;

    for (idx, line) in content.lines().enumerate() {
        let rucksack = RuckSack::from(line);
        dbg!(line);
        total_p1 += get_set_item(rucksack.shared()) as u32;

        match idx % 3 {
            0 => rucksacks.0 = Some(rucksack),
            1 => rucksacks.1 = Some(rucksack),
            2 => {
                let set = Group(
                    std::mem::take(&mut rucksacks.0).unwrap(),
                    std::mem::take(&mut rucksacks.1).unwrap(),
                    rucksack,
                )
                .shared();
                total_p2 += get_set_item(set) as u32;
            }
            _ => unreachable!(),
        }
    }

    println!("Part 1: {total_p1}");
    println!("Part 2: {total_p2}");

    println!("Elapsed: {:?}", start.elapsed());
}
