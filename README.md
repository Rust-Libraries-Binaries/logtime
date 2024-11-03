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

If you find this library helpful, consider supporting my open-source efforts: https://www.paypal.com/donate/?business=QMSZ2E6L75BYN&no_recurring=0&item_name=If+my+Rust+libraries+have+added+value+to+your+projects%2C+consider+a+small+donation+to+help+me+continue.+Thank+you%21&currency_code=USD
