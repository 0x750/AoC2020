use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

#[derive(Debug)]
struct BoardingPass {
    row: u16,
    column: u16,
    id: u16
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

fn main() {
    let tickets: Vec<BoardingPass> = read_file("../input/5.txt")
        .into_iter()
        .map(translate)
        .collect();

    let mut max_id: u16 = 0;

    for ticket in &tickets {
        if ticket.id > max_id {
            max_id = ticket.id;
        }
    }

    println!("Maximum ID : {}", max_id);

    let mut my_boarding_pass: BoardingPass = BoardingPass {
        row: 0,
        column: 0,
        id: 0,
    };

    for row in 9..120 {
        for column in 0..7 {
            let ticket: BoardingPass = BoardingPass {
                row: row,
                column: column,
                id: row * 8 + column,
            };

            if !tickets.contains(&ticket) {
                my_boarding_pass = ticket;
            }
        }
    }

    println!("My boarding pass is : {:#?}", my_boarding_pass);

}

fn translate(from: String) -> BoardingPass {
    let row: String = from
        .chars()
        .take(7)
        .collect::<String>()
        .replace("F", "0")
        .replace("B", "1");

    let column: String = from
        .chars()
        .skip(7)
        .take(3)
        .collect::<String>()
        .replace("L", "0")
        .replace("R", "1");

    BoardingPass {
        row: u16::from_str_radix(row.as_str(), 2).unwrap(),
        column: u16::from_str_radix(column.as_str(), 2).unwrap(),
        id: u16::from_str_radix(row.as_str(), 2).unwrap() 
        * 8 
        + u16::from_str_radix(column.as_str(), 2).unwrap(),
    }
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}