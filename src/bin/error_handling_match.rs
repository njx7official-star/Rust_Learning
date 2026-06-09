use std::error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
fn writer(file: File) {
    let mut writer = BufWriter::new(file);
    match writeln!(writer, "Nj new line added") {
        Ok(()) => {}
        Err(error) => {
            panic!("Error WRITING TO FILE : {}", error)
        }
    }
}
fn main() {
    let file = File::open("permission_denied_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
    let writer_file = OpenOptions::new()
        .write(true)
        .open("permission_denied_file.txt");
    match writer_file {
        Ok(file) => {
            println!("File Found and Opening");
            writer(file);
        }
        Err(error) => {
            panic!("Error while opening for editing {}", error);
        }
    }
}
