use std::{collections::HashSet, fs, io::Read, time::Instant};

fn scan_line(
    line: impl Iterator<Item = char>,
    last_idx: usize,
    other_axis: usize,
    rotated: bool,
    into: &mut HashSet<(usize, usize)>,
) {
    let mut rev = Vec::with_capacity(last_idx + 1);

    let mut vl_h = '0';
    for (idx, h) in line.enumerate() {
        rev.push(h);
        if h > vl_h || idx == 0 {
            vl_h = h;
            let pos = match rotated {
                true => (other_axis, idx),
                false => (idx, other_axis),
            };
            into.insert(pos);
        }
    }

    vl_h = '0';
    for (idx, h) in rev.into_iter().rev().enumerate() {
        if h > vl_h || idx == 0 {
            let idx = last_idx - idx;
            vl_h = h;
            let pos = match rotated {
                true => (other_axis, idx),
                false => (idx, other_axis),
            };
            into.insert(pos);
        }
    }
}

pub fn main() {
    let start = Instant::now();

    let mut file = fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // scan left-to-right
    let (last_x, last_y) = {
        let mut lines = content.lines();
        (lines.next().unwrap().len() - 1, lines.count())
    };
    let mut lines = Vec::with_capacity(last_y + 1);
    for (idx, line) in content.lines().enumerate() {
        scan_line(line.chars(), last_x, idx, false, &mut visible);
        lines.push(line.chars().into_iter());
    }

    // scan top-to-bottom
    for idx in 0..=last_x {
        let line = lines.iter_mut().map(|l| l.next().unwrap());
        scan_line(line, last_y, idx, true, &mut visible);
    }

    println!("Part 1: {}", visible.len());

    println!("Elapsed: {:?}", start.elapsed());
}
