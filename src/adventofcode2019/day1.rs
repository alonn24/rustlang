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

pub fn part1(input: Vec<i32>) {
  let sum_of_fuels = input
    .iter()
    .fold(0, |acc, mass| acc + mass_to_fuel(*mass));
  println!("> Day1 part1 is: {}", sum_of_fuels);

  let total_sum_of_fuels = input
    .iter()
    .fold(0, |acc, mass| acc + calculate_fuel_recursive(*mass));
  println!("> Day1 part2 is: {}", total_sum_of_fuels);
  // NOT 5040745, 5043715, 15129103
}
