use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug)]
pub struct CalendarArgs {
    #[arg(short, long)]
    pub relative_range: Option<String>,

    pub range: Option<String>,
}

impl CalendarArgs {
    pub fn new() -> Self {
        Self {
            relative_range: None,
            range: None,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Month(CalendarArgs),
    Year(CalendarArgs),
    Agenda,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
