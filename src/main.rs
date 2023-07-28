mod calendar;

use calendar::Calendar;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::env;

#[derive(Debug)]
enum Command {
    Month,
    Year,
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
    }?;
    disable_raw_mode()
}

fn parse_command_str(cmd_str: &String) -> Command {
    if "month".starts_with(&cmd_str.to_lowercase()) {
        return Command::Month;
    }
    if "year".starts_with(&cmd_str.to_lowercase()) {
        return Command::Year;
    }
    Command::Month
}

fn parse_range_str(range_str: &String) -> (i32, Option<i32>) {
    if range_str.len() == 0 {
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
                if t.len() == 0 {
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
