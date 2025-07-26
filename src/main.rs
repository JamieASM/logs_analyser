mod log_generator;
mod log_analyser;

use std::io::Result;
use std::fs::File;

fn main() -> Result<()> {
    // create the log file
    log_generator::generate_log()?;

    // analyse the log file
    // open the log 
    let (data, total_lines) = log_analyser::read_log(File::open("log.txt")?);

    // display data
    println!("--- Analysed Data ---");

    // display line count
    println!("\nTotal Lines: {}", total_lines);

    // display log level counts
    println!("\nLog level counts:");
    for (key, val) in &data {
        // avoiding printing the line count again
        println!("- {:?}: {:?}", key, val);
    }

    // display the most common level
    if let Some((key, val)) = data.iter().max_by_key(|entry|  entry.1) {
        println!("\nMost common level: {} ({} times)", key, val);
    }

    Ok(())
}
