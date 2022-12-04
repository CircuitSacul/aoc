use std::{fs::File, io::Read, time::Instant};

pub struct TaskRange(i32, i32);

impl TaskRange {
    #[inline]
    fn contains(&self, other: i32) -> bool {
        self.0 <= other && self.1 >= other
    }
}

fn get_range(elf: &str) -> TaskRange {
    let (left, right) = elf.split_once('-').unwrap();
    let (left, right) = (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap());

    TaskRange(left, right)
}

#[inline]
fn overlaps(left: TaskRange, right: TaskRange) -> (bool, bool) {
    let l0 = left.contains(right.0);
    let l1 = left.contains(right.1);
    let r0 = right.contains(left.0);
    let r1 = right.contains(left.1);

    (l0 || r0, (l0 && l1) || (r0 && r1))
}

fn main() {
    let start = Instant::now();

    let mut content = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut content).unwrap();

    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in content.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let (left, right) = (get_range(left), get_range(right));

        let (overlaps, contains) = overlaps(left, right);
        if overlaps {
            count_2 += 1;

            if contains {
                count_1 += 1;
            }
        }
    }

    println!("Part 1: {count_1}");
    println!("Part 2: {count_2}");

    println!("Elapsed: {:?}", start.elapsed());
}
