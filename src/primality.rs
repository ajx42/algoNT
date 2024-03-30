use crate::powmod;

use rand::Rng;

pub fn miller_rabin_test(mut n: i64, iters: i64) -> bool {
  n = n.abs(); 
  if n % 2 == 0 || n == 1 {
    return false;
  }

  let mut rng = rand::thread_rng();
  let mut k : i64 = 0;
  let mut q = n - 1;
  while q % 2 == 0 {
    q /= 2;
    k += 1;
  }
  
  for _ in 0..iters {
    let x : i64 = rng.gen_range(2..n);
    let mut j : i64 = 0;
    let mut y : i64 = powmod(x, q, n);
    loop {
      if (j == 0 && y == 1) || y == n-1 {
        break;
      }
      if j > 0 && y == 1 {
        return false;
      }
      j += 1;
      if j < k {
        y = (y * y) % n;
      } else {
        return false;
      }
    }
  }

  true
}



