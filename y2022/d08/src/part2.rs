use std::{fs, io::Read, time::Instant};

#[derive(Clone, Debug)]
struct Tree {
    height: usize,
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self {
            height,
            up: None,
            down: None,
            right: None,
            left: None,
        }
    }

    fn value(&self) -> usize {
        self.up.unwrap() * self.down.unwrap() * self.right.unwrap() * self.left.unwrap()
    }
}

fn iter_forest(map: &mut [Vec<Tree>], max: (usize, usize)) {
    let mut pointer = (0, 0);
    let mut reverse = false;

    let mut back_by_height: [usize; 10] = [0; 10];
    let mut back_row_by_height: Vec<[usize; 10]> = Vec::new();

    loop {
        let mut tree = &mut map[pointer.1][pointer.0];

        if pointer.0 >= back_row_by_height.len() {
            if reverse {
                back_row_by_height.push([max.1; 10]);
            } else {
                back_row_by_height.push([0; 10]);
            }
        }

        let back = back_by_height[tree.height];
        let row = back_row_by_height[pointer.0][tree.height];

        for h in 0..=tree.height {
            back_by_height[h] = pointer.0;
            back_row_by_height[pointer.0][h] = pointer.1;
        }

        if reverse {
            tree.right = Some(back - pointer.0);
            tree.down = Some(row - pointer.1);

            if pointer.0 == 0 {
                if pointer.1 == 0 {
                    break;
                }
                back_by_height = [max.0; 10];

                pointer.0 = max.0;
                pointer.1 -= 1;
            } else {
                pointer.0 -= 1;
            }
        } else {
            tree.left = Some(pointer.0 - back);
            tree.up = Some(pointer.1 - row);

            if pointer.0 == max.0 {
                if pointer.1 == max.1 {
                    reverse = true;
                    back_by_height = [max.0; 10];
                    back_row_by_height.iter_mut().for_each(|s| *s = [max.1; 10]);
                    continue;
                }

                pointer.0 = 0;
                back_by_height = [0; 10];
                pointer.1 += 1;
            } else {
                pointer.0 += 1;
            }
        }
    }
}

pub fn main() {
    let start = Instant::now();

    let mut file = fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut forest = Vec::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, l) in content.lines().enumerate() {
        forest.push(Vec::new());
        if y > max_y {
            max_y = y
        };
        for (x, c) in l.chars().enumerate() {
            if x > max_x {
                max_x = x
            };
            forest[y].push(Tree::new(c as usize - 48));
        }
    }

    iter_forest(&mut forest, (max_x, max_y));

    let mut best = 0;
    for line in forest {
        for tree in line {
            let val = tree.value();
            if val > best {
                best = val;
            }
        }
    }

    println!("Part 2: {:?}", best);
    println!("Elapsed: {:?}", start.elapsed());
}
