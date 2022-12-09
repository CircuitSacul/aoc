use std::{collections::HashSet, fs, io::Read, time::Instant};

fn scan_line(
    line: impl Iterator<Item = char>,
    rev: impl Iterator<Item = char>,
    last_idx: usize,
    other_axis: usize,
    rotated: bool,
    into: &mut HashSet<(usize, usize)>,
) {
    let mut vl_h = '0';
    for (idx, h) in line.enumerate() {
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
    for (idx, h) in rev.enumerate() {
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
    let mut lines = Vec::new();
    let mut last = 0;
    for (idx, line) in content.lines().enumerate() {
        last = line.len() - 1;
        scan_line(
            line.chars(),
            line.chars().rev(),
            last,
            idx,
            false,
            &mut visible,
        );
        lines.push(line.chars().into_iter());
    }

    // scan top-to-bottom
    for idx in 0..=last {
        let line: Vec<_> = lines.iter_mut().map(|l| l.next().unwrap()).collect();
        scan_line(
            line.iter().copied(),
            line.iter().copied().rev(),
            line.len() - 1,
            idx,
            true,
            &mut visible,
        );
    }

    println!("Part 1: {}", visible.len());

    println!("Elapsed: {:?}", start.elapsed());
}
