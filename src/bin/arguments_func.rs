use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("Failed Input");
    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|item| item.parse().expect("Invalid number"))
        .collect();

    let average = sum(&numbers) / numbers.len() as i32;
    let result = sum(&numbers);
    println!("The sum is {}", result);
    println!("The average is {}", average);
}
