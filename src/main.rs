mod booking;
mod reader;
mod writer;
use clap::Parser;
use reader::{load_csv_file, MoneyReader};
use std::process;
use std::str::FromStr;
use writer::{write_csv, MoneyWriter};

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

    println!(
        "Convert {} ({}) -> {} ({})",
        args.infile, args.informat, args.outfile, args.outformat
    );

    let booking_lines =
        match load_csv_file(args.infile, MoneyReader::from_str(&args.informat).unwrap()) {
            Ok(lines) => lines,
            Err(err) => {
                println!("error loading csv: {}", err);
                process::exit(1);
            }
        };

    match write_csv(
        booking_lines,
        args.outfile,
        MoneyWriter::from_str(&args.outformat).unwrap(),
    ) {
        Ok(_) => println!("Finished"),
        Err(err) => {
            println!("error writing csv: {}", err);
            process::exit(1);
        }
    }
}
