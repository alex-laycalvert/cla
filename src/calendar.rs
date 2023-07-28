use chrono::prelude::*;
use crossterm::{
    cursor::{MoveRight, MoveUp},
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal, Result,
};
use std::io::stdout;

const CAL_WIDTH: u16 = 20;
const CAL_HOR_PADDING: u16 = 4;
const MAX_MONTHS_PER_ROW: u16 = 4;
const CAL_HEADER_SIZE: u16 = 2;

pub struct Calendar {}

impl Calendar {
    pub fn print_year(year: i32) -> Result<()> {
        let months_per_row = Self::months_per_row()?;
        let row_width = (months_per_row * (CAL_WIDTH + CAL_HOR_PADDING)) as usize;
        execute!(
            stdout(),
            SetAttribute(Attribute::Bold),
            Print(format!("\r\n{: ^row_width$}\r\n\n", year)),
            SetAttribute(Attribute::Reset),
        )?;
        Self::print_range((0, Some(12)), year, false)
    }

    /// Display the given month as a calendar. Calls
    /// [`print_month_highlight_day`](Calendar::print_month_highlight_day) with the given options,
    /// no day to highlight, and to display the year in the title.
    ///
    /// * `year` - Year of month to display
    /// * `month` - Month to display
    pub fn _print_month(year: i32, month: i32) -> Result<()> {
        Self::print_month_highlight_day(year, month, 0, true, 0)
    }

    /// Given a tuple representing the start and end of a range of
    /// months where `1` = `January`, print each month as a calendar.
    ///
    /// * `range` - Tuple of numbers representing the range of months as indicies
    ///             where the first `i32` is the start of the range and the second
    ///             `Option<i32>` is the optional end of the range. If no end range
    ///             is given, the start will be the only month displayed. `1` = `January,
    ///             `2` = `February`, etc.
    /// * `year` - Year of months
    /// * `include_year` - Whether to include the year in the display of the months.
    pub fn print_range(range: (i32, Option<i32>), year: i32, include_year: bool) -> Result<()> {
        let today = Local::now();
        let mut calendar_range = vec![];
        let mut start = range.0 % 12;
        if start <= 0 {
            start = 1
        }
        if let Some(end) = range.1 {
            for i in start..=end {
                calendar_range.push((
                    year,
                    i,
                    if i == today.month() as i32 && year == today.year() {
                        today.day() as i32
                    } else {
                        0
                    },
                ));
            }
        } else {
            calendar_range.push((
                year,
                start,
                if today.year() == year && start == today.month() as i32 {
                    today.day() as i32
                } else {
                    0
                },
            ));
        }
        Self::print_many_months(calendar_range, include_year)
    }

    /// Given a tuple representing the start and end of a range of
    /// months relative to the current month, display each month as
    /// a calendar.
    ///
    /// * `range` - Tuple of numbers representing the relative difference in months
    ///             from the current month where the first `i32` is the start of the range
    ///             and the second `Option<i32>` is the optional end of the range. If no
    ///             end range is given, the start will be the only month displayed.
    ///             `-1` = last month, `0` = this month, `2` = two months from now, etc.
    /// * `include_year` - Whether to include the year in the display of the months.
    pub fn print_relative_range(range: (i32, Option<i32>), include_year: bool) -> Result<()> {
        let today = Local::now();
        let mut calendar_range = vec![];
        let start = range.0;
        if let Some(end) = range.1 {
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
                calendar_range.push((y, m, if i == 0 { today.day() as i32 } else { 0 }));
            }
        } else {
            let mut y = today.year();
            let mut m = today.month() as i32 + start;
            if m > 12 {
                y += m / 12;
                m %= 12;
            }
            if m < 1 {
                y -= 1 + m.abs() / 12;
                m = 12 - (m.abs() % 12);
            }
            calendar_range.push((y, m, if start == 0 { today.day() as i32 } else { 0 }));
        }
        Self::print_many_months(calendar_range, include_year)
    }

    pub fn print_many_months(months: Vec<(i32, i32, i32)>, include_year: bool) -> Result<()> {
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

    /// Display the current month as a calendar. Calls
    /// [`print_month_highlight_day`](Calendar::print_month_higlight_day) with
    /// the current date.
    ///
    /// * `include_year` - If the year of the month should be included in the display
    pub fn _print_current_month(include_year: bool) -> Result<()> {
        let today = Local::now();
        Self::print_month_highlight_day(
            today.year(),
            today.month() as i32,
            today.day() as i32,
            include_year,
            0,
        )
    }

    /// Display the given month as a calendar.
    ///
    /// * `year` - Year of month to display
    /// * `month` - Month to display
    /// * `day` - Day to highlight on display, set to `0` to not highlight anything
    /// * `include_year` - If the year of the month should be included in the display
    pub fn print_month_highlight_day(
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

    /// Return the number of days in a given month
    ///
    /// * `year` - Year of month
    /// * `month` - Month
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        let next_month = if month == 12 { 1 } else { month + 1 };
        NaiveDate::from_ymd_opt(year, next_month, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    }

    /// Returns the integer corresponding to the given `chrono::Weekday`
    /// where `Weekday::Sun` = `0`.
    ///
    /// * `weekday` - `chrono::Weekday` to get the number of
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
