mod cli;
mod modules;

use clap::Parser;
use cli::*;
use modules::calendar::Calendar;

pub struct Cla {}

impl Cla {
    pub fn execute_from_args() {
        let args = Cli::parse();
        let cmd = match args.command {
            Some(cmd) => cmd,
            None => Commands::Month(CalendarArgs::new()),
        };
        match cmd {
            Commands::Month(args) => Calendar::months_from_args(args),
            Commands::Year(args) => Calendar::years_from_args(args),
            Commands::Agenda => todo!(),
        }
        .unwrap();
    }
}
