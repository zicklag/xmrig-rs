//! Xmrig embedded as a library in Rust
//!
//! # Example
//!
//! ```
//! # use std::time::Duration;
//!
//! fn main() {
//!     println!("Starting Xmrig");
//!     // Spawn xmrig in another thread
//!     std::thread::spawn(|| {
//!         xmrig::start();
//!     });
//!     println!("Xmrig has started");
//!
//!     // Note: There is no way to stop xmrig other than exiting the program
//!
//!     // Let xmrig run for 5 seconds
//!     std::thread::sleep(Duration::from_secs(5));
//!     
//!     println!("Exiting now");
//!
//!     // When the main function exits, the thread running xmrig will be closed
//! }
//! ```

/// Start Xmrig exactly as if it was started on the commandline with no arguments
pub fn start() {
    unsafe {
        xmrig_sys::xmrig_start(0, std::ptr::null_mut());
    }
}
