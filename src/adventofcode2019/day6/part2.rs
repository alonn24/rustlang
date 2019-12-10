use std::collections::HashMap;

fn get_orbits(input: &HashMap<String, String>, key: &String) -> Vec<String> {
  if !input.contains_key(key) {
    return vec![];
  }
  let mut vec = get_orbits(input, input.get(key).expect("asd"));
  vec.push(String::from(key));
  vec
}

fn get_shared(vec1: &Vec<String>, vec2: &Vec<String>) -> usize {
  let mut shared = 0;
  loop {
    if vec1[shared] != vec2[shared] {
      break;
    }
    shared += 1;
  }
  shared
}

pub fn part2(input: HashMap<String, String>) -> i32 {
  let you_orbits: Vec<String> = get_orbits(&input, &String::from("YOU"));
  let san_orbits: Vec<String> = get_orbits(&input, &String::from("SAN"));
  let shared_index = get_shared(&you_orbits, &san_orbits);
  (you_orbits.len() + san_orbits.len() - 2 - shared_index * 2) as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let input = convert_input(
      "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN".to_string(),
    );
    assert_eq!(part2(input), 4);
  }
}
