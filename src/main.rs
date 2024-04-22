extern crate chrono;

use std::io;
use chrono::prelude::*;

fn main() {
    let name = input("What is your name? ").expect("Something went wrong!!!");
    println!("Hello, {}!", name);

    let age = input("What is your age? ").expect("Failed to get age.").parse::<i32>().expect("Invalid age.");

    let current_year = Utc::now().year();
    let hundred_year = 100 - age + current_year;

    if age > 100 {
        println!("Hey, {}! You turned 100 in the year: {}.", name, hundred_year);
    } else {
        println!("Hey, {}!, You'll turn 100 in the year: {}.", name, hundred_year);
    }
}

fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write;

    print!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}