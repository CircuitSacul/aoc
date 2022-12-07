use std::{collections::HashMap, fs, io::Read, time::Instant};

#[derive(Debug)]
struct Dir(HashMap<String, Dir>, u64);

impl Dir {
    fn new() -> Self {
        Self(HashMap::new(), 0)
    }

    fn traverse_to(&mut self, path: &[&str]) -> &mut Dir {
        if path.is_empty() {
            return self;
        }

        if !self.0.contains_key(path[0]) {
            self.0.insert(path[0].to_string(), Dir::new());
        }

        let dir = self.0.get_mut(path[0]).unwrap();
        dir.traverse_to(&path[1..])
    }

    fn set_size(&mut self, path: &[&str], size: u64) {
        self.traverse_to(path).1 = size;
    }

    fn insert_dir(&mut self, path: &[&str], dirname: &str) {
        self.traverse_to(path)
            .0
            .insert(dirname.to_string(), Dir::new());
    }

    fn size(&self) -> u64 {
        self.1 + self.0.iter().map(|(_, d)| d.size()).sum::<u64>()
    }
}

fn get_sizes(dir: &Dir, mut sizes: Vec<u64>) -> Vec<u64> {
    sizes.push(dir.size());

    for sub in dir.0.values() {
        sizes = get_sizes(sub, sizes);
    }

    sizes
}

fn main() {
    let start = Instant::now();

    let mut file = fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut fs = Dir::new();
    let mut path = Vec::new();

    let mut recording_dir_size: bool = false;
    let mut current_dir_size: u64 = 0;
    for line in content.lines() {
        if let Some(command) = line.strip_prefix("$ ") {
            if recording_dir_size {
                fs.set_size(&path, current_dir_size);
                recording_dir_size = false;
            }

            if let Some(dir) = command.strip_prefix("cd ") {
                match dir {
                    "/" => path.clear(),
                    ".." => {
                        path.pop();
                    }
                    dir => path.push(dir),
                }
            } else {
                recording_dir_size = true;
                current_dir_size = 0;
            }
        } else if let Some(dir) = line.strip_prefix("dir ") {
            fs.insert_dir(&path, dir);
        } else if recording_dir_size {
            // this is a file
            let (size, _filename) = line.split_once(' ').unwrap();
            let size = size.parse::<u64>().unwrap();
            current_dir_size += size;
        }
    }
    fs.set_size(&path, current_dir_size);

    println!(
        "Total {}",
        get_sizes(&fs, Vec::new())
            .iter()
            .filter(|s| **s < 100_000)
            .sum::<u64>()
    );

    let min_to_free = 30000000 - (70000000 - fs.size());

    println!(
        "Smallest {}",
        get_sizes(&fs, Vec::new())
            .iter()
            .filter(|s| **s > min_to_free)
            .min()
            .unwrap()
    );

    println!("Elapsed: {:?}", start.elapsed());
}
