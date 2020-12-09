use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

fn main() {
    let groups = read_file("./input/6.txt");

    println!("{:#?}", groups.into_iter().map(||));

}

fn read_file(path: &str) -> Vec<String> {
    let mut file = File::open(path).expect("No such file");
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {},
        _ => {}
    };
    s.split("\n\n").map(String::from).collect()
}