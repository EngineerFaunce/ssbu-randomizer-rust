use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let fighter_list = Fighter::get_all_fighters();
    let mut input = String::new();

    println!("SSBU Randomizer\n");

    loop {
        print!("Enter a command: ");
        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match &input.trim()[..] {
            "help" => {
                println!("\nlist - view all fighters");
                println!("gen, generate - generate a randomized list of fighters");
                println!("exit - exit the program\n");
            }
            "list" => {
                println!();
                for fighter in &fighter_list {
                    println!("{}", fighter.name);
                }
                println!();
            }
            "generate" | "gen" => {
                let mut new_list = fighter_list.to_vec();
                new_list.shuffle(&mut thread_rng());

                println!();
                for fighter in new_list {
                    println!("{}", fighter.name);
                }
                println!();
            }
            "exit" => std::process::exit(0),
            "" => continue,
            &_ => println!("Unknown command. Type 'help' for a list of commands."),
        }

        input = String::new();
    }
}

#[derive(Clone, Debug)]
struct Fighter {
    name: String,
    series: String,
    is_echo: bool,
}

impl Fighter {
    // Returns a list of all fighters from data file
    fn get_all_fighters() -> Vec<Fighter> {
        // The output is wrapped in a Result to allow matching on errors
        // Returns an Iterator to the Reader of the lines of the file.
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where
            P: AsRef<Path>,
        {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

        let mut fighter_list: Vec<Fighter> = Vec::new();

        if let Ok(lines) = read_lines("./src/fighters.csv") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(fighter) = line {
                    let split: Vec<&str> = fighter.split(",").collect();
                    let fighter = Fighter {
                        name: split[0].to_string(),
                        series: split[1].to_string(),
                        is_echo: split[2] == "true",
                    };
                    fighter_list.push(fighter);
                }
            }
        }
        fighter_list
    }
}
