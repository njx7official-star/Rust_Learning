enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000_000..=999_999_999_999_999 => FileSize::Terabytes(size / 1_000_000_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64),
    }
}

fn rep_largest(s: u64) -> String {
    let files_in_bytes = match s {
        0..=1023 => format!("{} bytes", s),
        1024..=1_048_575 => format!("{:.2} KB", s as f64 / 1024 as f64),
        1_048_576..=1_073_741_823 => format!("{:.2} MB", s as f64 / 1_048_576 as f64),
        1_073_741_824..=1_099_511_627_774 => format!("{:.2} GB", s as f64 / 1_073_741_824 as f64),
        _ => format!("{:.2} TB", s as f64 / 1_099_511_627_776.0 as f64),
    };
    files_in_bytes
}

fn main() {
    let result = format_size(60000888837399);
    let largest = rep_largest(1_048_57611);
    println!("{}", result);
    println!("{}", largest);
}
