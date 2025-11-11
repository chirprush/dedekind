pub fn integer_sqrt(n: u64) -> u64 {
    let close = (n as f64).sqrt() as u64;

    // Let q be the integer square root of n
    // Due to rounding errors, close is either q or q+1

    if close * close > n { return close - 1 }
    return close;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(integer_sqrt(0), 0);
        assert_eq!(integer_sqrt(1), 1);
        assert_eq!(integer_sqrt(2), 1);
        assert_eq!(integer_sqrt(3), 1);
        assert_eq!(integer_sqrt(4), 2);
        assert_eq!(integer_sqrt(5), 2);
        assert_eq!(integer_sqrt(6), 2);
        assert_eq!(integer_sqrt(7), 2);
    }

    #[test]
    fn test_before_squares() {
        let mut state: u64 = 639907;
        let modulus: u64 = 1 << 32 - 2;

        for _ in 0..1000 {
            let i: u64 = ((state * 893051) % modulus + 963793) % modulus;

            assert_eq!(integer_sqrt((i + 1) * (i + 1) - 1), i);
            state = i;
        }
    }
}