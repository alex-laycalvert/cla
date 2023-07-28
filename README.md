# cla

A Rust implementation of `cal` with some extra features.

## WORK IN PROGRESS

I just started this project and if I follow through with it, the goal will be for `cla` to be a functioning TODO/Agenda system.

Right now, cloning and repo and running using `cargo run` will just print out a calendar just like calling `cal` with no arguments.

## Usage

```text
cla
A terminal calendar tool written in Rust

USAGE:
    cla [command] [subcommand]

COMMANDS:
    month   Various ways of displaying months. Will display the
    current month if no subcommand is given.
    year    Display the current year.

SUBCOMMANDS:
    month   For the month command, the subcommand is interpreted
            as a number range representing the difference in months
            from the current month.

            Examples:
                cla month 0     This month
                cla month -1    Last month
                cla month 2     2 months from now
                cla month -2..0 2 months ago to this month
                cla month -2..  Same as above
                cla month 0..4  This month to 4 months from now
                cla month ..4   Same as above
                cla month -1..3 Last month to 3 months from now
```
