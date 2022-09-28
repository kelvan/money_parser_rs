mod reader;
mod writer;
mod booking;
use std::{process};
use reader::{load_csv, MoneyReader};
use writer::{write_csv, MoneyWriter};
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

    println!("Convert from {} to {}", args.informat, args.outformat);

    let booking_lines = match load_csv(args.infile, MoneyReader::from_str(&args.informat).unwrap()) {
        Ok(lines) => lines,
        Err(err) => {
            println!("error loading csv: {}", err);
            process::exit(1);
        }
    };

    match write_csv(booking_lines, args.outfile, MoneyWriter::from_str(&args.outformat).unwrap()) {
        Ok(_) => println!("Finished"),
        Err(err) => {
            println!("error writing csv: {}", err);
            process::exit(1);
        }
    }
}
