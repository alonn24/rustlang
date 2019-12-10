use std::collections::HashMap;

// Conver that key orbits value
pub fn convert_input(input: String) -> HashMap<String, String> {
  let mut res: HashMap<String, String> = HashMap::new();
  let lines: Vec<&str> = input.split("\n").collect();
  for line in lines.iter() {
    let parts: Vec<&str> = line.split(")").collect();
    res.insert(String::from(parts[1]), String::from(parts[0]));
  }
  res
}

fn get_orbits(input: &HashMap<String, String>, key: &String) -> i32 {
  if !input.contains_key(key) {
    return 0;
  }
  1 + get_orbits(input, input.get(key).expect("asd"))
}

pub fn part1(input: HashMap<String, String>) -> i32 {
  let mut count = 0;
  for key in input.keys() {
    count += get_orbits(&input, key);
  }
  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let input =
      convert_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_string());
    assert_eq!(part1(input), 42);
  }
}
