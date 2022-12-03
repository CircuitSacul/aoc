use std::{collections::HashSet, fs::File, io::Read, time::SystemTime};

pub struct Compartment {
    chars: HashSet<char>,
}

impl From<&str> for Compartment {
    fn from(line: &str) -> Self {
        Self {
            chars: line.chars().collect(),
        }
    }
}

pub struct RuckSack(Compartment, Compartment);

impl RuckSack {
    fn shared(&self) -> char {
        *self.0.chars.intersection(&self.1.chars).next().unwrap()
    }

    fn joined(mut self) -> HashSet<char> {
        self.0.chars.extend(self.1.chars);
        self.0.chars
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
    fn shared(self) -> char {
        let mut intersection = self.0.joined();
        let one = self.1.joined();
        let two = self.2.joined();
        intersection.retain(|e| one.contains(e) && two.contains(e));
        intersection.into_iter().next().unwrap()
    }
}

fn get_char_val(chr: char) -> u8 {
    if chr.is_lowercase() {
        chr as u8 - (b'a' - 1)
    } else {
        chr as u8 - (b'A' - 27)
    }
}

fn main() {
    let start = SystemTime::now();

    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut total_p1: u32 = 0;

    let mut rucksacks = (None, None, None);
    let mut total_p2: u32 = 0;

    for (idx, line) in content.lines().enumerate() {
        let rucksack = RuckSack::from(line);
        total_p1 += get_char_val(rucksack.shared()) as u32;

        match idx % 3 {
            0 => rucksacks.0 = Some(rucksack),
            1 => rucksacks.1 = Some(rucksack),
            2 => {
                rucksacks.2 = Some(rucksack);

                let chr = Group(
                    std::mem::take(&mut rucksacks.0).unwrap(),
                    std::mem::take(&mut rucksacks.1).unwrap(),
                    std::mem::take(&mut rucksacks.2).unwrap(),
                )
                .shared();
                total_p2 += get_char_val(chr) as u32;
            }
            _ => unreachable!(),
        }
    }

    println!("Part 1: {total_p1}");
    println!("Part 2: {total_p2}");

    println!("Elapsed: {:?}", start.elapsed());
}
