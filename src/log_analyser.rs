use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_log(log: File) -> (HashMap<String, i32>, i32) {
    // set up hashmap
    let levels = vec!["ERROR", "INFO", "WARNING"];
    let mut data = HashMap::new();
    let mut total_lines = 0;

    // set up reader
    let reader = BufReader::new(log);

    // read each line
    for line in reader.lines() {
        let line: String = match line {
            Ok(val) => val,
            Err(e) => {
                println!("Error: {}", e);
                return (HashMap::new(), 0);
            }
        };

        // decide which level type we have and increment
        for level in &levels {
            if line.contains(level) {
                data
                    .entry(level.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        // increment line count
        total_lines += 1;
    }

    (data, total_lines)
}