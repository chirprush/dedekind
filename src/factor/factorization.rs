use std::ops::Mul;
use std::collections::HashMap;

use crate::factor::Prime;

// In some cases you also want the underlying u64 value associated with this,
// but other times you cannot store this in a u64 (and even in BigInteger types
// this would potentially be inefficient to store)
// Thus, we can probably just have a function that computes this (as slow as it
// may be), but then we still have size issues, so what should we do?
#[derive(Clone)]
pub struct PrimeFactorization {
    factors: HashMap<Prime, u64>
}

pub fn new(factors: HashMap<Prime, u64>) -> PrimeFactorization {
    PrimeFactorization { factors }
}

// Hmm I'm not really sure whether I want this to be a small object that is
// fine with clones or a large object that isn't
impl PrimeFactorization {
    // Maybe get a better name I'm not actually sure. This is a connotative name
    // from a math perspective I guess

    // Returns 0 if not in the factors (which is mathematically true)
    pub fn vp(&self, p: Prime) -> u64 {
        self.factors.get(&p).cloned().unwrap_or(0)
    }

    // A function to return the keys would be nice but idk if I want the hashmap
    // iterator. Often in math algorithms we have that
    // a) The number of distinct prime factors is not very large
    // b) We want the exact list of primes at a certain instant and then we may
    // modify the factorization later.
    // If we keep an iterator (even using copied) we run into lifetime issues which
    // are lowkey annoying.
    pub fn bases(&self) -> Vec<Prime> {
        self.factors.keys().copied().collect()
    }

    // We can also implement gcd, lcm, multiplication -> these could be implemented
    // outside once we have iterator over keys
    //
    // Should we extend this for negative exponents? Probably not here because we
    // have integer values
}

impl Mul for PrimeFactorization {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let pf = PrimeFactorization { factors: HashMap::new() };

        for p in self.bases() {
            factors.insert(p, factors.get(&p).unwrap_or(0) + self.vp(p));
        }

        for p in rhs.bases() {
            factors.insert(p, factors.get(&p).unwrap_or(0) + rhs.vp(p));
        }

        PrimeFactorization { factors }
    }
}
