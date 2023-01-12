use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

const UP_WIND: char = '^';
const DOWN_WIND: char = 'v';
const LEFT_WIND: char = '<';
const RIGHT_WIND: char = '>';
const CLEAR: char = '.';

pub fn generator(input: &str) -> Valley {
  let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
  let start_x = map.first().unwrap().iter().position(|c| c == &CLEAR).unwrap();
  let end_x = map.last().unwrap().iter().position(|c| c == &CLEAR).unwrap();
  let width = map.first().unwrap().len();

  // Validate that there are no up or down arrows on start and end
  for y in 0..map.len() {
    if map[y].len() != width {
      panic!("Unexpected. The width for y {y} did not match {width}")
    }
    if map[y][start_x] == UP_WIND || map[y][start_x] == DOWN_WIND
      || map[y][end_x] == UP_WIND || map[y][end_x] == DOWN_WIND {
      panic!("Unexpected, cannot have up/down wind at ({start_x},{y}) and ({end_x},{y})");
    }
  }

  Valley {
    start: XY::new(start_x as i32, 0),
    end: XY::new(end_x as i32, (map.len() - 1) as i32),
    width: (width - 2) as i32,
    height: (map.len() - 2) as i32,
    map,
  }
}

pub fn part1(valley: &Valley) -> i32 {
  valley.find_time(&valley.start, &valley.end.add(0, -1), 0)
}

pub fn part2(valley: &Valley) -> i32 {
  let mut t = part1(valley);
  t = valley.find_time(&valley.end, &valley.start.add(0, 1), t);
  valley.find_time(&valley.start, &valley.end.add(0, -1), t)
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct XY {
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

pub struct Valley {
  map: Vec<Vec<char>>,
  width: i32,
  height: i32,
  start: XY,
  end: XY,
}

impl Valley {
  fn space(&self, x: i32, y: i32) -> &char {
    &self.map[y as usize][x as usize]
  }

  fn is_clear(&self, x: i32, y: i32, t: i32) -> bool {
    if x <= 0 || x > self.width || y <= 0 || y > self.height {
      false
    } else {
      let x = x - 1;
      let y = y - 1;

      self.space(1 + (x - t).rem_euclid(self.width), y + 1) != &RIGHT_WIND
        && self.space(1 + (x + t).rem_euclid(self.width), y + 1) != &LEFT_WIND
        && self.space(x + 1, 1 + (y - t).rem_euclid(self.height)) != &DOWN_WIND
        && self.space(x + 1, 1 + (y + t).rem_euclid(self.height)) != &UP_WIND
    }
  }

  fn find_time(&self, start: &XY, end: &XY, start_t: i32) -> i32 {
    let mut explored: HashSet<(XY, i32)> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), start_t));

    while !queue.is_empty() {
      let (p, mut t) = queue.pop_front().unwrap();
      if explored.contains(&(p.clone(), t.clone())) {
        continue;
      }
      explored.insert((p.clone(), t));
      t += 1;
      if &p == end {
        // Reached destination
        return t;
      }
      if self.is_clear(p.x + 1, p.y, t) {
        // Move right
        queue.push_back((p.add(1, 0), t))
      }
      if self.is_clear(p.x - 1, p.y, t) {
        // Move left
        queue.push_back((p.add(-1, 0), t))
      }
      if self.is_clear(p.x, p.y - 1, t) {
        // Move up
        queue.push_back((p.add(0, -1), t))
      }
      if self.is_clear(p.x, p.y + 1, t) {
        // Move down
        queue.push_back((p.add(0, 1), t))
      }
      if &p == start || self.is_clear(p.x, p.y, t) {
        // Wait
        queue.push_back((p, t))
      }
    }

    panic!("Did not find a path")
  }

  fn _layout_string(&self, t: i32) -> String {
    let mut result = String::new();
    for y in 0..self.height + 2 {
      for x in 0..self.width + 2 {
        if self.is_clear(x, y, t) {
          result.push(CLEAR);
        } else {
          result.push('x');
        }
      }
      result.push('\n');
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use crate::day24::{CLEAR, generator, part1, part2, RIGHT_WIND, WALL, XY};

  fn input() -> String {
    vec![
      "#.######",
      "#>>.<^<#",
      "#.<..<<#",
      "#>v.><>#",
      "#<^v^^>#",
      "######.#",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let v = generator(input().as_str());
    assert_eq!(4, v.height);
    assert_eq!(6, v.width);
    assert_eq!(XY::new(1, 0), v.start);
    assert_eq!(XY::new(6, 5), v.end);

    assert_eq!(v.space(1, 0), &CLEAR);
    assert_eq!(v.space(2, 0), &WALL);
    assert_eq!(v.space(1, 1), &RIGHT_WIND);
  }

  #[test]
  fn test_time() {
    let v = generator(input().as_str());
    assert_eq!(true, v.is_clear(1, 1, 1));
    assert_eq!(false, v.is_clear(2, 1, 1));
    assert_eq!(true, v.is_clear(4, 1, 1));

    assert_eq!(false, v.is_clear(6, 1, 4));
    assert_eq!(true, v.is_clear(6, 2, 4));
    assert_eq!(false, v.is_clear(0, 4, 4));
    assert_eq!(false, v.is_clear(1, 0, 4));
    assert_eq!(false, v.is_clear(6, 5, 4));
    assert_eq!(false, v.is_clear(7, 5, 4));

    assert_eq!(false, v.is_clear(1, 4, 6));
    assert_eq!(true, v.is_clear(2, 4, 6));
    assert_eq!(true, v.is_clear(3, 4, 6));

    assert_eq!(v._layout_string(0),
               "xxxxxxxx\n\
                xxx.xxxx\n\
                x.x..xxx\n\
                xxx.xxxx\n\
                xxxxxxxx\n\
                xxxxxxxx\n");
    assert_eq!(v._layout_string(4),
               "xxxxxxxx\n\
                x.x..xxx\n\
                xxx.x..x\n\
                xxx.xx.x\n\
                x.xxxx.x\n\
                xxxxxxxx\n");
    assert_eq!(v._layout_string(17),
               "xxxxxxxx\n\
                xx.x.xxx\n\
                xx.x..xx\n\
                x.xxxxxx\n\
                x.x..x.x\n\
                xxxxxxxx\n");
  }

  #[test]
  fn test_part1() {
    let v = generator(input().as_str());
    assert_eq!(18, part1(&v));
  }

  #[test]
  fn test_part2() {
    let v = generator(input().as_str());
    assert_eq!(54, part2(&v));
  }
}