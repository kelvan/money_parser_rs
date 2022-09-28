mod reader;
mod writer;
mod booking;
use std::{process};
use reader::{load_csv, MoneyReader};
use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    infile: String,
    #[clap(short, long, value_parser)]
    outfile: String,
    #[clap(long, value_parser)]
    informat: String,
    #[clap(long, value_parser, default_value = "homebank")]
    outformat: String,
}


fn main() {
    let args = Args::parse();

    println!("Convert from {} to {}\n", args.informat, args.outformat);

    if let Err(err) = load_csv(args.infile.to_string(), MoneyReader::from_str(&args.informat).unwrap()) {
        println!("error running example: {}", err);
        process::exit(1);
    }

    let booking_lines = match load_csv(args.infile, MoneyReader::from_str(&args.informat).unwrap()) {
        Ok(lines) => lines,
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
    };

    for line in booking_lines {
        print!("{:?}\n", line);
    }
}
