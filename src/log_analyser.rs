use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_log(log: File) -> HashMap<String, i32> {
    // set up hashmap
    let mut data = HashMap::new();
    data.insert("INFO".to_string(), 0);
    data.insert("WARNING".to_string(), 0);
    data.insert("ERROR".to_string(), 0);
    data.insert("Total Lines".to_string(), 0);

    // set up reader
    let reader = BufReader::new(log);

    // read each line
    for line in reader.lines() {
        let line: String = match line {
            Ok(val) => val,
            Err(e) => {
                println!("Error: {}", e);
                return HashMap::new();
            }
        };

        if line.contains("INFO") {
            increment(&mut data, "INFO".to_string());
        }
        else if line.contains("WARNING") {
            increment(&mut data, "WARNING".to_string());
        }
        else if line.contains("ERROR") {
            increment(&mut data, "ERROR".to_string());
        }

        // increment line count
        increment(&mut data, "Total Lines".to_string());
    }

    return HashMap::new();
}

fn increment(data: &mut HashMap<String, i32>, key: String) {
    let count = data.get(&key).unwrap();
    data.insert(key, count + 1);
}