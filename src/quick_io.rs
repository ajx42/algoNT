use std::io;
use std::str::FromStr;
use std::fmt::Debug;

pub fn read_one<T>() -> T 
    where T: FromStr,
          <T as FromStr>::Err: Debug
{
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
  buffer.trim().parse().unwrap()
}
 
pub fn read_vec<T>() -> Vec<T>
    where T: FromStr,
          <T as FromStr>::Err: Debug
{
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
  buffer.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

