pub fn gcd(mut a: i64, mut b: i64) -> i64 {
  while b != 0 {
    let tmp = b;
    b = a % b;
    a = tmp;
  }
  a
}

pub fn powmod(mut x: i64, mut y: i64, md: i64) -> i64 {
  let mut ans : i64 = 1;
  while y > 0 {
    if y & 1 > 0 {
      ans = (ans * x) % md;
    }
    x = (x * x) % md;
    y >>= 1;
  }
  ans
}
