use std::time::Duration;

fn main() {
    println!("Starting Xmrig");
    // Spawn xmrig in another thread
    std::thread::spawn(|| {
        xmrig::start();
    });
    println!("Xmrig has started");

    // Note: There is no way to stop xmrig other than exiting the program

    // Let xmrig run for 5 seconds
    std::thread::sleep(Duration::from_secs(5));
    
    println!("Exiting now");

    // When the main function exits, the thread running xmrig will be closed
}
