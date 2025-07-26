mod log_generator;
mod log_analyser;

use std::io::Result;

fn main() -> Result<()> {
    // create the log file
    log_generator::generate_log()?;

    // analyse the log file

    Ok(())
}
