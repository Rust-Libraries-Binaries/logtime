## All Rust libraries and binaries created by Ben Santora. All were coded using AI: ChatGPT - Widenex Rust GPT - Microsoft CoPilot

## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

## logtime

**Logtime** is a Rust library for measuring and optionally logging the execution time of code.

## Usage

Add **logtime** to your `Cargo.toml`:

```toml
[dependencies]
logtime = "0.1.0"
```
## Examples

```rust

use logtime::{time_execution, log_execution_time};

fn main() {
    let (_, duration) = time_execution(|| {
        // code to time
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
    println!("Code block took {:?}", duration);

    log_execution_time("sleep_200ms", || {
        std::thread::sleep(std::time::Duration::from_millis(200));
    });
}
```

## License
This project is licensed under the MIT License

## Author
Ben Santora 
