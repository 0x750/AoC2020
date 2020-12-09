use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

fn main() {
    let lines = read_file("../input/1.txt");

    'outer: for i in 0..lines.len() {
        for j in i..lines.len() {
            if lines[i] + lines[j] == 2020 {
                println!("{} + {} == 2020", lines[i], lines[j]);
                println!("{} * {} == {}",
                    lines[i],
                    lines[j],
                    lines[i] * lines[j]
                );
                break 'outer;
            }
        }
    }

    for i in 0..lines.len() {
        for j in i..lines.len() {
            for k in j..lines.len() {
                if lines[i] + lines[j] + lines[k] == 2020 {
                    println!("{} + {} + {} == 2020", lines[i], lines[j], lines[k]);
                    println!("{} * {} * {} == {}",
                        lines[i],
                        lines[j],
                        lines[k],
                        lines[i] * lines[j] * lines[k]
                    );
                    return;
                }
            }
        }
    }

}

fn read_file(path: &str) -> Vec<u32> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|i| i.parse::<u32>().unwrap())
        .collect()
}