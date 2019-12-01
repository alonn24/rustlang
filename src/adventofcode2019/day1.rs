use std::fs;
use std::iter::Iterator;

fn mass_to_fuel(mass: i32) -> i32 {
  (mass / 3) - 2
}

fn calculate_fuel_recursive(mass: i32) -> i32 {
  let fuel = mass_to_fuel(mass);
  if fuel <= 0 {
    0
  } else {
    fuel + calculate_fuel_recursive(fuel)
  }
}

pub fn input() -> Vec<i32> {
  let contents = fs::read_to_string("./src/adventofcode2019/day1.input")
    .expect("Something went wrong reading the file");
  contents.split("\n").map(|s| s.parse().unwrap()).collect()
}

// PART 1
pub fn part1(input: Vec<i32>) -> i32 {
  input.iter().fold(0, |acc, mass| acc + mass_to_fuel(*mass))
}

#[cfg(test)]
mod part1_tests {
  use super::{part1};

  #[test]
  fn calculate_fuel_of_12() {
    assert_eq!(2, part1(vec![12]));
  }

  #[test]
  fn calculate_fuel_of_100756() {
    assert_eq!(33583, part1(vec![100756]));
  }
}

// PART 2
pub fn part2(input: Vec<i32>) -> i32 {
  // NOT 5040745, 5043715, 15129103
  input
    .iter()
    .fold(0, |acc, mass| acc + calculate_fuel_recursive(*mass))
}

#[cfg(test)]
mod part2_tests {
  use super::{part2};

  #[test]
  fn calculate_fuel_of_12() {
    assert_eq!(2, part2(vec![12]));
  }

  #[test]
  fn calculate_fuel_of_100756() {
    assert_eq!(50346, part2(vec![100756]));
  }
}