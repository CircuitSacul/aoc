use std::{fs, io::Read, str::Lines, time};

struct LazyCaloryIter<'a> {
    lines_iter: Lines<'a>,
}

impl<'a> LazyCaloryIter<'a> {
    pub fn new(lines_iter: Lines<'a>) -> Self {
        Self { lines_iter }
    }

    fn generate_next_sum(&mut self) -> i32 {
        self.lines_iter
            .by_ref()
            .map_while(|line| line.parse::<i32>().ok())
            .sum()
    }
}

impl Iterator for LazyCaloryIter<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generate_next_sum() {
            0 => None,
            val => Some(val),
        }
    }
}

struct TopThreeStorage {
    values: (i32, i32, i32),
}

impl Default for TopThreeStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl TopThreeStorage {
    pub fn new() -> Self {
        Self { values: (0, 0, 0) }
    }

    pub fn lowest(&self) -> i32 {
        self.values.0
    }

    pub fn highest(&self) -> i32 {
        self.values.2
    }

    pub fn store(&mut self, value: i32) {
        if value > self.values.2 {
            self.values.0 = self.values.1;
            self.values.1 = self.values.2;
            self.values.2 = value;
        } else if value > self.values.1 {
            self.values.0 = self.values.1;
            self.values.1 = value;
        } else {
            self.values.0 = value;
        }
    }

    pub fn sum(&self) -> i32 {
        self.values.0 + self.values.1 + self.values.2
    }
}

fn main() -> anyhow::Result<()> {
    let start = time::Instant::now();

    let mut input = fs::File::open("input.txt")?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    let calory_iter = LazyCaloryIter::new(content.lines());
    let mut top_three = TopThreeStorage::new();
    for sum in calory_iter {
        if sum > top_three.lowest() {
            top_three.store(sum);
        }
    }

    println!("Part 1: Top elf sum: {}", top_three.highest());
    println!("Part 2: Sum of top three: {}", top_three.sum());

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
