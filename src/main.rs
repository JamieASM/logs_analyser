mod log_generator;
mod log_analyser;

use std::io::Result;
use std::fs::File;

fn main() -> Result<()> {
    // create the log file
    log_generator::generate_log()?;

    // analyse the log file
    // open the log 
    let data = log_analyser::read_log(File::open("log.txt")?);

    // display data
    println!("--- Analysed Data ---");

    // display line count
    println!("\nTotal Lines: {:?}", data.get("Total Lines"));

    // display log level counts
    println!("\nLog level counts:");
    for (key, val) in data {
        // avoiding printing the line count again
        if key != "Total Lines".to_string() {
            println!("- {:?}: {:?}", key, val);
        }
    }

    Ok(())
}
