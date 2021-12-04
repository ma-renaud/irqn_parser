use clap::{Arg, App, AppSettings};
use std::fs;
use regex::Regex;

fn main() {
    let matches = App::new("Mediaplayer Controller")
        .version("0.1.0")
        .author("Marc-André Renaud <ma.renaud@slashvoid.com>")
        .about("Call various actions of active media players")
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    if matches.is_present("INPUT") {
        let file = matches.value_of("INPUT").unwrap().to_string();
        println!("Value for input: {}", file);

        println!("In file {}", file);

        let contents = fs::read_to_string(file)
            .expect("Something went wrong reading the file");


        let re = Regex::new(r"typedef\s+enum\s*[^}]*}[^;]+;").unwrap();
        if  re.is_match(&*contents) {
            println!("Trouvé!");
        }

        for cap in re.captures_iter(&*contents) {
            println!("{}", &cap[0]);
        }
    }
}
