use std::{fs, io::Read, time::Instant};

pub fn main() {
    let start = Instant::now();

    let mut file = fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let forest_iter = content.lines().enumerate().flat_map(|(y, l)| {
        l.chars()
            .enumerate()
            .map(move |(x, c)| (x, y, c as usize - 48))
    });

    let mut tree_values = Vec::new();

    let mut x_by_height: [usize; 10] = [0; 10];
    let mut y_by_x_height: Vec<[usize; 10]> = Vec::new();
    let mut last_y = 0;
    let mut max_x = 0;
    for (x, y, h) in forest_iter {
        if x > max_x {
            max_x = x;
        }

        if x >= y_by_x_height.len() {
            y_by_x_height.push([0; 10]);
        }

        if last_y != y {
            last_y = y;
            x_by_height = [0; 10];
        }

        let x_back = x - x_by_height[h];
        let y_back = y - y_by_x_height[x][h];

        for h in 0..=h {
            x_by_height[h] = x;
            y_by_x_height[x][h] = y;
        }

        tree_values.push((x, y, h, y_back * x_back));
    }
    println!();

    let mut x_by_height: [usize; 10] = [max_x; 10];
    y_by_x_height.iter_mut().for_each(|v| *v = [max_x; 10]);
    let mut best = 0;
    for (x, y, h, ul_val) in tree_values.into_iter().rev() {
        if last_y != y {
            last_y = y;
            x_by_height = [x; 10];
        }

        let x_back = x_by_height[h] - x;
        let y_back = y_by_x_height[x][h] - y;

        for h in 0..=h {
            x_by_height[h] = x;
            y_by_x_height[x][h] = y;
        }

        let this = ul_val * y_back * x_back;
        if this > best {
            best = this;
        }
    }
    println!("Part 2 (new): {}", best);

    println!("Elapsed: {:?}", start.elapsed());
}
