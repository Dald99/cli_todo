use std::fs::File;
use std::io;
use std::io::{Read, Write};
use crate::tasks::Tasks;

pub fn save_file(tasks: &Tasks, file: &str) -> Result<(), io::Error> {
    let mut file = File::create(file)?;
    let json_file = serde_json::to_string(tasks)?;
    file.write_all(json_file.as_bytes())?;

    Ok(())
}

pub fn load_file(file: &str) -> Result<Tasks, io::Error> {
    let mut file = File::open(file)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    let tasks: Tasks = serde_json::from_str(&json_data)?;

    Ok(tasks)
}
