pub fn part1(input: &Vec<Vec<char>>) -> u32 {
  total_overlap_priority(input)
}

pub fn part2(input: &Vec<Vec<char>>) -> u32 {
  total_group_badge_priority(input)
}

fn total_group_badge_priority(sacks: &Vec<Vec<char>>) -> u32 {
  if sacks.len() % 3 != 0 {
    panic!("Total sacks {} not a multiple of 3", sacks.len());
  }
  let mut s: u32 = 0;
  for i in 0..(sacks.len() / 3) {
    let j = i * 3;
    s += overlap_char(&[&sacks[j], &sacks[j + 1], &sacks[j + 2]]) + 1;
  }
  s
}

fn total_overlap_priority(sacks: &Vec<Vec<char>>) -> u32 {
  sacks.iter().map(|x| overlap_middle(x) + 1).sum()
}

pub fn generator(input: &str) -> Vec<Vec<char>> {
  input.lines()
    .map(|x| {
      let c: Vec<char> = x.chars().collect();
      if c.len() % 2 != 0 { panic!("Expecting even number of characters") }
      c
    })
    .collect()
}

fn overlap_middle(fills: &Vec<char>) -> u32 {
  let (f, l) = fills.split_at(fills.len() / 2);
  overlap_char(&[f, l])
}

fn overlap_char(elems: &[&[char]]) -> u32 {
  let mut prev: u64 = u64::MAX;
  for &elem in elems {
    let mut presence: u64 = 0;
    for c in elem {
      let cidx = char_idx(c);
      if (prev & (1 << cidx)) != 0 {
        presence |= 1 << cidx;
      }
    }
    prev = presence
  }
  if prev == 0 {
    return 0;
  } else {
    for s in 0..52 {
      if prev == 1 << s {
        return s;
      }
    }
  }
  panic!("Unexpected overlap {} for {:?}", prev, elems);
}

fn char_idx(c: &char) -> u32 {
  let v = c.to_ascii_lowercase() as u32 - 'a' as u32;
  if c.is_uppercase() {
    v + 26
  } else {
    v
  }
}

#[cfg(test)]
mod tests {
  use crate::day3::{char_idx, generator, overlap_middle, total_group_badge_priority, total_overlap_priority};

  fn input() -> String {
    vec![
      "vJrwpWtwJgWrhcsFMMfFFhFp",
      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
      "PmmdzqPrVvPwwTWBwg",
      "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
      "ttgJtRGJQctTZtZT",
      "CrZsJsPPZsGzwwsLwLmpwMDw",
    ].join("\n")
  }

  #[test]
  fn test_overlap_char() {
    assert_eq!(char_idx(&'p'), overlap_middle(&"vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect()));
    assert_eq!(char_idx(&'L'), overlap_middle(&"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect()));
    assert_eq!(char_idx(&'P'), overlap_middle(&"PmmdzqPrVvPwwTWBwg".chars().collect()));
    assert_eq!(char_idx(&'v'), overlap_middle(&"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".chars().collect()));
    assert_eq!(char_idx(&'t'), overlap_middle(&"ttgJtRGJQctTZtZT".chars().collect()));
    assert_eq!(char_idx(&'s'), overlap_middle(&"CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect()));
  }

  #[test]
  fn test_part1() {
    assert_eq!(157, total_overlap_priority(&generator(input().as_str())));
  }

  #[test]
  fn test_part2() {
    assert_eq!(70, total_group_badge_priority(&generator(input().as_str())));
  }
}