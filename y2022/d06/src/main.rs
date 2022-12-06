use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::Chars,
    time::Instant,
};

struct RollingWindow<'a, const T: usize> {
    window: [char; T],
    chars: Chars<'a>,
}

impl<'a, const T: usize> RollingWindow<'a, T> {
    fn new(mut chars: Chars<'a>) -> Self {
        Self {
            window: [(); T].map(|_| chars.next().unwrap()),
            chars,
        }
    }

    fn next_group(&mut self) -> [char; T] {
        let mut old_window = self.window.into_iter().skip(1);
        self.window = [(); T].map(|_| {
            old_window
                .next()
                .unwrap_or_else(|| self.chars.next().unwrap())
        });
        self.window
    }

    fn start_idx(&mut self) -> usize {
        'outer: for (idx, group) in self.enumerate() {
            for (idx, c) in group.iter().enumerate() {
                if group[idx + 1..group.len()].contains(c) {
                    continue 'outer;
                }
            }

            return idx + 1 + T;
        }

        unreachable!();
    }
}

impl<const T: usize> Iterator for RollingWindow<'_, T> {
    type Item = [char; T];

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_group())
    }
}

fn main() {
    let start = Instant::now();

    let file = BufReader::new(File::open("input.txt").unwrap());
    let mut lines = file.lines();
    let first = lines.next().unwrap().unwrap();

    let mut window_p1: RollingWindow<4> = RollingWindow::new(first.chars());
    let mut window_p2: RollingWindow<14> = RollingWindow::new(first.chars());

    println!("Part 1: {}", window_p1.start_idx());
    println!("Part 2: {}", window_p2.start_idx());

    println!("Elapsed: {:?}", start.elapsed());
}
