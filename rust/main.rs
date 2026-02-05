use std::time::Instant;
const MAX: i64 = 3_00_000;

fn count_primes(limit: i64) -> i64 {
    let mut count = 0;
    let mut is_prime;
    for c in 2..limit {
        is_prime = true;
        for inner_c in 2..(c / 2 + 1) {
            if c % inner_c == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
        }
    }
    count
}

fn main() {
    let start_time = Instant::now();
    let count = count_primes(MAX);
    let end_time = start_time.elapsed();
    println!("{} - {:?}", count, end_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_primes() {
        assert_eq!(count_primes(10), 4);
        assert_eq!(count_primes(20), 8);
        assert_eq!(count_primes(100), 25);
    }
}
