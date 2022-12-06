use regex::Regex;

pub fn generator(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
  let mut s = input.split("\n\n");
  let s1 = s.next().unwrap();
  let s2 = s.next().unwrap();
  if s.next().is_some() {
    panic!("Unexpected sections")
  }
  (mk_stacks(s1), mk_moves(s2))
}

pub fn part1(input: &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
  let mut stacks = input.0.clone();
  for (times, from, to) in input.1.iter() {
    for _ in 0..*times {
      if let Some(c) = stacks.get_mut(*from).unwrap().pop() {
       stacks.get_mut(*to).unwrap().push(c);
      } else {
        panic!("Unexpected, try to move out of empty stack");
      }
    }
  }
  stacks.iter().map(|s|s.last().unwrap()).collect()
}

pub fn part2(input: &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
  let mut stacks = input.0.clone();
  let mut buff: Vec<char> = Vec::new();
  for (times, from, to) in input.1.iter() {
    for _ in 0..*times {
      if let Some(c) = stacks.get_mut(*from).unwrap().pop() {
        buff.push(c);
      } else {
        panic!("Unexpected, try to move out of empty stack");
      }
    }
    for _ in 0..*times {
      stacks.get_mut(*to).unwrap().push(buff.pop().unwrap());
    }
  }
  stacks.iter().map(|s|s.last().unwrap()).collect()
}

fn mk_stacks(input: &str) -> Vec<Vec<char>> {
  let mut lines = input.lines().rev();
  let mut stacks: Vec<Vec<char>> = Vec::new();
  for _ in 0..lines.next().unwrap().split_whitespace().count() {
    stacks.push(Vec::new());
  }

  for line in lines {
    let mut sidx = 0;
    let mut chars = line.chars();
    while sidx < stacks.len() && chars.next().is_some() {
      match chars.next().unwrap() {
        v if v.is_ascii_alphabetic() => stacks[sidx].push(v),
        v if v.is_whitespace() => {}
        v => panic!("Unexpected char {}", v)
      }
      sidx += 1;
      chars.next();
      chars.next();
    }
  }
  stacks
}

fn mk_moves(input: &str) -> Vec<(usize, usize, usize)> {
  let move_line = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
  let lines = input.lines();
  lines.map(|l| {
    let entry = move_line.captures(l).unwrap();
    (entry.get(1).unwrap().as_str().parse::<usize>().expect("Expecting a number"),
     entry.get(2).unwrap().as_str().parse::<usize>().expect("Expecting a number") - 1,
     entry.get(3).unwrap().as_str().parse::<usize>().expect("Expecting a number") - 1)
  }).collect()
}

#[cfg(test)]
mod tests {
  use crate::day5::{generator, part1, part2};

  fn input() -> String {
    vec![
      "    [D]",
      "[N] [C]",
      "[Z] [M] [P]",
      " 1   2   3",
      "",
      "move 1 from 2 to 1",
      "move 3 from 1 to 3",
      "move 2 from 2 to 1",
      "move 1 from 1 to 2",
    ].join("\n")
  }

  #[test]
  fn test_stacks() {
    let (stacks, moves) = generator(input().as_str());
    assert_eq!(3, stacks.len());
    assert_eq!(&'N', stacks[0].last().unwrap());
    assert_eq!(&'D', stacks[1].last().unwrap());
    assert_eq!(&'P', stacks[2].last().unwrap());

    assert_eq!(4, moves.len());
    assert_eq!(&(1, 1, 0), moves.first().unwrap());
    assert_eq!(&(1, 0, 1), moves.last().unwrap());
  }

  #[test]
  fn test_part1() {
    let result = part1(&mut generator(input().as_str()));
    assert_eq!("CMZ", result);
  }

  #[test]
  fn test_part2() {
    let result = part2(&mut generator(input().as_str()));
    assert_eq!("MCD", result);
  }
}