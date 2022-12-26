use std::collections::{HashMap, HashSet};

pub fn generator(input: &str) -> HashSet<Position> {
  input.lines()
       .map(|l| {
         let mut splits = l.split(',');
         Position {
           x: splits.next().unwrap().parse().unwrap(),
           y: splits.next().unwrap().parse().unwrap(),
           z: splits.next().unwrap().parse().unwrap(),
         }
       }).collect()
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Position {
  x: u32,
  y: u32,
  z: u32,
}

impl Position {
  fn new(x: u32, y: u32, z: u32) -> Position {
    if x == 0 || y == 0 || z == 0 {
      panic!("Expecting coordinates > 1, but received ({x},{y},{z})");
    }
    Position { x, y, z }
  }

  fn change_x(&self, x: isize) -> Position {
    if x != -1 && x != 1 {
      panic!("Unexpected value, expecting either -1 or 1");
    }
    let mut result = self.clone();
    result.x = (result.x as isize + x) as u32;
    result
  }

  fn change_y(&self, y: isize) -> Position {
    if y != -1 && y != 1 {
      panic!("Unexpected value, expecting either -1 or 1");
    }
    let mut result = self.clone();
    result.y = (result.y as isize + y) as u32;
    result
  }

  fn change_z(&self, z: isize) -> Position {
    if z != -1 && z != 1 {
      panic!("Unexpected value, expecting either -1 or 1");
    }
    let mut result = self.clone();
    result.z = (result.z as isize + z) as u32;
    result
  }
}

pub fn part1(positions: &HashSet<Position>) -> u32 {
  let mut count: u32 = 0;
  for position in positions {
    // Check the x neighbors
    for c in [-1, 1] {
      let p = position.change_x(c);
      if !positions.contains(&p) {
        count += 1;
      }
    }

    // Check the y neighbors
    for c in [-1, 1] {
      let p = position.change_y(c);
      if !positions.contains(&p) {
        count += 1;
      }
    }

    // Check the z neighbors
    for c in [-1, 1] {
      let p = position.change_z(c);
      if !positions.contains(&p) {
        count += 1;
      }
    }
  }

  count
}

pub fn part2(positions: &HashSet<Position>) -> u32 {
  // Find spaces that are not open and exclude them from the counts
  let mut min_pos = Position::new(u32::MAX, u32::MAX, u32::MAX);
  let mut max_pos = Position::new(1, 1, 1);
  for position in positions {
    min_pos.x = min_pos.x.min(position.x);
    min_pos.y = min_pos.y.min(position.y);
    min_pos.z = min_pos.z.min(position.z);

    max_pos.x = max_pos.x.max(position.x);
    max_pos.y = max_pos.y.max(position.y);
    max_pos.z = max_pos.z.max(position.z);
  }

  let mut count: u32 = 0;
  let mut empty_status = HashMap::new();
  for position in positions {
    // Check the x neighbors
    for c in [-1, 1] {
      let p = position.change_x(c);
      if !positions.contains(&p) && is_space_open(p, positions, &min_pos, &max_pos, &mut empty_status) {
        count += 1;
      }
    }

    // Check the y neighbors
    for c in [-1, 1] {
      let p = position.change_y(c);
      if !positions.contains(&p) && is_space_open(p, positions, &min_pos, &max_pos, &mut empty_status) {
        count += 1;
      }
    }

    // Check the z neighbors
    for c in [-1, 1] {
      let p = position.change_z(c);
      if !positions.contains(&p) && is_space_open(p, positions, &min_pos, &max_pos, &mut empty_status) {
        count += 1;
      }
    }
  }

  count
}

fn is_space_open(position: Position, positions: &HashSet<Position>,
                 min_pos: &Position, max_pos: &Position,
                 empty_status: &mut HashMap<Position, bool>) -> bool {
  let mut visited: HashSet<Position> = HashSet::new();
  let mut stack: Vec<Position> = Vec::new();
  stack.push(position);
  let mut open: bool = false;
  while !stack.is_empty() {
    // Navigate on empty neighbors until you get to out of bounds or exhaust the stack
    let curr = stack.pop().unwrap();
    if visited.contains(&curr) {
      continue;
    }
    if empty_status.contains_key(&curr) {
      open = *empty_status.get(&curr).unwrap();
      break;
    }
    if curr.x <= min_pos.x || curr.x >= max_pos.x
      || curr.y <= min_pos.y || curr.y >= max_pos.y
      || curr.z <= min_pos.z || curr.z >= max_pos.z {
      // If we have come to the edge then mark this as open
      open = true;
      break;
    }
    // Check the x neighbors
    for c in [-1, 1] {
      match curr.change_x(c) {
        p if !positions.contains(&p) => stack.push(p),
        _ => {}
      }
    }

    // Check the y neighbors
    for c in [-1, 1] {
      match curr.change_y(c) {
        p if !positions.contains(&p) => stack.push(p),
        _ => {}
      }
    }

    // Check the z neighbors
    for c in [-1, 1] {
      match curr.change_z(c) {
        p if !positions.contains(&p) => stack.push(p),
        _ => {}
      }
    }
    visited.insert(curr);
  }
  // Mark the visited positions
  for position in visited {
    empty_status.insert(position, open);
  }
  open
}

#[cfg(test)]
mod tests {
  use crate::day18::{generator, part1, part2};

  fn input() -> String {
    vec![
      "2,2,2",
      "1,2,2",
      "3,2,2",
      "2,1,2",
      "2,3,2",
      "2,2,1",
      "2,2,3",
      "2,2,4",
      "2,2,6",
      "1,2,5",
      "3,2,5",
      "2,1,5",
      "2,3,5",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let droplets = generator(input().as_str());
    assert_eq!(13, droplets.len());
  }

  #[test]
  fn test_part1() {
    let droplets = generator(input().as_str());
    assert_eq!(64, part1(&droplets));
  }

  #[test]
  fn test_part2() {
    let droplets = generator(input().as_str());
    assert_eq!(58, part2(&droplets));
  }
}