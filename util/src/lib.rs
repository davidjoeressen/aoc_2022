use std::env;
use std::fs;
use std::io;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn execute<F>(f: F)
where
    F: Fn(&str),
{
    for file_name in env::args().skip(1) {
        if let Err(error) = execute_file(&file_name, &f) {
            println!("Error reading file {}: {}", file_name, error);
        }
    }
}

fn execute_file<F>(file_name: &str, f: &F) -> io::Result<()>
where
    F: Fn(&str),
{
    let file: String = fs::read_to_string(file_name)?;
    f(&file);
    Ok(())
}
