use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

#[derive(Debug)]
struct Password {
    range: (u8, u8),
    letter: char,
    value: String,
}

impl Password {
    fn is_ok(&self) -> bool {
        let mut count: u8 = 0;

        for _char in self.value.chars() {
            if _char == self.letter {
                count += 1;
            }
        }
        count >= self.range.0 && count <= self.range.1
    }

    fn is_ok_newpolicy(&self) -> bool {
        (self.value.as_bytes()[(self.range.0 - 1) as usize] == self.letter as u8)
         ^ (self.value.as_bytes()[(self.range.1 - 1) as usize] == self.letter as u8)
    }
}

fn main() {
    let lines: Vec<Password> = read_file("../input/2.txt")
        .into_iter()
        .map(translate)
        .collect();

    let mut ok_count: u16 = 0;
    let mut ok_new: u16 = 0;

    for line in lines {
        if line.is_ok() {
            ok_count += 1;
        }
        if line.is_ok_newpolicy() {
            ok_new += 1;
        }

    }

    println!("Number of valid passwords : {}", ok_count);
    println!("Number of valid passwords (new policy) : {}", ok_new);
}

fn translate(from: String) -> Password {
    let splitted: Vec<&str> = from.split(" ").collect();

    Password {
        range: (
            splitted[0].split('-').collect::<Vec<&str>>()[0].parse::<u8>().unwrap(),
            splitted[0].split('-').collect::<Vec<&str>>()[1].parse::<u8>().unwrap(),
        ),
        letter: splitted[1].chars().next().unwrap(),
        value: String::from(splitted[2])
    }
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}