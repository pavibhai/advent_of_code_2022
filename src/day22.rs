use std::collections::{HashMap, HashSet};
use crate::day22::Step::{Left, Move, Right};

const OPEN_SPACE: char = '.';
const BLOCKED_SPACE: char = '#';

pub fn generator(input: &str) -> Puzzle {
  let mut parts = input.split("\n\n");
  let map = parts.next().unwrap();
  let steps = make_steps(parts.next().unwrap().lines().next().unwrap());
  let regions = make_lines(map);
  Puzzle {
    steps,
    lines: regions,
  }
}

fn make_lines(input: &str) -> Vec<Line> {
  let mut y = 0;
  let mut lines = Vec::new();
  for line in input.lines() {
    let x_l = line.find([OPEN_SPACE, BLOCKED_SPACE]).unwrap();
    let x_r = line.rfind([OPEN_SPACE, BLOCKED_SPACE]).unwrap();
    lines.push(Line {
      min_x: x_l as u32,
      max_x: x_r as u32,
      y,
      contents: line[x_l..=x_r].chars().collect(),
    });
    y += 1;
  }
  lines
}

fn make_steps(input: &str) -> Vec<Step> {
  let mut result = Vec::new();
  let mut move_steps = 0;
  for c in input.chars() {
    if !c.is_digit(10) {
      if move_steps > 0 {
        result.push(Move(move_steps));
      }
      move_steps = 0;
    }
    match c {
      'L' => result.push(Left),
      'R' => result.push(Right),
      c if c.is_digit(10) => {
        move_steps *= 10;
        move_steps += c.to_digit(10).unwrap();
      }
      _ => panic!("Unexpected character {c}"),
    }
  }
  if move_steps > 0 {
    result.push(Move(move_steps));
  }
  result
}

pub fn part1(puzzle: &Puzzle) -> i32 {
  puzzle.follow_path().password()
}

pub fn part2(puzzle: &Puzzle) -> i32 {
  puzzle.follow_cube_path(50).password()
}

fn find_p(e: &Edge, p: &XY) -> u32 {
  if e.p1.x.abs_diff(p.x) < 2 && e.p1.y.abs_diff(p.y) < 2 {
    2
  } else {
    1
  }
}

fn make_links(edges: &Vec<Edge>) -> Vec<(usize, usize)> {
  let mut links = vec![(0, 0); edges.len()];
  let mut curr_idx = 0;
  let mut next_idx;
  // Rotating clockwise
  let mut p = 2;
  let mut visited = 0;
  while visited < edges.len() {
    let curr = &edges[curr_idx];
    next_idx = if p == 2 {
      get_edge(&curr.p2, &curr.p1, edges)
    } else {
      get_edge(&curr.p1, &curr.p2, edges)
    };
    links.get_mut(curr_idx).unwrap().1 = next_idx;
    links.get_mut(next_idx).unwrap().0 = curr_idx;
    p = find_p(&edges[next_idx], if p == 2 { &curr.p2 } else { &curr.p1 });
    curr_idx = next_idx;
    visited += 1;
  }

  links
}

fn get_edge(p: &XY, o_p: &XY, edges: &Vec<Edge>) -> usize {
  for i in 0..edges.len() {
    let e = &edges[i];
    if (e.p1.x.abs_diff(p.x) < 2 && e.p1.y.abs_diff(p.y) < 2 && &e.p2 != o_p)
      || (e.p2.x.abs_diff(p.x) < 2 && e.p2.y.abs_diff(p.y) < 2 && &e.p1 != o_p) {
      return i;
    }
  }
  panic!("Edge not found");
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
struct XY {
  x: i32,
  y: i32,
}

impl XY {
  fn new(x: i32, y: i32) -> XY {
    XY { x, y }
  }
  fn add(&mut self, other: &XY) {
    self.y += other.y;
    self.x += other.x;
  }

  fn reverse(&self) -> XY {
    XY::new(self.x * -1, self.y * -1)
  }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Edge {
  p1: XY,
  p2: XY,
  dir: XY,
}

impl Edge {
  fn identify_closest_right(&self) -> XY {
    match self.dir {
      XY { x: 1, y: 0 } => { if self.p1.y > self.p2.y { self.p1.clone() } else { self.p2.clone() } }
      XY { x: 0, y: 1 } => { if self.p1.x < self.p2.x { self.p1.clone() } else { self.p2.clone() } }
      XY { x: -1, y: 0 } => { if self.p1.y < self.p2.y { self.p1.clone() } else { self.p2.clone() } }
      XY { x: 0, y: -1 } => { if self.p1.x > self.p2.x { self.p1.clone() } else { self.p2.clone() } }
      _ => panic!("Unexpected direction {:?}", self),
    }
  }

  fn identify_closest_left(&self) -> XY {
    match self.dir {
      XY { x: 1, y: 0 } => { if self.p1.y < self.p2.y { self.p1.clone() } else { self.p2.clone() } }
      XY { x: 0, y: 1 } => { if self.p1.x > self.p2.x { self.p1.clone() } else { self.p2.clone() } }
      XY { x: -1, y: 0 } => { if self.p1.y > self.p2.y { self.p1.clone() } else { self.p2.clone() } }
      XY { x: 0, y: -1 } => { if self.p1.x < self.p2.x { self.p1.clone() } else { self.p2.clone() } }
      _ => panic!("Unexpected direction {:?}", self),
    }
  }
}

pub struct Puzzle {
  lines: Vec<Line>,
  steps: Vec<Step>,
}

impl Puzzle {
  fn map_edges(&self, side: u32) -> HashMap<(XY, XY), (XY, XY)> {
    let edges = self.make_edges(side);
    let mut mappings: HashMap<(XY, XY), (XY, XY)> = HashMap::new();
    let mut mapped_edges: HashSet<usize> = HashSet::new();

    // Find the remaining mappings
    let mut mapped_count;
    let links = make_links(&edges);

    while mapped_edges.len() < edges.len() {
      for idx in 0..edges.len() {
        if mapped_edges.contains(&idx) {
          // Edge already mapped ignore
          continue;
        }

        // try rotating right
        let mut next_idx = links[idx].1;
        mapped_count = 0;
        while mapped_edges.contains(&next_idx) {
          mapped_count += 1;
          next_idx = links[next_idx].1;
        }
        let start = &edges[idx];
        let next = &edges[next_idx];
        let mut next_move_dir = start.dir.clone();
        let mut start_move_dir = next.dir.clone();
        for _ in 0..mapped_count / 2 {
          Right.change_dir(&mut next_move_dir);
          Left.change_dir(&mut start_move_dir);
        }
        let mut start_p = start.identify_closest_right();
        let mut next_p = next.identify_closest_left();
        for _ in 0..side - 1 {
          start_p.add(&start_move_dir);
          next_p.add(&next_move_dir);
        }
        if start_p == start.identify_closest_left() && next_p == next.identify_closest_right() {
          let mut start_p = start.identify_closest_right();
          let mut next_p = next.identify_closest_left();
          for _ in 0..side {
            mappings.insert((start_p.clone(), start.dir.clone()), (next_p.clone(), next.dir.reverse()));
            mappings.insert((next_p.clone(), next.dir.clone()), (start_p.clone(), start.dir.reverse()));
            start_p.add(&start_move_dir);
            next_p.add(&next_move_dir);
          }
          mapped_edges.insert(idx);
          mapped_edges.insert(next_idx);
          continue;
        }

        // As we didn't succeed rotating right, let us try to rotate left
        let mut next_idx = links[idx].0;
        mapped_count = 0;
        while mapped_edges.contains(&next_idx) {
          mapped_count += 1;
          next_idx = links[next_idx].0;
        }
        let start = &edges[idx];
        let next = &edges[next_idx];
        let mut next_move_dir = start.dir.clone();
        let mut start_move_dir = next.dir.clone();
        for _ in 0..mapped_count / 2 {
          Left.change_dir(&mut next_move_dir);
          Right.change_dir(&mut start_move_dir);
        }
        let mut start_p = start.identify_closest_left();
        let mut next_p = next.identify_closest_right();
        for _ in 0..side - 1 {
          start_p.add(&start_move_dir);
          next_p.add(&next_move_dir);
        }
        if start_p == start.identify_closest_right() && next_p == next.identify_closest_left() {
          let mut start_p = start.identify_closest_left();
          let mut next_p = next.identify_closest_right();
          for _ in 0..side {
            mappings.insert((start_p.clone(), start.dir.clone()), (next_p.clone(), next.dir.reverse()));
            mappings.insert((next_p.clone(), next.dir.clone()), (start_p.clone(), start.dir.reverse()));
            start_p.add(&start_move_dir);
            next_p.add(&next_move_dir);
          }
          mapped_edges.insert(idx);
          mapped_edges.insert(next_idx);
        }
      }
    }
    mappings
  }

  fn make_edges(&self, side: u32) -> Vec<Edge> {
    let mut edges = Vec::new();

    for y in 0..self.lines.len() / side as usize {
      let y = y * side as usize;
      let line = &self.lines[y];
      let mut x = line.min_x;
      while x < line.max_x {
        // Top edge
        if line.y == 0
          || &x < &self.lines[(line.y - 1) as usize].min_x
          || &x > &self.lines[(line.y - 1) as usize].max_x {
          edges.push(Edge {
            p1: XY { x: x as i32, y: line.y as i32 },
            p2: XY { x: (x + side - 1) as i32, y: line.y as i32 },
            dir: XY { x: 0, y: -1 },
          });
        }

        // Bottom edge
        let y_next = y + side as usize;
        if y_next == self.lines.len()
          || &x < &self.lines[(y_next) as usize].min_x
          || &x > &self.lines[(y_next) as usize].max_x {
          edges.push(Edge {
            p1: XY { x: x as i32, y: (y_next - 1) as i32 },
            p2: XY { x: (x + side - 1) as i32, y: (y_next - 1) as i32 },
            dir: XY { x: 0, y: 1 },
          });
        }

        x += side;
      }

      // Vertical edges
      edges.push(
        Edge {
          p1: XY { x: line.min_x as i32, y: line.y as i32 },
          p2: XY { x: line.min_x as i32, y: (line.y + side - 1) as i32 },
          dir: XY { x: -1, y: 0 },
        }
      );
      edges.push(
        Edge {
          p1: XY { x: line.max_x as i32, y: line.y as i32 },
          p2: XY { x: line.max_x as i32, y: (line.y + side - 1) as i32 },
          dir: XY { x: 1, y: 0 },
        }
      );
    }
    edges
  }

  fn initial_state(&self) -> State {
    State {
      p: XY::new(self.lines.first().unwrap().min_x as i32, 0),
      dir: XY::new(1, 0),
    }
  }

  fn follow_path(&self) -> State {
    let mut curr = self.initial_state();

    for step in &self.steps {
      match step {
        Move(n) => {
          for _ in 0..*n {
            if !curr.move_forward(self) {
              break;
            }
          }
        }
        Left | Right => step.change_dir(&mut curr.dir)
      }
    }
    curr
  }

  fn follow_cube_path(&self, side: u32) -> State {
    let mut curr = self.initial_state();
    let mappings = self.map_edges(side);

    for step in &self.steps {
      match step {
        Move(n) => {
          for _ in 0..*n {
            if !curr.move_forward_with_mappings(self, &mappings) {
              break;
            }
          }
        }
        Left | Right => step.change_dir(&mut curr.dir)
      }
    }
    curr
  }

  fn get_space(&self, x: &i32, y: &i32) -> char {
    let line = &self.lines[*y as usize];
    line.contents[(*x - line.min_x as i32) as usize]
  }
}

struct Line {
  min_x: u32,
  max_x: u32,
  y: u32,
  contents: Vec<char>,
}

impl Line {
  fn width(&self) -> u32 {
    self.contents.len() as u32
  }
}

#[derive(Eq, PartialEq, Debug)]
enum Step {
  Move(u32),
  Left,
  Right,
}

impl Step {
  fn change_dir(&self, dir: &mut XY) {
    match self {
      Left if dir.x == 0 => {
        dir.x = dir.y;
        dir.y = 0;
      }
      Left if dir.y == 0 => {
        dir.y = -dir.x;
        dir.x = 0;
      }
      Right if dir.y == 0 => {
        dir.y = dir.x;
        dir.x = 0;
      }
      Right if dir.x == 0 => {
        dir.x = -dir.y;
        dir.y = 0;
      }
      _ => {}
    }
  }
}

struct State {
  p: XY,
  dir: XY,
}

impl State {
  fn facing(&self) -> u32 {
    match (self.dir.x, self.dir.y) {
      (1, 0) => 0,
      (0, 1) => 1,
      (-1, 0) => 2,
      (0, -1) => 3,
      _ => panic!("Unexpected direction ({}, {})", self.dir.x, self.dir.y),
    }
  }

  fn move_forward(&mut self, puzzle: &Puzzle) -> bool {
    let new_x = if self.dir.x != 0 {
      let line = &puzzle.lines[self.p.y as usize];
      line.min_x as i32
        + (self.p.x as i32 - line.min_x as i32 + self.dir.x).rem_euclid(line.width() as i32)
    } else {
      self.p.x
    };
    let new_y = if self.dir.y != 0 {
      self.determine_new_y(puzzle, self.dir.y, new_x)
    } else {
      self.p.y
    };

    if puzzle.get_space(&new_x, &new_y) == OPEN_SPACE {
      self.p.x = new_x;
      self.p.y = new_y as i32;
      true
    } else {
      false
    }
  }

  fn move_forward_with_mappings(&mut self, puzzle: &Puzzle, mappings: &HashMap<(XY, XY), (XY, XY)>) -> bool {
    let (new_p, new_dir) = match mappings.get(&(self.p, self.dir)) {
      Some((p, dir)) => (p.clone(), dir.clone()),
      None => {
        let mut np = self.p.clone();
        np.add(&self.dir);
        (np, self.dir.clone())
      }
    };

    if puzzle.get_space(&new_p.x, &new_p.y) == OPEN_SPACE {
      self.p = new_p;
      self.dir = new_dir;
      true
    } else {
      false
    }
  }

  fn determine_new_y(&mut self, puzzle: &Puzzle, y_change: i32, new_x: i32) -> i32 {
    let mut new_y = (self.p.y + y_change).rem_euclid(puzzle.lines.len() as i32) as usize;
    if new_x < puzzle.lines[new_y].min_x as i32
      || new_x > puzzle.lines[new_y].max_x as i32 {
      // As the next region does not include the range, check for the wrap around by going in the
      // opposite direction
      new_y = (self.p.y - y_change).rem_euclid(puzzle.lines.len() as i32) as usize;
      while new_x >= puzzle.lines[new_y as usize].min_x as i32
        && new_x <= puzzle.lines[new_y as usize].max_x as i32 {
        new_y = (new_y as i32 - y_change).rem_euclid(puzzle.lines.len() as i32) as usize;
      }
      (new_y as i32 + y_change).rem_euclid(puzzle.lines.len() as i32) as i32
    } else {
      // Since the next region includes the x range, use this value
      new_y.try_into().unwrap()
    }
  }

  fn password(&self) -> i32 {
    (1000 * (self.p.y + 1)) + (4 * (self.p.x + 1)) + self.facing() as i32
  }
}

#[cfg(test)]
mod tests {
  use crate::day22::{Edge, generator, part1, XY, State, make_links};
  use crate::day22::Step::{Left, Move, Right};

  fn input() -> String {
    vec![
      "        ...#",
      "        .#..",
      "        #...",
      "        ....",
      "...#.......#",
      "........#...",
      "..#....#....",
      "..........#.",
      "        ...#....",
      "        .....#..",
      "        .#......",
      "        ......#.",
      "",
      "10R5L5R10L4R5L5",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let p = generator(input().as_str());
    assert_eq!(p.lines.len(), 12);
    let line = p.lines.first().unwrap();
    assert_eq!(line.min_x, 8);
    assert_eq!(line.y, 0);
    assert_eq!("...#", line.contents.iter().collect::<String>());

    let line = &p.lines[3];
    assert_eq!(line.min_x, 8);
    assert_eq!(line.y, 3);
    assert_eq!("....", line.contents.iter().collect::<String>());

    let line = &p.lines[4];
    assert_eq!(line.min_x, 0);
    assert_eq!(line.y, 4);
    assert_eq!("...#.......#", line.contents.iter().collect::<String>());

    let line = &p.lines[7];
    assert_eq!(line.min_x, 0);
    assert_eq!(line.y, 7);
    assert_eq!("..........#.", line.contents.iter().collect::<String>());

    let line = &p.lines[8];
    assert_eq!(line.min_x, 8);
    assert_eq!(line.y, 8);
    assert_eq!("...#....", line.contents.iter().collect::<String>());

    let line = &p.lines[11];
    assert_eq!(line.min_x, 8);
    assert_eq!(line.y, 11);
    assert_eq!("......#.", line.contents.iter().collect::<String>());

    assert_eq!(p.steps.len(), 13);
    assert_eq!(&Move(10), p.steps.first().unwrap());
    assert_eq!(&Right, &p.steps[1]);
    assert_eq!(&Left, &p.steps[11]);
    assert_eq!(&Move(5), p.steps.last().unwrap());
  }

  #[test]
  fn test_mod() {
    assert_eq!(6, (-1_i32).rem_euclid(7));
    assert_eq!(0, (7_i32).rem_euclid(7));
    assert_eq!(1, (8_i32).rem_euclid(7));
  }

  #[test]
  fn test_steps() {
    let p = generator(input().as_str());
    let mut curr = State {
      p: XY::new(p.lines.first().unwrap().min_x as i32, 0),
      dir: XY::new(1, 0),
    };
    assert_eq!(true, curr.move_forward(&p));
    assert_eq!(9, curr.p.x);
    assert_eq!(true, curr.move_forward(&p));
    assert_eq!(10, curr.p.x);
    assert_eq!(false, curr.move_forward(&p));
    assert_eq!(10, curr.p.x);
    Left.change_dir(&mut curr.dir);
    assert_eq!(10, curr.p.x);
    assert_eq!(true, curr.move_forward(&p));
    assert_eq!(10, curr.p.x);
    assert_eq!(11, curr.p.y);
    for _ in 0..3 {
      assert_eq!(true, curr.move_forward(&p));
    }
    assert_eq!(10, curr.p.x);
    assert_eq!(8, curr.p.y);
    assert_eq!(false, curr.move_forward(&p));
    assert_eq!(10, curr.p.x);
    assert_eq!(8, curr.p.y);
    for _ in 0..3 {
      Right.change_dir(&mut curr.dir);
    }
    for _ in 0..3 {
      assert_eq!(true, curr.move_forward(&p));
    }
    assert_eq!(8, curr.p.y);
    assert_eq!(15, curr.p.x);
    Right.change_dir(&mut curr.dir);
    Right.change_dir(&mut curr.dir);
    assert_eq!(true, curr.move_forward(&p));
    assert_eq!(8, curr.p.x);
    assert_eq!(8, curr.p.y);
    Left.change_dir(&mut curr.dir);
    assert_eq!(true, curr.move_forward(&p));
    assert_eq!(8, curr.p.x);
    assert_eq!(7, curr.p.y);
    Left.change_dir(&mut curr.dir);
    for _ in 0..5 {
      assert_eq!(true, curr.move_forward(&p));
    }
    assert_eq!(3, curr.p.x);
    assert_eq!(7, curr.p.y);
    Left.change_dir(&mut curr.dir);
    assert_eq!(false, curr.move_forward(&p));
    assert_eq!(3, curr.p.x);
    assert_eq!(7, curr.p.y);
    Right.change_dir(&mut curr.dir);
    assert_eq!(true, curr.move_forward(&p));
    assert_eq!(2, curr.p.x);
    assert_eq!(7, curr.p.y);
    Left.change_dir(&mut curr.dir);
    assert_eq!(true, curr.move_forward(&p));
    assert_eq!(2, curr.p.x);
    assert_eq!(4, curr.p.y);
  }

  #[test]
  fn test_part1() {
    let p = generator(input().as_str());
    assert_eq!(6032, part1(&p));
  }

  #[test]
  fn test_edges() {
    let p = generator(input().as_str());
    let edges = p.make_edges(4);
    assert_eq!(14, edges.len());
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 8, y: 0 }, p2: XY { x: 11, y: 0 }, dir: XY { x: 0, y: -1 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 8, y: 0 }, p2: XY { x: 8, y: 3 }, dir: XY { x: -1, y: 0 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 11, y: 0 }, p2: XY { x: 11, y: 3 }, dir: XY { x: 1, y: 0 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 4, y: 4 }, p2: XY { x: 7, y: 4 }, dir: XY { x: 0, y: -1 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 0, y: 4 }, p2: XY { x: 3, y: 4 }, dir: XY { x: 0, y: -1 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 11, y: 4 }, p2: XY { x: 11, y: 7 }, dir: XY { x: 1, y: 0 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 0, y: 4 }, p2: XY { x: 0, y: 7 }, dir: XY { x: -1, y: 0 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 0, y: 7 }, p2: XY { x: 3, y: 7 }, dir: XY { x: 0, y: 1 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 4, y: 7 }, p2: XY { x: 7, y: 7 }, dir: XY { x: 0, y: 1 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 8, y: 8 }, p2: XY { x: 8, y: 11 }, dir: XY { x: -1, y: 0 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 12, y: 8 }, p2: XY { x: 15, y: 8 }, dir: XY { x: 0, y: -1 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 15, y: 8 }, p2: XY { x: 15, y: 11 }, dir: XY { x: 1, y: 0 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 8, y: 11 }, p2: XY { x: 11, y: 11 }, dir: XY { x: 0, y: 1 } }));
    assert_eq!(true, edges.contains(&Edge { p1: XY { x: 12, y: 11 }, p2: XY { x: 15, y: 11 }, dir: XY { x: 0, y: 1 } }));
  }

  #[test]
  fn test_links() {
    let p = generator(input().as_str());
    let edges = p.make_edges(4);
    let links = make_links(&edges);
    assert_eq!(edges.len(), links.len());
    assert_eq!(edges[links[0].1],
               Edge { p1: XY::new(11, 0), p2: XY::new(11, 3), dir: XY::new(1, 0) });
    assert_eq!(edges[links[0].0],
               Edge { p1: XY::new(8, 0), p2: XY::new(8, 3), dir: XY::new(-1, 0) });

    assert_eq!(edges[links[links[0].1].1],
               Edge { p1: XY::new(11, 4), p2: XY::new(11, 7), dir: XY::new(1, 0) });
    assert_eq!(edges[links[links[0].1].0],
               Edge { p1: XY::new(8, 0), p2: XY::new(11, 0), dir: XY::new(0, -1) });

    assert_eq!(edges[links[links[links[0].1].1].1],
               Edge { p1: XY::new(12, 8), p2: XY::new(15, 8), dir: XY::new(0, -1) });
    assert_eq!(edges[links[links[links[0].1].1].0],
               Edge { p1: XY::new(11, 0), p2: XY::new(11, 3), dir: XY::new(1, 0) });
  }

  #[test]
  fn test_mappings() {
    let p = generator(input().as_str());
    let edges = p.make_edges(4);
    let mappings = p.map_edges(4);
    assert_eq!(mappings.len(), edges.len() * 4);

    assert_eq!(mappings.get(&(XY::new(7, 4), XY::new(0, -1))).unwrap(),
               &(XY::new(8, 3), XY::new(1, 0)));
    assert_eq!(mappings.get(&(XY::new(8, 3), XY::new(-1, 0))).unwrap(),
               &(XY::new(7, 4), XY::new(0, 1)));

    assert_eq!(mappings.get(&(XY::new(7, 7), XY::new(0, 1))).unwrap(),
               &(XY::new(8, 8), XY::new(1, 0)));
    assert_eq!(mappings.get(&(XY::new(4, 7), XY::new(0, 1))).unwrap(),
               &(XY::new(8, 11), XY::new(1, 0)));

    assert_eq!(mappings.get(&(XY::new(12, 8), XY::new(0, -1))).unwrap(),
               &(XY::new(11, 7), XY::new(-1, 0)));
    assert_eq!(mappings.get(&(XY::new(11, 4), XY::new(1, 0))).unwrap(),
               &(XY::new(15, 8), XY::new(0, 1)));

    assert_eq!(mappings.get(&(XY::new(0, 4), XY::new(0, -1))).unwrap(),
               &(XY::new(11, 0), XY::new(0, 1)));
    assert_eq!(mappings.get(&(XY::new(3, 4), XY::new(0, -1))).unwrap(),
               &(XY::new(8, 0), XY::new(0, 1)));

    assert_eq!(mappings.get(&(XY::new(11, 0), XY::new(1, 0))).unwrap(),
               &(XY::new(15, 11), XY::new(-1, 0)));
    assert_eq!(mappings.get(&(XY::new(11, 3), XY::new(1, 0))).unwrap(),
               &(XY::new(15, 8), XY::new(-1, 0)));

    assert_eq!(mappings.get(&(XY::new(0, 4), XY::new(-1, 0))).unwrap(),
               &(XY::new(15, 11), XY::new(0, -1)));
    assert_eq!(mappings.get(&(XY::new(0, 7), XY::new(-1, 0))).unwrap(),
               &(XY::new(12, 11), XY::new(0, -1)));

    assert_eq!(mappings.get(&(XY::new(0, 7), XY::new(0, 1))).unwrap(),
               &(XY::new(11, 11), XY::new(0, -1)));
    assert_eq!(mappings.get(&(XY::new(3, 7), XY::new(0, 1))).unwrap(),
               &(XY::new(8, 11), XY::new(0, -1)));
  }

  #[test]
  fn test_mappings_1() {
    let input = vec![
      "    ...#...#",
      "    .#...#..",
      "    #...#...",
      "    ........",
      "    ...#",
      "    #...",
      "    ....",
      "    ..#.",
      ".......#",
      "....#...",
      "...#....",
      "......#.",
      "...#",
      "....",
      ".#..",
      "....",
      "",
      "10R5L5R10L4R5L5",
    ].join("\n");
    let p = generator(input.as_str());
    let edges = p.make_edges(4);
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(4, 0), p2: XY::new(7, 0), dir: XY::new(0, -1) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(8, 0), p2: XY::new(11, 0), dir: XY::new(0, -1) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(11, 0), p2: XY::new(11, 3), dir: XY::new(1, 0) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(4, 0), p2: XY::new(4, 3), dir: XY::new(-1, 0) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(8, 3), p2: XY::new(11, 3), dir: XY::new(0, 1) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(4, 4), p2: XY::new(4, 7), dir: XY::new(-1, 0) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(7, 4), p2: XY::new(7, 7), dir: XY::new(1, 0) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(0, 8), p2: XY::new(3, 8), dir: XY::new(0, -1) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(7, 8), p2: XY::new(7, 11), dir: XY::new(1, 0) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(0, 8), p2: XY::new(0, 11), dir: XY::new(-1, 0) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(4, 11), p2: XY::new(7, 11), dir: XY::new(0, 1) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(0, 12), p2: XY::new(0, 15), dir: XY::new(-1, 0) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(0, 15), p2: XY::new(3, 15), dir: XY::new(0, 1) }));
    assert_eq!(true, edges.contains(&Edge { p1: XY::new(3, 12), p2: XY::new(3, 15), dir: XY::new(1, 0) }));

    let mappings = p.map_edges(4);
    assert_eq!(edges.len() * 4, mappings.len());

    assert_eq!(mappings.get(&(XY::new(8, 3), XY::new(0, 1))).unwrap(),
               &(XY::new(7, 4), XY::new(-1, 0)));
    assert_eq!(mappings.get(&(XY::new(7, 7), XY::new(1, 0))).unwrap(),
               &(XY::new(11, 3), XY::new(0, -1)));

    assert_eq!(mappings.get(&(XY::new(4, 0), XY::new(-1, 0))).unwrap(),
               &(XY::new(0, 11), XY::new(1, 0)));
    assert_eq!(mappings.get(&(XY::new(4, 3), XY::new(-1, 0))).unwrap(),
               &(XY::new(0, 8), XY::new(1, 0)));

    assert_eq!(mappings.get(&(XY::new(4, 0), XY::new(0, -1))).unwrap(),
               &(XY::new(0, 12), XY::new(1, 0)));
    assert_eq!(mappings.get(&(XY::new(7, 0), XY::new(0, -1))).unwrap(),
               &(XY::new(0, 15), XY::new(1, 0)));

    assert_eq!(mappings.get(&(XY::new(0, 15), XY::new(0, 1))).unwrap(),
               &(XY::new(8, 0), XY::new(0, 1)));
    assert_eq!(mappings.get(&(XY::new(3, 15), XY::new(0, 1))).unwrap(),
               &(XY::new(11, 0), XY::new(0, 1)));
  }

  #[test]
  fn test_mappings_2() {
    let input = vec![
      "...#",
      ".#..",
      "#...",
      "....",
      "...#...#",
      "#....#..",
      "....#...",
      "..#.....",
      "    ...#...#",
      "    #...#...",
      "    ........",
      "    ..#...#.",
      "        ...#",
      "        .#..",
      "        #...",
      "        ....",
      "",
      "10R5L5R10L4R5L5",
    ].join("\n");

    let p = generator(input.as_str());
    let edges = p.make_edges(4);
    assert_eq!(14, edges.len());

    let mappings = p.map_edges(4);
    assert_eq!(edges.len() * 4, mappings.len());

    assert_eq!(mappings.get(&(XY::new(3, 0), XY::new(1, 0))).unwrap(),
               &(XY::new(7, 4), XY::new(0, 1)));
    assert_eq!(mappings.get(&(XY::new(3, 3), XY::new(1, 0))).unwrap(),
               &(XY::new(4, 4), XY::new(0, 1)));

    assert_eq!(mappings.get(&(XY::new(7, 4), XY::new(1, 0))).unwrap(),
               &(XY::new(11, 8), XY::new(0, 1)));
    assert_eq!(mappings.get(&(XY::new(7, 7), XY::new(1, 0))).unwrap(),
               &(XY::new(8, 8), XY::new(0, 1)));

    assert_eq!(mappings.get(&(XY::new(3, 0), XY::new(0, -1))).unwrap(),
               &(XY::new(11, 8), XY::new(-1, 0)));
    assert_eq!(mappings.get(&(XY::new(0, 0), XY::new(0, -1))).unwrap(),
               &(XY::new(11, 11), XY::new(-1, 0)));

    assert_eq!(mappings.get(&(XY::new(0, 0), XY::new(-1, 0))).unwrap(),
               &(XY::new(11, 12), XY::new(-1, 0)));
    assert_eq!(mappings.get(&(XY::new(0, 3), XY::new(-1, 0))).unwrap(),
               &(XY::new(11, 15), XY::new(-1, 0)));
  }

  #[test]
  fn test_part2() {
    let p = generator(input().as_str());
    assert_eq!(p.follow_cube_path(4).password(), 5031);
  }
}