use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;
use std::time::Instant;

fn main() {
    let start_time = Instant::now(); // Record start time

    let mut input_filename = String::new();
    let mut fdata = String::new();

    println!("Enter the input file name:");
    io::stdin().read_line(&mut input_filename).expect("Failed to read input");

    let input_filename = input_filename.trim().to_string();

    let mut input_file = File::open(&input_filename).expect("File not found");
    input_file.read_to_string(&mut fdata).expect("Failed to read file");

    println!("Read successfully");

    let output_filename = "output.json".to_string();
    let fdata_clone = fdata.clone();

    // Spawn a thread for writing to the output file
    let handle = thread::spawn(move || {
        write_to_file(&output_filename, &fdata_clone);
    });

    // Continue with other tasks in the main thread

    // Wait for the thread to finish
    handle.join().expect("Failed to join thread");

    let end_time = Instant::now(); // Record end time
    let runtime = end_time.duration_since(start_time); // Calculate runtime

    println!("Total runtime: {:?}", runtime);
}

fn write_to_file(filename: &str, data: &str) {
    // Replace ";" with ":" while writing to the output file
    let replaced_data = data.replace(";", ":");
    let mut output_file = File::create(filename).expect("Failed to create output file");
    output_file.write_all(replaced_data.as_bytes()).expect("Failed to write to output file");
    println!("Data written to {}", filename);
}
