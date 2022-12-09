use std::{collections::HashSet, fs, io::Read, time::Instant};

fn main() {
    let start = Instant::now();

    let mut file = fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut knots = [(0, 0); 10];
    let mut knots_1 = HashSet::new();
    let mut knots_9 = HashSet::new();
    knots_1.insert((0, 0));
    knots_9.insert((0, 0));
    for (direction, steps) in content
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[2..].parse::<i32>().unwrap()))
    {
        // move the head
        for _ in 0..steps {
            match direction {
                'R' => {
                    knots[0].0 += 1;
                }
                'L' => {
                    knots[0].0 -= 1;
                }
                'D' => {
                    knots[0].1 -= 1;
                }
                'U' => {
                    knots[0].1 += 1;
                }
                _ => unreachable!(),
            }

            let mut lead = knots[0];
            for (idx, tail) in knots[1..].iter_mut().enumerate() {
                let mut visits = match idx {
                    0 => Some(&mut knots_1),
                    8 => Some(&mut knots_9),
                    _ => None,
                };
                let diff_0: i32 = lead.0 - tail.0;
                let diff_1: i32 = lead.1 - tail.1;

                if diff_0.abs() < 2 && diff_1.abs() < 2 {
                    break;
                }

                if diff_0 != 0 && diff_1 != 0 {
                    tail.0 += diff_0.signum();
                    tail.1 += diff_1.signum();
                } else if diff_0 != 0 {
                    debug_assert!(diff_1 == 0);
                    tail.0 += diff_0.signum();
                } else if diff_1 != 0 {
                    debug_assert!(diff_0 == 0);
                    tail.1 += diff_1.signum();
                }

                if let Some(visits) = &mut visits {
                    visits.insert(*tail);
                }

                lead = *tail;
            }
        }
    }

    println!("Part 1: {}", knots_1.len());
    println!("Part 2: {}", knots_9.len());
    println!("Elapsed: {:?}", start.elapsed());
}
