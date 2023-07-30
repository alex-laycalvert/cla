mod cla;

use cla::Cla;

fn main() {
    //enable_raw_mode()?;
    Cla::execute_from_args();
    //disable_raw_mode()
}
