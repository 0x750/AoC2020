use std::{
    collections::HashMap,
    fs::File,
    io::{
        prelude::*,
    },
};

fn main() {
    let mut passeports: Vec<HashMap<String, String>> = read_file("../input/4.txt")
        .into_iter()
        .map(translate)
        .collect();

    println!("Passports total count : {}", passeports.len());

    passeports = passeports
        .into_iter()
        .filter(|a| {
            match a.len() {
                8 => true,
                7 if a.contains_key("cid") == false => true,
                _ => false
            }
        })
        .collect();

    println!("Valid passports count : {}", passeports.len());

    passeports = passeports
        .into_iter()
        .filter(|a| {
            let byr: u16 = a.get("byr").unwrap().parse().unwrap();
            if byr > 2002 || byr < 1920 {return false;}
            
            let iyr: u16 = a.get("iyr").unwrap().parse().unwrap();
            if iyr > 2020 || iyr < 2010 {return false;}

            let eyr: u16 = a.get("eyr").unwrap().parse().unwrap();
            if eyr > 2030 || eyr < 2020 {return false;}

            let hgt_len: usize = a.get("hgt").unwrap().len();
            let unit: String = a
                .get("hgt")
                .unwrap()
                .chars()
                .skip(hgt_len - 2)
                .collect();

            match unit.as_str() {
                "cm" => {
                    let h: u8 = a
                        .get("hgt")
                        .unwrap()
                        .chars()
                        .take(hgt_len - 2)
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    if h > 193 || h < 150 {return false;}
                },
                "in" => {
                    let h: u8 = a
                        .get("hgt")
                        .unwrap()
                        .chars()
                        .take(hgt_len - 2)
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    if h > 76 || h < 59 {return false;}
                },
                _ => return false,
            }
            
            let hcl: &String = a.get("hcl").unwrap();
            if hcl.chars().take(1).collect::<String>() != "#" {return false;}
            if !hcl.chars().skip(1).all(|a| {
                a.is_numeric()
                || a == 'a'
                || a == 'b'
                || a == 'c'
                || a == 'd'
                || a == 'e'
                || a == 'f'
            }) {return false;}

            match a.get("ecl").unwrap().as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                    ()
                },
                _ => return false,
            };

            let pid: &String = a.get("pid").unwrap();
            if pid.len() < 9 {
                return false;
            }
            match pid.parse::<u32>() {
                Ok(_) => (),
                Err(_) => return false,
            }

            true
        })
        .collect();

    println!("Valid passports count, new rules : {}", passeports.len());
}

fn translate(from: String) -> HashMap<String, String> {
    let splitted: Vec<String> = from.replace("\n", " ").split(" ").map(String::from).collect();
    let mut pass: HashMap<String, String> = HashMap::new();
    for s in splitted {
        let ss: Vec<String> = s.split(":").map(String::from).collect();
        pass.insert(ss[0].to_owned(), ss[1].to_owned());
    }
    pass
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