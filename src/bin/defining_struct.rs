#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    email: String,
    ph_no: u64,
    age: u8,
}

fn main() {
    println!(
        "{:?}",
        Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "nirajv.official@gmail.com".to_string(),
            ph_no: 9526256200,
            age: 25,
        }
    );
}
