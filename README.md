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
Ben Santora (<bensatlantik@gmail.com>)