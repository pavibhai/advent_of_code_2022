pub fn part1(elfs: &Vec<i32>) -> i32 {
  find_max_calories(elfs)
}

pub fn part2(elfs: &Vec<i32>) -> i32 {
  find_top_n_calories(elfs, 3)
}

pub fn generator(input: &str) -> Vec<i32> {
  input.split("\n\n")
    .map(|x|
      x.lines()
        .map(|x| x.parse::<i32>().expect("Expecting number"))
        .sum()).collect()
}

fn find_max_calories(elfs: &Vec<i32>) -> i32 {
  **elfs.iter().max().get_or_insert(&0)
}

fn find_top_n_calories(elfs: &Vec<i32>, n: usize) -> i32 {
  let mut stack = Vec::new();
  while stack.len() < n {
    stack.push(i32::MIN);
  }
  let mut stack_min = i32::MIN;

  for e in elfs {
    if e < &stack_min {
      continue;
    }
    let mut new_min = i32::MAX;
    let mut pending = true;
    for s in stack.iter_mut() {
      if s == &stack_min && pending {
        *s = *e;
        pending = false;
      }
      new_min = new_min.min(*s);
    }
    stack_min = new_min;
  }
  stack.iter().sum()
}

#[cfg(test)]
mod tests {
  use crate::day1::{find_max_calories, find_top_n_calories, generator};

  fn input() -> String {
    vec![
      "1000",
      "2000",
      "3000",
      "",
      "4000",
      "",
      "5000",
      "6000",
      "",
      "7000",
      "8000",
      "9000",
      "",
      "10000",
    ].join("\n")
  }

  #[test]
  fn test_max_calories() {
    let e = generator(input().as_str());
    assert_eq!(5, e.len());
    assert_eq!(24000, find_max_calories(&e));
  }

  #[test]
  fn test_top_n_calories() {
    let e = generator(input().as_str());
    assert_eq!(45000, find_top_n_calories(&e, 3))
  }
}