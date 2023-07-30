use crate::cla::cli::CalendarArgs;
use chrono::prelude::*;
use crossterm::{
    cursor::{MoveRight, MoveUp},
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{self, disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::stdout;

const CAL_WIDTH: u16 = 20;
const CAL_HOR_PADDING: u16 = 4;
const MAX_MONTHS_PER_ROW: u16 = 4;
const CAL_HEADER_SIZE: u16 = 2;
const MONTHS: [&str; 12] = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

pub struct Calendar {}

impl Calendar {
    pub fn years_from_args(args: CalendarArgs) -> Result<()> {
        enable_raw_mode()?;
        let range = if let Some(r) = args.relative_range {
            Self::parse_year_relative_range(r)
        } else {
            vec![]
        };
        let today = Local::now();
        let months_per_row = Self::months_per_row()?;
        let row_width = (months_per_row * (CAL_WIDTH + CAL_HOR_PADDING) - CAL_HOR_PADDING) as usize;
        for y in range {
            let mut months = vec![];
            for m in 1..=12 {
                let d = if y == today.year() && m == today.month() as i32 {
                    today.day() as i32
                } else {
                    0
                };
                months.push((y, m, d));
            }
            execute!(
                stdout(),
                SetAttribute(Attribute::Bold),
                Print(format!("\r\n{: ^row_width$}\r\n\n", y)),
                SetAttribute(Attribute::Reset),
            )?;
            Self::print_months(months, false)?;
        }
        disable_raw_mode()
    }

    pub fn months_from_args(args: CalendarArgs) -> Result<()> {
        enable_raw_mode()?;
        let range = if let Some(r) = args.relative_range {
            Self::parse_month_relative_range(r)
        } else {
            Self::parse_month_range(args.range)
        };
        Self::print_months(range, true)?;
        disable_raw_mode()
    }

    fn parse_month_range(range_str: Option<String>) -> Vec<(i32, i32, i32)> {
        let today = Local::now();
        let this_month = today.month() as i32;
        let (start, end) = match range_str {
            Some(range_str) => {
                let splits: Vec<&str> = range_str.split("..").collect();
                let mut start = splits[0].parse().unwrap_or(Self::month_number(splits[0])) % 12;
                if start <= 0 {
                    start = 1
                }
                let end = match splits.len() {
                    2.. => splits[1].parse().unwrap_or(Self::month_number(splits[1])),
                    _ => start,
                };
                (start, end)
            }
            None => return vec![(today.year(), today.month() as i32, today.day() as i32)],
        };
        let mut range = vec![];
        let y = today.year();
        for m in start..=end {
            let d = if m == this_month {
                today.day() as i32
            } else {
                0
            };
            range.push((y, m, d));
        }
        range
    }

    fn parse_month_relative_range(range_str: String) -> Vec<(i32, i32, i32)> {
        let today = Local::now();
        let (start, end) = Self::parse_relative_range_bounds(range_str);
        let mut range = vec![];
        for i in start..=end {
            let mut y = today.year();
            let mut m = today.month() as i32 + i;
            if m > 12 {
                y += m / 12;
                m %= 12;
            }
            if m < 1 {
                y -= 1 + m.abs() / 12;
                m = 12 - (m.abs() % 12);
            }
            range.push((y, m, if i == 0 { today.day() as i32 } else { 0 }));
        }
        range
    }

    fn parse_year_relative_range(range_str: String) -> Vec<i32> {
        let today = Local::now();
        let (start, end) = Self::parse_relative_range_bounds(range_str);
        let mut range = vec![];
        for i in start..=end {
            range.push(today.year() + i);
        }
        range
    }

    fn parse_relative_range_bounds(range_str: String) -> (i32, i32) {
        let splits: Vec<&str> = range_str.split("..").collect();
        if splits.is_empty() {
            return (0, 0);
        }
        let start: i32 = splits[0].parse().unwrap_or_default();
        let end: i32 = match splits.len() {
            2.. => splits[1].parse().unwrap_or_default(),
            _ => start,
        };
        (start, end)
    }

    fn month_number(month_str: &str) -> i32 {
        for (i, m) in MONTHS.iter().enumerate() {
            if m.starts_with(&month_str.to_lowercase()) {
                return i as i32 + 1;
            }
        }
        12
    }

    fn print_months(months: Vec<(i32, i32, i32)>, include_year: bool) -> Result<()> {
        let months_per_row = Self::months_per_row()?;
        for (i, r) in months.iter().enumerate() {
            let mut rows = 0;
            let cols = 7;
            let first_day_of_month = Local.with_ymd_and_hms(r.0, r.1 as u32, 1, 0, 0, 0).unwrap();
            let days_in_current_month = Self::days_in_month(r.0, r.1 as u32);
            let weekday_num = Self::weekday_number(first_day_of_month.weekday());
            rows += 1;
            rows += (days_in_current_month - (7 - weekday_num)) / cols;
            if (days_in_current_month - (7 - weekday_num)) % cols != 0 {
                rows += 1
            }
            Self::print_month_highlight_day(
                r.0,
                r.1,
                r.2,
                include_year,
                (i as u16 % months_per_row) * (CAL_WIDTH + CAL_HOR_PADDING),
            )?;
            if i != months.len() - 1 && i as u16 % months_per_row != months_per_row - 1 {
                execute!(stdout(), MoveUp(rows as u16 + CAL_HEADER_SIZE))?;
            } else {
                execute!(stdout(), Print("\r\n\n"))?;
            }
        }
        Ok(())
    }

    /// Display the given month as a calendar.
    ///
    /// * `year` - Year of month to display
    /// * `month` - Month to display
    /// * `day` - Day to highlight on display, set to `0` to not highlight anything
    /// * `include_year` - If the year of the month should be included in the display
    fn print_month_highlight_day(
        year: i32,
        month: i32,
        day: i32,
        include_year: bool,
        horizontal_offset: u16,
    ) -> Result<()> {
        let mut rows = 0;
        let cols = 7;

        let first_day_of_month = Local
            .with_ymd_and_hms(year, month as u32, 1, 0, 0, 0)
            .unwrap();
        let days_in_current_month = Self::days_in_month(year, month as u32);
        let weekday_num = Self::weekday_number(first_day_of_month.weekday());
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
                    current_row.push(-1);
                } else {
                    i += 1;
                    current_row.push(i as i32);
                }
            }
            calendar.push(current_row.clone());
            current_row.clear();
        }

        let mut label_str = first_day_of_month.format("%B").to_string();
        if include_year {
            label_str = label_str + " " + &year.to_string();
        }
        let w = CAL_WIDTH as usize;
        if horizontal_offset > 0 {
            execute!(stdout(), MoveRight(horizontal_offset),)?;
        }
        execute!(
            stdout(),
            SetAttribute(Attribute::Bold),
            Print(format!("{: ^w$}\r\n", label_str)),
            SetAttribute(Attribute::Reset),
        )?;
        if horizontal_offset > 0 {
            execute!(stdout(), MoveRight(horizontal_offset),)?;
        }
        execute!(stdout(), Print("Su Mo Tu We Th Fr Sa\r\n"),)?;
        for r in calendar {
            if horizontal_offset > 0 {
                execute!(stdout(), MoveRight(horizontal_offset),)?;
            }
            for d in r {
                let str = if d > 0 { d.to_string() } else { "".to_string() };
                if day == d {
                    execute!(
                        stdout(),
                        SetBackgroundColor(Color::White),
                        SetForegroundColor(Color::Black),
                        Print(format!("{: >2}", str)),
                        ResetColor,
                        MoveRight(1)
                    )?;
                } else {
                    execute!(stdout(), Print(format!("{: >2}", str)), MoveRight(1))?;
                }
            }
            execute!(stdout(), Print("\r\n"))?;
        }
        Ok(())
    }

    fn days_in_month(year: i32, month: u32) -> u32 {
        let next_month = if month == 12 { 1 } else { month + 1 };
        NaiveDate::from_ymd_opt(year, next_month, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    }

    fn weekday_number(weekday: chrono::Weekday) -> u32 {
        match weekday {
            Weekday::Sun => 0,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        }
    }

    fn months_per_row() -> Result<u16> {
        let (term_cols, _) = terminal::size()?;
        Ok((term_cols / (CAL_WIDTH + CAL_HOR_PADDING))
            .min(MAX_MONTHS_PER_ROW)
            .max(1))
    }
}

#[cfg(test)]
mod tests {
    use crate::cla::modules::calendar::Calendar;

    #[test]
    fn relative_ranges() {
        let inputs = vec![
            ("", (0, 0)),
            ("0", (0, 0)),
            ("4", (4, 4)),
            ("1..5", (1, 5)),
            ("-2..5", (-2, 5)),
            ("..5", (0, 5)),
            ("-5..", (-5, 0)),
        ];
        for i in inputs {
            assert_eq!(
                i.1,
                Calendar::parse_relative_range_bounds(String::from(i.0))
            );
        }
    }
}
