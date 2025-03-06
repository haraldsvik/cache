use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Write;

fn generate_random_number(length: usize) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect()
}

fn main() {
    let mut file = File::create("mock_data.txt").expect("Failed to create file");
    let mut rng = thread_rng();

    // Generate 1000 random key-value pairs
    for _ in 0..1000 {
        // Generate key length between 5 and 11 digits
        let key_length = rng.gen_range(5..=11);
        let key = generate_random_number(key_length);
        
        // Generate value (14 digits)
        let value = generate_random_number(14);
        
        // Write to file
        writeln!(file, "{}:{}", key, value).expect("Failed to write to file");
    }

    println!("Generated mock_data.txt with 1000 numeric key-value pairs");
} 