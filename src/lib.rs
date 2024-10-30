use std::time::{Duration, Instant};

/// Times the execution of a function and returns the elapsed `Duration`.
pub fn time_execution<F, T>(mut func: F) -> (T, Duration)
where
    F: FnMut() -> T,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (result, duration)
}

/// Logs the elapsed time in a human-readable format (e.g., "Execution took 123ms").
pub fn log_execution_time<F, T>(func_name: &str, func: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    println!("Execution of '{}' took {:?}", func_name, duration);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_time_execution() {
        let (_, duration) = time_execution(|| {
            sleep(Duration::from_millis(100));
        });
        
        // Assert that the duration is at least 100 milliseconds
        assert!(duration >= Duration::from_millis(100));
    }

    #[test]
    fn test_log_execution_time() {
        let result = log_execution_time("test_sleep", || {
            sleep(Duration::from_millis(50));
            42
        });
        
        // Check that the result is as expected
        assert_eq!(result, 42);
    }
}