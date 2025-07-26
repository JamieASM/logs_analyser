mod log_generator;
mod log_analyser;

use std::collections::HashMap;
use std::io::Result;
use std::fs::File;

fn main() -> Result<()> {
    // create the log file
    log_generator::generate_log()?;

    // analyse the log file
    // open the log 
    let data: HashMap<String, i32> = log_analyser::read_log(File::open("log.txt")?);

    // display data
    println!("--- Analysed Data ---");

    // display line count
    println!("\nTotal Lines: {:?}", data.get("Total Lines").unwrap());

    // display log level counts
    println!("\nLog level counts:");
    for (key, val) in &data {
        // avoiding printing the line count again
        if key != "Total Lines" {
            println!("- {:?}: {:?}", key, val);
        }
    }

    // display the most common level
    let most_common: (String, i32) = log_analyser::get_most_common(&data);
    println!("\nMost common level: {} ({} times)", most_common.0, most_common.1);

    Ok(())
}
