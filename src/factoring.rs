use log::{info, warn};
use crate::{miller_rabin_test, gcd};

// probabilistic: may fail, returns an empty vector and prints a warning
// returned factors may or may not be prime
// this algorithm may need to be called repeatedly to get complete factorisation to primes
pub fn pollard_rho_factorisation(mut n: i64, next: Option<fn(i64) -> i64 >) -> Vec<i64>
{
  let next_fn : fn(i64) -> i64 = next.unwrap_or(|x|  x*x + 1);
  let mut x : i64 = 5;      // x_m
  let mut x_p : i64 = 2;    // x_{l(m) - 1}
  let mut k : i64 = 1;      // k = 2l - m (we want m to be smaller than next l)
  let mut l : i64 = 1;      // largest power of 2 smaller than m

  if miller_rabin_test(n, 25) {
    return vec![n];
  }

  let mut factors : Vec<i64> = vec![];
  
  // Ideally we should test primality, but that is not implemented
  loop {
    if miller_rabin_test(n, 25) {
        info!("Found new factor: {}", n);
        factors.push(n);
        return factors;
    }

    let g = gcd((x_p-x).abs(), n);
    if g == 1 {
      k = k - 1;
      if k == 0 { // we are done with this l
        info!("Got reset: l={}", 2*l);
        x_p = x;
        l = 2*l;
        k = l;
      }
      x = next_fn(x) % n;
    } else {
      if g == n {
        warn!("Pollard Rho Factorisation has failed for {}", n);
        return vec![];
      } else {
        info!("Found new factor: {}", g);
        factors.push(g);
        n = n/g;
        x = x % n;
        x_p = x_p % n;
      }
    }
  }
}
