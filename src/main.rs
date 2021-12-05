use clap::{Arg, App};
use std::fs;
use std::fs::File;
use std::io::Write;
use regex::Regex;
use std::num::ParseIntError;
use std::path::Path;

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
        .arg(Arg::new("OUTPUT")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .about("Sets a custom output file")
            .takes_value(true))
        .get_matches();

    if matches.is_present("INPUT") {
        let file = matches.value_of("INPUT").unwrap().to_string();
        let path = Path::new(&file);
        if path.exists() {
            let mut output = String::from("");
            if matches.is_present("OUTPUT") {
                output = matches.value_of("OUTPUT").unwrap().to_owned();
                let output_path = Path::new(&output);
                if !output_path.parent().unwrap().exists() {
                    println!("Invalid output file path: {}", output);
                    output = String::from("");
                }
            } else {
                if let Some(output_path) = path.parent() {
                    let tmp_path = output_path.join("stm32f401_irqn.h");
                    output.push_str(tmp_path.to_str().unwrap());
                } else {
                    println!("Invalid input file path: {}", file);
                }
            }

            if !output.is_empty() {
                let text = fs::read_to_string(file)
                    .expect("Something went wrong reading the file");

                let mut irq_vector = irq_vector_from_enum(&text);
                irq_vector.reverse();
                write_irqn_enum(&output, irq_vector);
                println!("File created: {}", output);
            }
        } else {
            println!("Invalid input file path: {}", file);
        }
    }
}

fn irq_vector_from_enum(typedef: &str) -> Vec<IRQ> {
    let regex_typedef = Regex::new(r"typedef\s+enum\s*[^}]*}[^;]+;").unwrap();
    let regex_letter = Regex::new(r"[a-zA-Z]").unwrap();
    let mut irq_vector: Vec<IRQ> = Vec::new();

    for cap in regex_typedef.captures_iter(&*typedef) {
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

    return irq_vector;
}

fn get_irq_from_string(irq_def: &str) -> Option<IRQ> {
    let split = irq_def.split("=").collect::<Vec<&str>>();
    if split.len() == 2 {
        let name = split.first().unwrap().trim();
        if let Ok(number) = get_irq_number(split.last().unwrap().trim()) {
            return Some(IRQ { name: name.to_owned(), number });
        }
    }
    None
}

fn get_irq_number(irq_num: &str) -> Result<i16, ParseIntError> {
    let number_str = irq_num.split(" ").collect::<Vec<&str>>().first().unwrap().trim();
    if &number_str[number_str.len() - 1..] == "," {
        number_str[..number_str.len() - 1].parse::<i16>()
    } else {
        number_str.parse::<i16>()
    }
}

fn write_irqn_enum(output: &str, irq_vector: Vec<IRQ>) {
    let mut out_file = File::create(output).expect("Unable to create output file");

    out_file.write_all(b"#ifndef STM32F401_IRQN_H\r\n").expect("Unable to write data");
    out_file.write_all(b"#define STM32F401_IRQN_H\r\n\r\n").expect("Unable to write data");

    out_file.write_all(b"#include <cstdint>\r\n\r\n").expect("Unable to write data");

    out_file.write_all(b"enum class Stm32IRQn : int16_t {\r\n").expect("Unable to write data");

    out_file.write_all(b"  NonMaskableInt_IRQn = -14,\r\n").expect("Unable to write data");
    out_file.write_all(b"  HardFault_IRQn = -13,\r\n").expect("Unable to write data");
    out_file.write_all(b"  MemoryManagement_IRQn = -12,\r\n").expect("Unable to write data");
    out_file.write_all(b"  BusFault_IRQn = -11,\r\n").expect("Unable to write data");
    out_file.write_all(b"  UsageFault_IRQn = -10,\r\n").expect("Unable to write data");
    out_file.write_all(b"  SVCall_IRQn = -5,\r\n").expect("Unable to write data");
    out_file.write_all(b"  DebugMonitor_IRQn = -4,\r\n").expect("Unable to write data");
    out_file.write_all(b"  PendSV_IRQn = -2,\r\n").expect("Unable to write data");
    out_file.write_all(b"  SysTick_IRQn = -1,\r\n").expect("Unable to write data");

    for irq in irq_vector {
        if irq.number >= 0 {
            out_file.write_all(format!("  {} = {},\r\n", irq.name, irq.number).as_bytes()).expect("Unable to write data");
        }
    }

    out_file.write_all(b"};\r\n\r\n").expect("Unable to write data");
    out_file.write_all(b"#endif //STM32F401_IRQN_H").expect("Unable to write data");
}
