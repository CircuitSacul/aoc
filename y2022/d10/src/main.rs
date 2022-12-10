use std::{fs, io::Read, time::Instant, fmt::Write};

#[inline(always)]
fn maybe_add(cycle: i32, strength_sum: &mut i32, screen: &mut String, x: i32) {
    if (cycle - 20) % 40 == 0 {
        *strength_sum += x * cycle;
    }

    let cycle_mod = cycle % 40;
    if cycle_mod.abs_diff(x) < 2 {
        write!(screen, "#").unwrap();
    } else {
        write!(screen, " ").unwrap();
    }

    if cycle_mod == 39 {
        writeln!(screen).unwrap();
    }
}

fn main() {
    let start = Instant::now();

    let mut file = fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut cycle = 0;
    let mut x = 1;

    let mut strength_sum = 0;
    let mut screen = String::new();

    for line in content.lines() {
        if let Some(count) = line.strip_prefix("addx ") {
            maybe_add(cycle, &mut strength_sum, &mut screen, x);
            cycle += 1;
            maybe_add(cycle, &mut strength_sum, &mut screen, x);
            x += count.parse::<i32>().unwrap();
        } else {
            maybe_add(cycle, &mut strength_sum, &mut screen, x);
        }
        cycle += 1;
    }

    println!("Part 1: {strength_sum}");
    println!("{screen}");

    println!("Elapsed: {:?}", start.elapsed());
}
