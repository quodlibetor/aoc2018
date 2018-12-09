use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};

use structopt::StructOpt;

#[derive(StructOpt)]
struct App {
    fname: String,
    #[structopt(long = "two")]
    is_two: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = App::from_args();
    let lines = BufReader::new(File::open(&args.fname)?).lines();
    if !args.is_two {
        let freq = lines.fold(0i64, |freq, l| freq + l.unwrap().parse::<i64>().unwrap());
        println!("{}", freq);
    } else {
        println!("{}", original(&args, lines)?);
        println!("{}", other(args)?);
    }
    Ok(())
}

fn original(args: &App, lines: Lines<BufReader<File>>) -> Result<i64, Box<dyn Error>> {
    let mut seen = HashSet::new();
    let mut freq = 0;
    let lines = lines.collect::<Result<Vec<String>, _>>().unwrap();
    for change in lines.iter().cloned().cycle() {
        freq += change.parse::<i64>().unwrap();
        if seen.contains(&freq) {
            return Ok(freq)
        }
        seen.insert(freq);
    }
    Err("Frequency not found".into())
}

fn other(args: App) -> Result<i32, Box<dyn Error>> {
    let mut s = String::new();
    File::open(args.fname)?.read_to_string(&mut s)?;
    let mut seen = HashSet::new();
    let mut freq = 0i32;
    for change in s.lines().cycle() {
        freq += change.parse::<i32>()?;
        if seen.contains(&freq) {
            return Ok(freq);
        }
        seen.insert(freq);
    }
    unreachable!()
}
