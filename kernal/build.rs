use std::thread;
use std::time::Duration;

fn main() {
    // Only sleep during build/run, not during check
    println!("cargo:rerun-if-changed=build.rs");
    
    // Pause for 0.5 seconds (500 milliseconds)
    thread::sleep(Duration::from_millis(500));
}
