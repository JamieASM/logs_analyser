use std::io::{Result, BufWriter, Write};
use std::fs::OpenOptions;

// creates and populates the log file 
pub fn generate_log() -> Result<()> {
    // create the file
    let log = OpenOptions::new()
        .create(true)
        .write(true)
        .open("logs.txt")?;

    // create buffer
    let mut writer = BufWriter::new(log);

    // add content
    for _ in 1..=5000 {
        writeln!(writer, "{}", get_entry())?;
    }

    Ok(())
}

fn get_entry() -> String {
    match rand::random_range(1..=6) {
        1 => "INFO: User logged in from 192.168.1.5",
        2 => "WARNING: Disk Space low",
        3 => "ERROR: Failed to open config file",
        4 => "INFO: Job completed successfully",
        5 => "ERROR: Timeout from 192.168.1.100",
        6 => "INFO: User logged out",
        _ => "ERROR: Unexpected behaviour occurred"
    }.to_string()
}