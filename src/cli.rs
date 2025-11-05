//REPL + clap integration
use std::path::PathBuf;
use std::io::{self, Write};
use clap::{Parser, Subcommand};
use crate::{commands, replstate::ReplState};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands{
    Ls{
        #[arg(default_value = ".")]
        path: PathBuf,
    },  
    Cat{
        path: PathBuf,
    },
    Cd{
        path: PathBuf,
    },
    Exit
}

pub fn run() {
    println!("Starting ProV!");
    //REPL loop
    loop {
        //let mut state = ReplState::new();

        //Printing the os signature thing??? bro what
        //Flushing the output
        print!("prov> ");
        io::stdout().flush().unwrap();

        //Creating a new var input to store the input in and error handling it
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read input: {}", e);
            continue;
        }
        
        //If user just presses enter with no arguments
        if input.trim().is_empty() {
            continue;
        }

        //Converting the input from string into a vector
        let mut argv: Vec<String> = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        argv.insert(0, "prov".to_string());

        //Parsing the arguments with clap
        let args = match Args::try_parse_from(argv) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        
        //Matching the parsed arguments with commands
        match args.cmd {
            Commands::Ls { path } => commands::ls::run(path),
            Commands::Cat { path } => commands::cat::run(path),
            Commands::Cd { path } => commands::cat::run(path),
            Commands::Exit => commands::exit::run(),
        }
    }
}


