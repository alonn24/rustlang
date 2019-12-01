fn mass_to_fuel(mass: i32) -> i32 {
  (mass / 3) - 2
}

pub fn part1(input: Vec<i32>) -> i32 {
  input.iter().fold(0, |acc, mass| acc + mass_to_fuel(*mass))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_12() {
    assert_eq!(2, mass_to_fuel(12));
  }

  #[test]
  fn calculate_100756() {
    assert_eq!(33583, mass_to_fuel(100756));
  }
}