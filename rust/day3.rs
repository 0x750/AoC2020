use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

fn main() {
    let lines = read_file("../input/3.txt");

    let slope: (u16, u16) = (3, 1); // (x, y)
    let mut position: (u16, u16) = (0, 0); // starting point
    let mut tree_count: u32 = 0;

    while usize::from(position.1) < lines.len() {
        if get_tree(&lines[position.1 as usize], position.0 as usize) {
            tree_count += 1;
        }

        position.0 = (position.0 + slope.0) % 31;
        position.1 += slope.1;
    }

    println!("Tree count : {}", tree_count);

    let slopes: [(u16, u16); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut total: u32 = 1;

    for slope in &slopes {
        tree_count = 0;
        position = (0,0);
        while usize::from(position.1) < lines.len() {
            if get_tree(&lines[position.1 as usize], position.0 as usize) {
                tree_count += 1;
            }

            position.0 = (position.0 + slope.0) % 31;
            position.1 += slope.1;
        }
        total *= tree_count;
    }

    println!("Total tree count multiply : {}", total);
}

fn get_tree(line: &String, position: usize) -> bool {
    line.as_bytes()[position] == "#".as_bytes()[0]
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}