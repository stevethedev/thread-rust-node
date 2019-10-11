/// Create `n` counters that count to `m`. Returns 0 if all threads return
/// successfully, or else -1.
///
/// ```c
/// int create_counters(unsigned int m, unsigned int n);
/// ```
#[no_mangle]
pub extern "C" fn create_counters(m: u32, n: u32) -> i32 {
    println!("Spawning {} threads", m);
    println!("Each thread counts to {}", n);

    if (0..m)
        .map(|i| {
            println!("Thread #{} spawned", i);

            std::thread::spawn(move || {
                for j in 0..n {
                    println!("Thread #{} -> {}", i, j);
                    std::thread::sleep(std::time::Duration::from_micros(1));
                }
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|joiner| joiner.join().err())
        .collect::<Vec<_>>()
        .is_empty()
    {
        0
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_threads() {
        assert_eq!(create_counters(5, 100), 0);
    }
}
