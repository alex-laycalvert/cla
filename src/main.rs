mod calendar;

use calendar::Calendar;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::env;
use std::process;

#[derive(Debug)]
enum Command {
    Month,
    Year,
    Help,
    Invalid(String),
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 {
        parse_command_str(&args[1])
    } else {
        Command::Month
    };
    let range = if args.len() > 2 {
        parse_range_str(&args[2])
    } else {
        (0, None)
    };

    enable_raw_mode()?;
    match command {
        Command::Month => Calendar::print_relative_range(range, true),
        Command::Year => Calendar::print_year(2023),
        Command::Help => {
            disable_raw_mode()?;
            usage();
            process::exit(1);
        }
        Command::Invalid(s) => {
            disable_raw_mode()?;
            usage();
            eprintln!("Error: invalid command \"{s}\"");
            process::exit(1);
        }
    }?;
    disable_raw_mode()
}

fn usage() {
    println!("cla");
    println!("A terminal calendar tool written in Rust");
    println!("");
    println!("USAGE:");
    println!("\tcla [command] [subcommand]");
    println!("");
    println!("COMMANDS:");
    println!("\tmonth\tVarious ways of displaying months. Will display the");
    println!("\t\tcurrent month if no subcommand is given.");
    println!("\tyear\tDisplay the current year.");
    println!("");
    println!("SUBCOMMANDS:");
    println!("\tmonth\tFor the month command, the subcommand is interpreted");
    println!("\t\tas a number range representing the difference in months");
    println!("\t\tfrom the current month.");
    println!("");
    println!("\t\tExamples:");
    println!("\t\t\tcla month 0\tThis month");
    println!("\t\t\tcla month -1\tLast month");
    println!("\t\t\tcla month 2\t2 months from now");
    println!("\t\t\tcla month -2..0\t2 months ago to this month");
    println!("\t\t\tcla month -2..\tSame as above");
    println!("\t\t\tcla month 0..4\tThis month to 4 months from now");
    println!("\t\t\tcla month ..4\tSame as above");
    println!("\t\t\tcla month -1..3\tLast month to 3 months from now");
}

fn parse_command_str(cmd_str: &str) -> Command {
    if "--help".starts_with(&cmd_str.to_lowercase())
        || "-help".starts_with(&cmd_str.to_lowercase())
        || "help".starts_with(&cmd_str.to_lowercase())
    {
        return Command::Help;
    }
    if "month".starts_with(&cmd_str.to_lowercase()) {
        return Command::Month;
    }
    if "year".starts_with(&cmd_str.to_lowercase()) {
        return Command::Year;
    }
    Command::Invalid(String::from(cmd_str))
}

fn parse_range_str(range_str: &str) -> (i32, Option<i32>) {
    if range_str.is_empty() {
        return (0, None);
    }
    let tokens = range_str.split("..");
    let mut range = (0, None);
    for (i, t) in tokens.enumerate() {
        if i > 1 {
            return range;
        }
        let n = match t.parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                if t.is_empty() {
                    0
                } else {
                    return range;
                }
            }
        };
        if i == 0 {
            range.0 = n;
        } else {
            range.1 = Some(n);
        }
    }
    range
}
