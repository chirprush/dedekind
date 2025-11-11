// TODO: Maybe make this more generic with traits
// I think one could imagine having sieves that are specialized for certain
// tasks (like for example the prime counting function doesn't actually need all
// the space).
// That being said, what if you want to combine the sieves? That seems difficult
// to program very generally if you want to combine efficiency.

use crate::factor::Prime;
use crate::util::integer_sqrt;

pub struct LinearSieveResult {
    left: u64,
    right: u64,
    lpf: Vec<Prime>,
    primes: Vec<Prime>
}

impl LinearSieveResult {
    fn in_bounds(&self, n: u64) -> bool {
        self.left <= n && n <= self.right
    }

    pub fn left(&self) -> u64 {
        self.left
    }
    
    pub fn right(&self) -> u64 {
        self.right
    }

    pub fn lpf(&self, n: u64) -> Result<Prime, ()> {
        if !self.in_bounds(n) { return Err(()); }

        return Ok(self.lpf[(n - self.left) as usize]);
    }

    pub fn is_prime(&self, n: u64) -> Result<bool, ()> {
        if !self.in_bounds(n) { return Err(()); }

        return Ok(self.lpf[(n - self.left) as usize].value() == n);
    }

    pub fn primes(&self) -> &Vec<Prime> {
        &self.primes
    }
}

pub struct LinearSieve {
    left: u64,
    right: u64
}

impl LinearSieve {
    pub fn new(left: u64, right: u64) -> Result<LinearSieve, ()> {
        if left < 2 || left > right { return Err(()); }

        return Ok(LinearSieve { left, right });
    }

    pub fn run(&self) -> LinearSieveResult {
        let mut lpf: Vec<u64> = vec![0; (self.right - self.left + 1) as usize];
        let mut primes: Vec<Prime> = Vec::new();

        let small_bound = integer_sqrt(self.right);

        let mut is_small_prime: Vec<bool> = vec![true; (small_bound + 1) as usize];

        for p in 2..=small_bound {
            if !is_small_prime[p as usize] { continue; }
            // else if is_small_prime[p as usize] && p >= self.left {
            //     primes.push(Prime::new_unchecked(p));
            // }

            for j in 2..=small_bound / p {
                is_small_prime[(j * p) as usize] = false;
            }

            let start = if self.left % p == 0 {
                self.left / p
            } else {
                self.left / p + 1
            };
            let end = self.right / p;

            for j in start..=end {
                if lpf[(j * p - self.left) as usize] == 0 {
                    lpf[(j * p - self.left) as usize] = p;
                }
            }
        }

        for p in self.left..=self.right {
            let least = lpf[(p - self.left) as usize];
            if least == 0 || least == p {
                lpf[(p - self.left) as usize] = p;
                primes.push(Prime::new_unchecked(p));
            }
        }

        LinearSieveResult {
            left: self.left,
            right: self.right,
            lpf: lpf
                    .into_iter()
                    .map(Prime::new_unchecked)
                    .collect(),
            primes
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invariant() {
        assert!(LinearSieve::new(0, 100).is_err());
        assert!(LinearSieve::new(1, 100).is_err());
        assert!(LinearSieve::new(100, 2).is_err());
        assert!(LinearSieve::new(2, 100).is_ok())
    }

    #[test]
    fn test_small_unshifted() {
        let sieve = LinearSieve::new(2, 10).unwrap();
        let result = sieve.run();

        assert_eq!(result.left(), 2);
        assert_eq!(result.right(), 10);
        assert_eq!(result.primes().len(), 4);
        assert_eq!(result.primes()[0].value(), 2);
        assert_eq!(result.primes()[1].value(), 3);
        assert_eq!(result.primes()[2].value(), 5);
        assert_eq!(result.primes()[3].value(), 7);
        
        for p in result.primes() {
            assert_eq!(result.lpf(p.value()).unwrap(), *p);
        }

        assert_eq!(result.lpf(9).unwrap().value(), 3);
        assert_eq!(result.lpf(10).unwrap().value(), 2);
    }

    #[test]
    fn test_large_unshifted() {
        let sieve = LinearSieve::new(2, 317 * 317).unwrap();
        let result = sieve.run();

        assert_eq!(result.left(), 2);
        assert_eq!(result.right(), 100489);
        assert_eq!(result.primes().len(), 9631);
        assert_eq!(result.primes()[0].value(), 2);
        assert_eq!(result.primes()[1].value(), 3);
        assert_eq!(result.primes()[2].value(), 5);
        assert_eq!(result.primes()[3].value(), 7);
        
        for p in result.primes() {
            assert_eq!(result.lpf(p.value()).unwrap(), *p);
        }

        assert_eq!(result.lpf(9).unwrap().value(), 3);
        assert_eq!(result.lpf(1027).unwrap().value(), 13);
        assert_eq!(result.lpf(317).unwrap().value(), 317);
        assert_eq!(result.lpf(317*23).unwrap().value(), 23);
        assert_eq!(result.lpf(317*317).unwrap().value(), 317);

        assert!(result.is_prime(67).unwrap());
    }

    #[test]
    fn test_large_shifted() {
        let sieve = LinearSieve::new(1000000000, 1000000000 + 100000).unwrap();
        let result = sieve.run();

        assert_eq!(result.left(), 1e9 as u64);
        assert_eq!(result.right(), (1e9 + 1e5) as u64);
        assert_eq!(result.primes().len(), 4832);

        assert_eq!(result.primes()[0].value(), 1e9 as u64 + 7);
        assert_eq!(result.primes()[1].value(), 1e9 as u64 + 9);
        assert_eq!(result.primes()[2].value(), 1e9 as u64 + 21);
        assert_eq!(result.primes()[3].value(), 1e9 as u64 + 33);

        for p in result.primes() {
            assert_eq!(result.lpf(p.value()).unwrap(), *p);
        }

        assert!(result.lpf(9).is_err());
        assert!(result.lpf(1000000000 - 1).is_err());
        assert!(result.lpf(1000000000).is_ok());
        assert!(result.is_prime(1e9 as u64 + 7).unwrap());
    }
}