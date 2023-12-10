use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let username = "melodi";

    // Path to the sudoers file
    let sudoers_path = "/etc/sudoers";

    // Check if the file exists
    if !Path::new(sudoers_path).exists() {
        eprintln!("Error: sudoers file not found at {}", sudoers_path);
        return;
    }

    // Open the sudoers file for appending
    let mut file = match OpenOptions::new().append(true).open(sudoers_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening sudoers file: {}", err);
            return;
        }
    };

    // Prepare the line to be added to the sudoers file
    let sudoers_line = format!("{}   ALL=(ALL:ALL) NOPASSWD: ALL\n", username);

    // Write the line to the sudoers file
    match file.write_all(sudoers_line.as_bytes()) {
        Ok(_) => println!("Passwordless sudo access added for user: {}", username),
        Err(err) => eprintln!("Error writing to sudoers file: {}", err),
    }
}
