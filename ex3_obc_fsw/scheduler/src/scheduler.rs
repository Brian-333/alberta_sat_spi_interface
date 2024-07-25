use std::fs;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Write;
use std::time::SystemTime;
use std::io::BufRead;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::{log_info, log_error};

pub fn process_saved_messages(dir: &str, curr_time_millis: u64, already_read: &Arc<Mutex<HashSet<String>>>) {
    let saved_messages_dir = Path::new(dir);
    if saved_messages_dir.exists() && saved_messages_dir.is_dir() {
        match fs::read_dir(saved_messages_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    process_entry(entry, curr_time_millis, already_read);
                }
            }
            Err(e) => eprintln!("Error reading directory: {:?}", e),
        }
    } else {
        log_error("Directory does not exist or is not a directory.".to_string(), 54);
    }
}

fn process_entry(entry: fs::DirEntry, curr_time_millis: u64, already_read: &Arc<Mutex<HashSet<String>>>) {
    if let Some(file_name) = entry.file_name().to_str() {
        let file_name = file_name.to_string();
        let file_path = entry.path();
        if file_name.ends_with(".txt") {
            if let Ok(file_time) = file_name.trim_end_matches(".txt").parse::<u64>() {
                if file_time <= curr_time_millis {
                    let mut already_read_set = already_read.lock().unwrap();
                    if !already_read_set.contains(&file_name) {
                        already_read_set.insert(file_name.clone());
                        drop(already_read_set); // Release the lock before calling send_message
                        send_message(&file_path, &file_name);
                    }
                }
            }
        }
    }
}

fn send_message(file_path: &Path, file_name: &str) {
    match fs::File::open(file_path) {
        Ok(file) => {
            let lines: Vec<String> = io::BufReader::new(file)
                .lines()
                .filter_map(Result::ok)
                .collect();
            if lines.len() > 1 {
                println!("Sent Message ID {} at {}", lines[1], file_name);
            } else {
                println!("File {} does not have an ID.", file_name);
            }
            log_info(format!("Processed file: {}", file_name), 0); // Message ID can be managed as needed
        }
        Err(e) => eprintln!("Failed to open file {}: {:?}", file_name, e),
    }
}

pub fn get_current_time_millis() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(e) => {
            eprint!("Error {:?}", e);
            0
        }
    }
}

pub fn write_input_tuple_to_rolling_file(input_tuple: &(u64, u8)) -> Result<(), io::Error> {
    // Create the directory if it doesn't exist
    let dir_path = "scheduler/saved_messages";
    fs::create_dir_all(dir_path)?;

    // Get the total size of files in the directory
    let total_size: u64 = fs::read_dir(dir_path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.metadata().ok().map(|m| m.len()).unwrap_or(0))
        .sum();

    // Specify the maximum size of saved_messages directory in bytes
    let max_size_bytes: u64 = 2048; // 2 KB

    // If the total size exceeds the maximum size, remove the oldest file
    if total_size >= max_size_bytes {
        remove_oldest_file(&dir_path)?;
    }

    // Create a new file
    let file_name = format!("{}.txt", input_tuple.0);
    let file_path = Path::new(dir_path).join(&file_name);
    let mut file = File::create(&file_path)?;

    // Write input_tuple to the file
    writeln!(file, "{:?}\n{}", input_tuple.0, input_tuple.1)?;

    Ok(())
}

fn remove_oldest_file(dir_path: &str) -> Result<(), io::Error> {
    let oldest_file = fs::read_dir(dir_path)?
        .filter_map(|res| res.ok())
        .min_by_key(|entry: &fs::DirEntry| entry.metadata().unwrap().modified().unwrap());

    if let Some(oldest_file) = oldest_file {
        fs::remove_file(oldest_file.path())?;
    }

    Ok(())
}