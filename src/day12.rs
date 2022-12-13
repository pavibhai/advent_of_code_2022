use std::borrow::BorrowMut;

pub fn generator(input: &str) -> ElevationMap {
  ElevationMap::new(input)
}

pub struct ElevationMap {
  map: Vec<Vec<char>>,
  start: (usize, usize),
  end: (usize, usize),
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
            start = Some((x, y));
            chars.push('a');
          }
          'E' => {
            end = Some((x, y));
            chars.push('z');
          }
          _ => chars.push(c)
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
    let mut min_steps: Vec<Vec<u32>> = vec![vec![u32::MAX; self.map.first().unwrap().len()]; self.map.len()];
    let mut to_check: Vec<(usize, usize)> = Vec::new();
    *min_steps[self.end.1][self.end.0].borrow_mut() = 0;
    to_check.push(self.end);
    let mut best_start_steps = u32::MAX;
    while !to_check.is_empty() {
      let (c_x, c_y) = to_check.pop().unwrap();
      let min_elevation = self.map[c_y][c_x] as u32 - 1;
      let next_steps = min_steps[c_y][c_x] + 1;
      for (n_x, n_y) in self.neighbors(c_x, c_y) {
        if (self.map[n_y][n_x] as u32) < min_elevation || min_steps[n_y][n_x] <= next_steps {
          continue;
        }
        *min_steps[n_y][n_x].borrow_mut() = next_steps;
        to_check.push((n_x, n_y));
        if self.map[n_y][n_x] == 'a' {
          best_start_steps = best_start_steps.min(next_steps);
        }
      }
    }

    if best {
      best_start_steps
    } else {
      min_steps[self.start.1][self.start.0]
    }
  }

  fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    // left
    if x > 0 {
      result.push((x - 1, y));
    }
    // right
    if x + 1 < self.width {
      result.push((x + 1, y));
    }
    // up
    if y > 0 {
      result.push((x, y - 1));
    }
    // down
    if y + 1 < self.height {
      result.push((x, y + 1));
    }
    result
  }
}

pub fn part1(elev_map: &ElevationMap) -> u32 {
  elev_map.compute_steps(false)
}

pub fn part2(elev_map: &ElevationMap) -> u32 {
  elev_map.compute_steps(true)
}

#[cfg(test)]
mod tests {
  use crate::day12::{generator, part1, part2};

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
    assert_eq!((0, 0), elev_map.start);
    assert_eq!((5, 2), elev_map.end);
    assert_eq!(5, elev_map.map.len());
    assert_eq!(8, elev_map.map.first().unwrap().len());
  }

  #[test]
  fn test_neighbors() {
    let elev_map = generator(input().as_str());
    assert_eq!(vec![(4, 2), (6, 2), (5, 1), (5, 3)], elev_map.neighbors(5, 2));
    assert_eq!(vec![(0, 0), (2, 0), (1, 1)], elev_map.neighbors(1, 0));
    assert_eq!(vec![(1, 0), (0, 1)], elev_map.neighbors(0, 0));
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