use std::{fs::File, io::Read, ops::RangeInclusive, time::Instant};

pub struct TaskRange {
    left: i32,
    right: i32,
    range: RangeInclusive<i32>,
}

fn get_range(elf: &str) -> TaskRange {
    let (left, right) = elf.split_once('-').unwrap();
    let (left, right) = (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap());

    TaskRange {
        left,
        right,
        range: left..=right,
    }
}

fn one_contains_other(left: &TaskRange, right: &TaskRange) -> bool {
    if left.range.contains(&right.left) && left.range.contains(&right.right) {
        true
    } else {
        right.range.contains(&left.left) && right.range.contains(&left.right)
    }
}

fn overlaps(left: &TaskRange, right: &TaskRange) -> bool {
    left.range.contains(&right.left)
        || left.range.contains(&right.right)
        || right.range.contains(&left.left)
        || right.range.contains(&left.right)
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

        if overlaps(&left, &right) {
            count_2 += 1;

            if one_contains_other(&left, &right) {
                count_1 += 1;
            }
        }
    }

    println!("Part 1: {count_1}");
    println!("Part 2: {count_2}");

    println!("Elapsed: {:?}", start.elapsed());
}
