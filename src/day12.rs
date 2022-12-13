use std::borrow::BorrowMut;
use std::collections::VecDeque;

pub fn generator(input: &str) -> ElevationMap {
  ElevationMap::new(input)
}

pub struct ElevationMap {
  map: Vec<Vec<u32>>,
  start: Position,
  end: Position,
  width: usize,
  height: usize,
}

impl ElevationMap {
  fn new(input: &str) -> ElevationMap {
    let mut y = 0;
    let mut x;
    let mut start = None;
    let mut end = None;
    let mut map = Vec::new();
    for line in input.lines() {
      let mut chars = Vec::new();
      x = 0;
      for c in line.chars() {
        match c {
          'S' => {
            start = Some(Position { x, y });
            chars.push('a' as u32);
          }
          'E' => {
            end = Some(Position { x, y });
            chars.push('z' as u32);
          }
          _ => chars.push(c as u32)
        }
        x += 1;
      }
      y += 1;
      map.push(chars);
    }
    let height = map.len();
    let width = map.first().unwrap().len();
    ElevationMap {
      map,
      start: start.unwrap(),
      end: end.unwrap(),
      height,
      width,
    }
  }

  fn compute_steps(&self, best: bool) -> u32 {
    let neighbors: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut min_steps: Vec<Vec<u32>> = vec![vec![u32::MAX; self.map.first().unwrap().len()]; self.map.len()];
    let mut to_check: VecDeque<Position> = VecDeque::new();
    *min_steps[self.end.y][self.end.x].borrow_mut() = 0;
    to_check.push_back(self.end.clone());
    let mut best_start_steps = u32::MAX;
    let start_code = self.map[self.start.y][self.start.x];
    while !to_check.is_empty() {
      let curr = to_check.pop_front().unwrap();
      let min_elevation = self.map[curr.y][curr.x] as u32 - 1;
      let next_steps = min_steps[curr.y][curr.x] + 1;
      if next_steps > best_start_steps {
        break;
      }
      for (dx, dy) in neighbors {
        let n_x = curr.x as isize + dx;
        let n_y = curr.y as isize + dy;
        if n_x < 0
          || n_x >= self.width as isize
          || n_y < 0
          || n_y >= self.height as isize {
          continue;
        }
        let next_pos = Position { x: n_x as usize, y: n_y as usize };
        if (self.map[next_pos.y][next_pos.x] as u32) < min_elevation
          || min_steps[next_pos.y][next_pos.x] <= next_steps {
          continue;
        }
        *min_steps[n_y as usize][n_x as usize].borrow_mut() = next_steps;
        to_check.push_back(next_pos);
        if (n_y as usize == self.start.y && n_x as usize == self.start.x)
          || (best && self.map[n_y as usize][n_x as usize] == start_code) {
          best_start_steps = best_start_steps.min(next_steps);
        }
      }
    }

    best_start_steps
  }
}

pub fn part1(elev_map: &ElevationMap) -> u32 {
  elev_map.compute_steps(false)
}

pub fn part2(elev_map: &ElevationMap) -> u32 {
  elev_map.compute_steps(true)
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Position {
  x: usize,
  y: usize,
}

#[cfg(test)]
mod tests {
  use crate::day12::{generator, part1, part2, Position};

  fn input() -> String {
    vec![
      "Sabqponm",
      "abcryxxl",
      "accszExk",
      "acctuvwj",
      "abdefghi",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let elev_map = generator(input().as_str());
    assert_eq!(Position { x: 0, y: 0 }, elev_map.start);
    assert_eq!(Position { x: 5, y: 2 }, elev_map.end);
    assert_eq!(5, elev_map.map.len());
    assert_eq!(8, elev_map.map.first().unwrap().len());
  }

  #[test]
  fn test_part1() {
    let elev_map = generator(input().as_str());
    assert_eq!(31, part1(&elev_map));
  }

  #[test]
  fn test_part2() {
    let elev_map = generator(input().as_str());
    assert_eq!(29, part2(&elev_map));
  }
}