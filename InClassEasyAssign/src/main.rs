use std::io::{self, Write};
use std::io::Read;
use std::fs::File;
//use std::io::prelude::*;

fn reading_from_console() {
    let mut buffer = String::new();
    let mut file = File::create("config.txt").unwrap();

    print!("What is your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    writeln!(file, "{}", buffer.trim().to_string()).unwrap();
    buffer.clear();

    print!("What is your student ID? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    writeln!(file, "{}", buffer.trim().to_string()).unwrap();
}

struct Config {
    name: String,
    stuid: String,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let stuid = lines.next().unwrap().to_string();

        Config {name, stuid}
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Student Name: {}", config.name);
    println!("Student ID: {}", config.stuid);
}

fn main(){
    reading_from_console();
    reading_from_file();
}