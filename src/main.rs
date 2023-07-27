extern crate chrono;

use chrono::prelude::*;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
};
use std::io::stdout;

fn main() -> Result<()> {
    let mut rows = 0;
    let cols = 7;

    let today = Local::now();
    let year = today.year();
    let month = today.month();

    let first_day_of_month = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
    let days_in_current_month = days_in_month(year, month);
    let weekday_num = match first_day_of_month.weekday() {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    };
    rows += 1;
    rows += (days_in_current_month - (7 - weekday_num)) / cols;
    if (days_in_current_month - (7 - weekday_num)) % cols != 0 {
        rows += 1
    }

    let mut calendar = vec![];
    let mut i = 0;
    for _ in 0..rows {
        let mut current_row = vec![];
        for c in 0..cols {
            if (i == 0 && c != weekday_num) || i == days_in_current_month {
                current_row.push(0);
            } else {
                i += 1;
                current_row.push(i);
            }
        }
        calendar.push(current_row.clone());
        current_row.clear();
    }

    execute!(
        stdout(),
        Print(format!(
            "{: ^20}\n",
            today.format("%B").to_string() + " " + &year.to_string()
        )),
        Print("Su Mo Tu We Th Fr Sa\n")
    )?;
    for r in calendar {
        for d in r {
            let str = if d == 0 {
                "".to_string()
            } else {
                d.to_string()
            };
            if today.day() == d {
                execute!(
                    stdout(),
                    SetBackgroundColor(Color::White),
                    SetForegroundColor(Color::Black),
                    Print(format!("{: >2}", str)),
                    ResetColor,
                    Print(" "),
                )?;
            } else {
                execute!(stdout(), Print(format!("{: >2} ", str)))?;
            }
        }
        execute!(stdout(), Print("\n"))?;
    }

    Ok(())
}

fn days_in_month(year: i32, month: u32) -> u32 {
    // Get the first day of the next month and subtract 1 to get the last day of the given month.
    let next_month = if month == 12 { 1 } else { month + 1 };
    let last_day_of_month = NaiveDate::from_ymd_opt(year, next_month, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day();

    last_day_of_month
}
