use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

const NORTH: u32 = 0;
const SOUTH: u32 = 1;
const WEST: u32 = 2;
const EAST: u32 = 3;
const DIRECTION_COUNT: usize = 4;
const ELF_CHAR: char = '#';
const EMPTY_CHAR: char = '.';
const NORTH_BITS: u8 = 224;
const SOUTH_BITS: u8 = 7;
const WEST_BITS: u8 = 148;
const EAST_BITS: u8 = 41;

pub fn generator(input: &str) -> Elves {
  let mut layout: Vec<XY> = Vec::new();
  let mut y = 0;
  for line in input.lines() {
    for (x, c) in line.chars().enumerate() {
      if c == ELF_CHAR {
        layout.push(XY::new(x as i32, y));
      }
    }
    y += 1;
  }
  Elves { layout }
}

pub fn part1(elves: &Elves) -> usize {
  let mut elves = elves.clone();
  elves.perform_rounds(10);
  elves.empty_space_count()
}

pub fn part2(elves: &Elves) -> usize {
  elves.clone().run()
}

#[derive(Clone)]
pub struct Elves {
  layout: Vec<XY>,
}

impl Display for Elves {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let (x_range, y_range) = self.bounds();
    let mut output: String = String::new();
    for y in y_range.clone() {
      for x in x_range.clone() {
        if self.layout.contains(&XY::new(x, y)) {
          output.push(ELF_CHAR);
        } else {
          output.push(EMPTY_CHAR);
        }
      }
      output.push('\n');
    }
    write!(f, "{output}")
  }
}

impl Elves {
  fn empty_space_count(&self) -> usize {
    let (xb, yb) = self.bounds();
    (xb.count() * yb.count()) - self.layout.len()
  }
  fn bounds(&self) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for xy in &self.layout {
      min_x = min_x.min(xy.x);
      max_x = max_x.max(xy.x);
      min_y = min_y.min(xy.y);
      max_y = max_y.max(xy.y);
    }
    (min_x..=max_x, min_y..=max_y)
  }

  fn neighbors(&self, elf: &XY, positions: &HashSet<XY>) -> u8 {
    // the bit positions of the neighbors are as given below
    // 765
    // 4x3
    // 210
    // x is the current item
    let mut result = 0;
    for y in [-1, 0, 1] {
      for x in [-1, 0, 1] {
        if x == 0 && y == 0 {
          continue;
        }
        result <<= 1;
        if positions.contains(&elf.add(x, y)) {
          result += 1;
        }
      }
    }
    result
  }

  fn propose(&self, elf: &XY, start: usize, positions: &HashSet<XY>) -> Option<XY> {
    let neighbors = self.neighbors(elf, positions);
    if neighbors == 0 {
      return None;
    }
    for i in 0..DIRECTION_COUNT {
      match ((start + i) % DIRECTION_COUNT) as u32 {
        NORTH if NORTH_BITS & neighbors == 0 => return Some(elf.add(0, -1)),
        SOUTH if SOUTH_BITS & neighbors == 0 => return Some(elf.add(0, 1)),
        WEST if WEST_BITS & neighbors == 0 => return Some(elf.add(-1, 0)),
        EAST if EAST_BITS & neighbors == 0 => return Some(elf.add(1, 0)),
        _ => {}
      }
    }
    None
  }

  fn perform_round(&mut self,
                   start: usize) -> bool {
    let l = self.layout.len();
    let mut elf_next = Vec::with_capacity(l);
    let mut proposals = HashMap::with_capacity(l);
    let positions: HashSet<XY> = self.layout.clone().into_iter().collect();

    for elf in &self.layout {
      let proposal = self.propose(elf, start, &positions);
      match &proposal {
        Some(p) => {
          // Add the proposal
          proposals.entry(p.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        }
        _ => {}
      }
      elf_next.push(proposal);
    }

    // Record new positions where possible
    let mut result = false;
    for e in self.layout.iter_mut().rev() {
      match elf_next.pop().unwrap() {
        Some(p) if proposals.get(&p).unwrap() == &1 => {
          *e = p;
          result = true;
        }
        _ => {}
      }
    }
    result
  }

  fn perform_rounds(&mut self, rounds: usize) {
    for i in 0..rounds {
      if !self.perform_round(i) {
        break;
      }
    }
  }

  fn run(&mut self) -> usize {
    let mut round = 0;
    while self.perform_round(round) {
      round += 1;
    }
    round + 1
  }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct XY {
  x: i32,
  y: i32,
}

impl XY {
  fn new(x: i32, y: i32) -> XY {
    XY { x, y }
  }

  fn add(&self, x: i32, y: i32) -> XY {
    XY::new(self.x + x, self.y + y)
  }
}

#[cfg(test)]
mod tests {
  use crate::day23::{generator, XY};

  fn input() -> String {
    vec![
      "....#..",
      "..###.#",
      "#...#.#",
      ".#...##",
      "#.###..",
      "##.#.##",
      ".#..#..",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let elves = generator(input().as_str());
    assert_eq!(22, elves.layout.len());

    assert_eq!(true, elves.layout.contains(&XY::new(4, 0)));
    assert_eq!(true, elves.layout.contains(&XY::new(2, 1)));
    assert_eq!(true, elves.layout.contains(&XY::new(3, 1)));
    assert_eq!(true, elves.layout.contains(&XY::new(4, 1)));
    assert_eq!(false, elves.layout.contains(&XY::new(5, 1)));
    assert_eq!(true, elves.layout.contains(&XY::new(6, 1)));

    let exp = "....#..\n\
               ..###.#\n\
               #...#.#\n\
               .#...##\n\
               #.###..\n\
               ##.#.##\n\
               .#..#..\n";
    assert_eq!(exp, elves.to_string());
  }

  #[test]
  fn test_neighbors() {
    let elves = generator(input().as_str());
    let positions = &elves.layout.clone().into_iter().collect();
    assert_eq!(elves.neighbors(&XY::new(4, 0), &positions),
               2_u8.pow(2) + 2_u8.pow(1));
    assert_eq!(elves.neighbors(&XY::new(2, 1), &positions),
               2_u8.pow(3));
    assert_eq!(elves.neighbors(&XY::new(3, 4), &positions),
               2_u8.pow(4) + 2_u8.pow(1) + 2_u8.pow(3));
    assert_eq!(elves.neighbors(&XY::new(10, 10), &positions), 0);
  }

  #[test]
  fn test_part1() {
    let mut elves = generator(input().as_str());

    let exp = "....#..\n\
               ..###.#\n\
               #...#.#\n\
               .#...##\n\
               #.###..\n\
               ##.#.##\n\
               .#..#..\n";
    assert_eq!(exp, elves.to_string());

    elves.perform_round(0);
    let exp = ".....#...\n\
               ...#...#.\n\
               .#..#.#..\n\
               .....#..#\n\
               ..#.#.##.\n\
               #..#.#...\n\
               #.#.#.##.\n\
               .........\n\
               ..#..#...\n";
    assert_eq!(exp, elves.to_string());

    elves.perform_round(1);
    let exp = "......#....\n\
               ...#.....#.\n\
               ..#..#.#...\n\
               ......#...#\n\
               ..#..#.#...\n\
               #...#.#.#..\n\
               ...........\n\
               .#.#.#.##..\n\
               ...#..#....\n";
    assert_eq!(exp, elves.to_string());

    elves.perform_round(2);
    let exp = "......#....\n\
               ....#....#.\n\
               .#..#...#..\n\
               ......#...#\n\
               ..#..#.#...\n\
               #..#.....#.\n\
               ......##...\n\
               .##.#....#.\n\
               ..#........\n\
               ......#....\n";
    assert_eq!(exp, elves.to_string());

    let mut elves = generator(input().as_str());
    elves.perform_rounds(10);
    let exp = "......#.....\n\
               ..........#.\n\
               .#.#..#.....\n\
               .....#......\n\
               ..#.....#..#\n\
               #......##...\n\
               ....##......\n\
               .#........#.\n\
               ...#.#..#...\n\
               ............\n\
               ...#..#..#..\n";
    assert_eq!(exp, elves.to_string());
    assert_eq!(110, elves.empty_space_count());
  }

  #[test]
  fn test_part2() {
    let mut elves = generator(input().as_str());
    assert_eq!(20, elves.run());
  }

  #[test]
  fn test_simple_input() {
    let input = vec![
      "##",
      "#.",
      "..",
      "##",
    ].join("\n");
    let mut elves = generator(input.as_str());

    elves.perform_round(0);
    let exp = vec![
      "##",
      "..",
      "#.",
      ".#",
      "#.",
      "",
    ].join("\n");
    assert_eq!(exp, elves.to_string());

    elves.perform_round(1);
    let exp = vec![
      ".##.",
      "#...",
      "...#",
      "....",
      ".#..",
      "",
    ].join("\n");
    assert_eq!(exp, elves.to_string());

    elves.perform_round(2);
    let exp = vec![
      "..#..",
      "....#",
      "#....",
      "....#",
      ".....",
      "..#..",
      "",
    ].join("\n");
    assert_eq!(exp, elves.to_string());
  }
}