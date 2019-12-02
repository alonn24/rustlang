fn get_swap_value(input: &Vec<usize>, i: usize) -> usize {
  match input[i] {
    1 => input[input[i + 1]] + input[input[i + 2]],
    2 => input[input[i + 1]] * input[input[i + 2]],
    _ => panic!("Uh!")
  }
}
// NOT 103, 89
pub fn part1(input: Vec<usize>) -> Vec<usize> {
  let mut result = input.to_vec();
  let mut i = 0;
  loop {
    if i >= result.len() || result[i] == 99 {
      break;
    };
    let output_i = result[i + 3];
    result[output_i] = get_swap_value(&result, i);
    i += 4;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_12() {
    assert_eq!(vec![2,0,0,0,99], part1(vec![1, 0, 0, 0, 99]));
    assert_eq!(vec![2,3,0,6,99], part1(vec![2,3,0,3,99]));
  }
}
