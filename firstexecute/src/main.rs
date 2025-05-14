use std::process::Command;

fn main() {
    println!("Executing ls command...");
    
    let output = Command::new("ls")
        .output()
        .expect("Failed to execute command");
        
    println!("Command output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
