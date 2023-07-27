mod calendar;

use calendar::Calendar;
use crossterm::Result;

fn main() -> Result<()> {
    Calendar::print_current_month()
}
