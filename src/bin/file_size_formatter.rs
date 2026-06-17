use std::{env, f64};

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug, Default)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}
fn convert_size(cs: u64) -> Sizes {
    let sizes = Sizes {
        bytes: format!("{} bytes", cs),
        kilobytes: format!("{:.2} kilobytes", cs as f64 / 1000.0),
        megabytes: format!("{:.2} megabytes", cs as f64 / 1_000_000.0),
        gigabytes: format!("{:.2} gigabytes", cs as f64 / 1_000_000_000.0),
    };
    sizes
}
fn m_check(s: Vec<&str>) -> u64 {
    let s2 = s[0];
    if s[1] == "mb" || s[1] == "kb" || s[1] == "gb" || s[1] == "bytes" {
        let s1 = s[1];
        let bytes = match s1 {
            "kb" => s2.parse::<u64>().expect("er") * 1000,
            "mb" => s2.parse::<u64>().expect("er") * 1_000_000,
            "gb" => s2.parse::<u64>().expect("er") * 1_000_000_000,
            _ => s2.parse::<u64>().expect("er"),
        };
        println!("check : {:?}", bytes);
        bytes
    } else {
        panic!("Wrong Input");
    }
}
fn main() {
    let arg: Vec<String> = env::args().collect();
    let arg123: String = arg[1].parse().expect("Not now");
    let arg_string: Vec<&str> = arg123.split_whitespace().collect();
    println!("{:?}", arg_string[1]);
    let byte = m_check(arg_string);
    let converted = convert_size(byte);
    // let result = format_size(byte);
    println!("{:?}", converted);
}
