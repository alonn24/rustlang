use std::fs::File;
use std::io::prelude::*;

mod day1;
pub fn day1() -> std::io::Result<()> {
  let mut file = File::open("./src/adventofcode2019/day1.input")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let vec = contents.split("\n").map(|s| s.parse().unwrap()).collect();
  day1::part1(vec);
  Ok(())
}