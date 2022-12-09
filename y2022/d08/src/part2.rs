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

    let mut lines = content.lines();
    let max_x = lines.next().unwrap().len() - 1;
    let max_y = lines.count(); // + 1 - 1 == 0

    let mut tree_values = Vec::with_capacity((max_x + 1) * (max_y + 1));

    let mut x_by_height: [usize; 10] = [0; 10];
    let mut y_by_x_height: Vec<[usize; 10]> = Vec::with_capacity(max_x + 1);
    for _ in 0..=max_x {
        y_by_x_height.push([0; 10]);
    }
    for (x, y, h) in forest_iter {
        if x == 0 {
            x_by_height = [0; 10];
        }

        let y_by_height = &mut y_by_x_height[x];

        let x_back = x - x_by_height[h];
        let y_back = y - y_by_height[h];

        for h in 0..=h {
            x_by_height[h] = x;
            y_by_height[h] = y;
        }

        tree_values.push((x, y, h, y_back * x_back));
    }

    x_by_height = [max_x; 10];
    y_by_x_height.iter_mut().for_each(|v| *v = [max_x; 10]);
    let mut best = 0;
    for (x, y, h, ul_val) in tree_values.into_iter().rev() {
        if x == max_x {
            x_by_height = [max_x; 10];
        }

        let y_by_height = &mut y_by_x_height[x];

        let x_back = x_by_height[h] - x;
        let y_back = y_by_height[h] - y;

        for h in 0..=h {
            x_by_height[h] = x;
            y_by_height[h] = y;
        }

        let this = ul_val * y_back * x_back;
        if this > best {
            best = this;
        }
    }
    println!("Part 2 (new): {}", best);

    println!("Elapsed: {:?}", start.elapsed());
}
