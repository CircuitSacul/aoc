use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
    time::Instant,
};

fn char_u32(c: char) -> u32 {
    c as u32 - 48
}

fn scan_line(line: impl Iterator<Item = u32>) -> Vec<usize> {
    let mut vl: Vec<usize> = Vec::new();
    let mut vr: HashMap<u32, Vec<usize>> = HashMap::new();

    let mut vl_h: u32 = 0;
    let mut last_h: u32 = 0;
    for (idx, h) in line.enumerate() {
        if h > vl_h || idx == 0 {
            vl_h = h;
            vl.push(idx);
        }

        if h >= last_h {
            for h in last_h..=h {
                vr.remove(&h);
            }
        }
        last_h = h;

        if let Some(v) = vr.get_mut(&h) {
            v.push(idx);
        } else {
            vr.insert(h, vec![idx]);
        }
    }

    for v in vr.into_values() {
        vl.extend(v);
    }
    vl
}

pub fn main() {
    let start = Instant::now();

    let mut file = fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let lines = content
        .lines()
        .map(|l| l.chars().map(char_u32).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // scan left-to-right
    for (idx, line) in lines.iter().enumerate() {
        visible.extend(
            scan_line(line.iter().copied())
                .into_iter()
                .map(|v| (v, idx)),
        );
    }

    // scan top-to-bottom
    let mut rotated_lines: Vec<Vec<u32>> = Vec::new();
    for (idx, h) in lines[0].iter().enumerate() {
        rotated_lines.insert(idx, vec![*h]);
    }
    for line in lines[1..].iter() {
        for (idx, h) in line.iter().enumerate() {
            rotated_lines[idx].push(*h);
        }
    }

    for (idx, line) in rotated_lines.iter().enumerate() {
        visible.extend(
            scan_line(line.iter().copied())
                .into_iter()
                .map(|v| (idx, v)),
        );
    }

    println!("Part 1: {}", visible.len());

    println!("Elapsed: {:?}", start.elapsed());
}
