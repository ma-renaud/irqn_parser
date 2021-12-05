use clap::{Arg, App};
use std::fs;
use regex::Regex;
use std::num::ParseIntError;

struct IRQ {
    name: String,
    number: i16,
}

fn main() {
    let matches = App::new("IRQn parser")
        .version("0.1.0")
        .author("Marc-André Renaud <ma.renaud@slashvoid.com>")
        .about("Create c++ enum for IRQn from stm32 defines file.")
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    if matches.is_present("INPUT") {
        let file = matches.value_of("INPUT").unwrap().to_string();

        let contents = fs::read_to_string(file)
            .expect("Something went wrong reading the file");

        let regex_typedef = Regex::new(r"typedef\s+enum\s*[^}]*}[^;]+;").unwrap();
        let regex_letter = Regex::new(r"[a-zA-Z]").unwrap();
        let mut irq_vector: Vec<IRQ> = Vec::new();

        for cap in regex_typedef.captures_iter(&*contents) {
            let mut lines: Vec<&str> = cap[0].lines().collect();
            lines.reverse();

            if lines.first().unwrap().contains("IRQn_Type") {
                let first = *lines.first().unwrap();
                let last = *lines.last().unwrap();
                for line in lines {
                    let trimmed_line = line.trim();
                    if line != first && line != last && regex_letter.is_match(&trimmed_line[..1]) {
                        match get_irq_from_string(trimmed_line) {
                            None => println!("Invalid irq number in {}", line),
                            Some(irq) => irq_vector.push(irq),
                        }
                    }
                }
            }
        }
    }
}

fn get_irq_from_string(irq_def: &str) -> Option<IRQ> {
    let split = irq_def.split("=").collect::<Vec<&str>>();
    if split.len() == 2 {
        let name = split.first().unwrap().trim();
        if let Ok(number) = get_irq_number(split.last().unwrap().trim()) {
            return Some(IRQ{name:name.to_owned(), number})
        }
    }
    None
}

fn get_irq_number(irq_num: &str) -> Result<i16, ParseIntError> {
    let number_str = irq_num.split(" ").collect::<Vec<&str>>();
    number_str.first().unwrap().parse::<i16>()
}
