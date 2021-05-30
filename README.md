# xmrig

[![Crates.io](https://img.shields.io/crates/v/xmrig.svg)](https://crates.io/crates/xmrig)
[![Docs.rs](https://docs.rs/xmrig/badge.svg)](https://docs.rs/xmrig)
[![Build Status](https://github.com/katharostech/bevy_retro/actions/workflows/rust.yaml/badge.svg)](https://github.com/katharostech/bevy_retro/actions/workflows/rust.yaml)
[![lines of code](https://tokei.rs/b1/github/katharostech/xmrig?category=code)](https://github.com/katharostech/xmrig)
[![Katharos License](https://img.shields.io/badge/License-Katharos-blue)](https://github.com/katharostech/katharos-license)

Xmrig embedded as a library in Rust


## Example


```rust

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
```




