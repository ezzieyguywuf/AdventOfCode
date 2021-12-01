mod runners;

use std::io;

fn main() -> io::Result<()> {
    runners::run_day_01()?;

    Ok(())
}
