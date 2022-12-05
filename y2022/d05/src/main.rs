use std::{fs::File, io::Read, time::Instant};

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}

fn parse_initial(initial: &str) -> Vec<Stack> {
    let mut crates = Vec::new();

    let mut lines = initial.lines().rev().map(|l| l.chars());
    let last = lines.next().unwrap();
    let mut lines: Vec<_> = lines.collect();

    let mut last_idx = 0;
    for (idx, crate_id) in last.enumerate() {
        if crate_id == ' ' {
            continue;
        }

        crates.push(Stack {
            crates: lines
                .iter_mut()
                .filter_map(|l| {
                    l.nth(idx - last_idx)
                        .and_then(|c| if c == ' ' { None } else { Some(c) })
                })
                .collect(),
        });

        last_idx = idx + 1;
    }

    crates
}

fn move_crates_p1(stacks: &mut [Stack], count: usize, from: usize, to: usize) {
    if count == 1 {
        let to_push = stacks[from].crates.pop().unwrap();
        stacks[to].crates.push(to_push);
    } else {
        for _ in 0..count {
            move_crates_p1(stacks, 1, from, to);
        }
    }
}

fn move_crates_p2(stacks: &mut [Stack], count: usize, from: usize, to: usize) {
    if count == 1 {
        move_crates_p1(stacks, count, from, to);
    } else {
        let mut to_push = Vec::new();
        for _ in 0..count {
            to_push.push(stacks[from].crates.pop().unwrap());
        }
        to_push.reverse();

        stacks[to].crates.extend(to_push);
    }
}

fn main() {
    let start = Instant::now();

    let mut content = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (initial, instructions) = content.split_once("\n\n").unwrap();
    let mut stacks_p1 = parse_initial(initial);
    let mut stacks_p2 = stacks_p1.clone();

    let re = regex::Regex::new(r#"^move (\d+) from (\d+) to (\d+)"#).unwrap();

    for line in instructions.lines() {
        let groups = re.captures_iter(line).next().unwrap();

        let count = groups.get(1).unwrap().as_str().parse().unwrap();

        let from: usize = groups.get(2).unwrap().as_str().parse().unwrap();
        let from = from - 1;

        let to: usize = groups.get(3).unwrap().as_str().parse().unwrap();
        let to = to - 1;

        move_crates_p1(&mut stacks_p1, count, from, to);
        move_crates_p2(&mut stacks_p2, count, from, to);
    }

    print!("Part 1: ");
    for s in stacks_p1 {
        let Some(last) = s.crates.last() else { continue; };
        print!("{last}");
    }
    println!();

    print!("Part 2: ");
    for s in stacks_p2 {
        let Some(last) = s.crates.last() else { continue; };
        print!("{last}");
    }
    println!();

    println!("Elapsed: {:?}", start.elapsed());
}
