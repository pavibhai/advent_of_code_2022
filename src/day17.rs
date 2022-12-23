use std::collections::{HashMap};

const CHAMBER_WIDTH: u8 = 7;
const STARTS_ABOVE: u8 = 3;
const DIRECTION_LEFT: char = '<';
const DIRECTION_RIGHT: char = '>';
const _MOST_SIG_BIT_SET: u8 = 1 << 6;

pub fn generator(input: &str) -> Puzzle {
  Puzzle::new(input.lines().next().unwrap().trim().chars().collect())
}

pub fn part1(puzzle: &Puzzle) -> u64 {
  let mut simulation = Simulation::new(puzzle.clone());
  simulation.run(2022) as u64
}

#[derive(Eq, PartialEq, Debug)]
struct CacheEntry {
  idx: u64,
  scope: Vec<u8>,
  change: usize,
}

struct Simulation {
  puzzle: Puzzle,
  jet_idx: usize,
  rock_idx: usize,
}

impl Simulation {
  fn new(puzzle: Puzzle) -> Simulation {
    Simulation {
      puzzle,
      jet_idx: 0,
      rock_idx: 0,
    }
  }
  fn next_jet(&mut self) -> &char {
    let r = &self.puzzle.jets[self.jet_idx];
    self.jet_idx = (self.jet_idx + 1) % self.puzzle.jets.len();
    r
  }

  fn next_rock(&mut self) -> Rock {
    let r = self.puzzle.rocks[self.rock_idx].clone();
    self.rock_idx = (self.rock_idx + 1) % self.puzzle.rocks.len();
    r
  }

  fn run(&mut self, rock_count: u64) -> u64 {
    self.rock_idx = 0;
    self.jet_idx = 0;
    let mut chamber: Chamber = Vec::new();
    let mut repeats: HashMap<(usize, usize), CacheEntry> = HashMap::new();
    let mut max_depth = 0;
    for i in 0..rock_count {
      let curr_key = (self.rock_idx, self.jet_idx);
      let curr_len = chamber.len();
      let rock = self.next_rock();
      let (scope, new_depth) = self.simulate_rock(rock, &mut chamber);
      if new_depth > max_depth {
        max_depth = new_depth;
        repeats.clear();
      }
      if let Some(cache_entry) = repeats.get(&curr_key) {
        if cache_entry.scope == scope {
          // We have found a repeat so short circuit the result
          let mut increases: Vec<(u64, usize)> = repeats.values().filter_map(|e| {
            if e.idx >= cache_entry.idx {
              Some((e.idx, e.change))
            } else {
              None
            }
          }).collect();
          increases.sort_by_key(|e| e.0);
          let increase_pass: u64 = increases.iter().map(|e| e.1 as u64).sum();
          let prev_height: u64 = (chamber.len() - cache_entry.change) as u64;
          let pass_size = i - cache_entry.idx;
          let remaining = rock_count - i;
          let remainder = (remaining % pass_size) as usize;
          let remaining_height_quotient = remaining / pass_size * increase_pass;
          let remaining_height_remainder: usize = increases[..remainder].iter().map(|e| e.1).sum();
          return prev_height as u64 + remaining_height_quotient as u64 + remaining_height_remainder as u64;
        } else {
          repeats.insert(curr_key, CacheEntry { idx: i, scope, change: chamber.len() - curr_len });
        }
      } else {
        repeats.insert(curr_key, CacheEntry { idx: i, scope, change: chamber.len() - curr_len });
      }
    }
    chamber.len() as u64
  }

  fn simulate_rock(&mut self, mut rock: Rock, chamber: &mut Chamber) -> (Vec<u8>, usize) {
    // As the rock starts 3 above, we will have 4 horizontal movements before we are top of the
    // existing rock pile
    for _ in 0..=STARTS_ABOVE {
      shift_rock(&mut rock, self.next_jet(), false);
    }

    let mut start = self.handle_into_existing_rocks(&mut rock, chamber);
    let result = if start == 0 {
      (chamber[start..].to_vec(), chamber.len() - start)
    } else {
      (chamber[start - 1..].to_vec(), chamber.len() - start + 1)
    };

    // Fill the chamber line with the rock
    while start < chamber.len() && !rock.is_empty() {
      let line = rock.pop().unwrap();
      if chamber[start] & line != 0 {
        panic!("Rock clashes with chamber");
      }
      chamber[start] |= line;
      start += 1;
    }
    // If we have any more rock left, place on top of the pile
    while !rock.is_empty() {
      chamber.push(rock.pop().unwrap());
    }
    result
  }

  fn handle_into_existing_rocks(&mut self, rock: &mut Rock, chamber: &Chamber) -> usize {
    // Start checking the top line
    let mut start = chamber.len();
    while start > 0 {
      // Start with the top rock line in chamber
      start -= 1;
      //println!("down: {allow_down} and shift: {allow_shift}");
      if !simulate_down(rock, chamber, start) {
        // We cannot move down, come to rest at current
        return start + 1;
      }

      let d = self.next_jet();
      if simulate_shift(rock, chamber, start, d) {
        shift_rock(rock, d, true);
      }
    }
    start
  }
}

fn simulate_down(rock: &Rock, chamber: &Chamber, start: usize) -> bool {
  let mut rock_idx = rock.len();
  let mut allow_down = true;
  // Check if we can move 1 level deeper into the existing pile
  for chamber_idx in start..(start + rock.len()).min(chamber.len()) {
    rock_idx -= 1;
    allow_down &= (chamber[chamber_idx] & rock[rock_idx]) == 0;
    if !allow_down {
      break;
    }
  }
  allow_down
}

fn simulate_shift(rock: &mut Rock, chamber: &Chamber, start: usize, direction: &char) -> bool {
  let mut allow_shift = if direction == &DIRECTION_LEFT {
    rock.iter().all(|l| l.leading_zeros() > 1)
  } else {
    rock.iter().all(|l| l.trailing_zeros() > 0)
  };
  let mut rock_idx = rock.len();
  // Check if a move is possible
  for c_i in start..(start + rock.len()).min(chamber.len()) {
    rock_idx -= 1;
    allow_shift &= can_shift(&rock[rock_idx], &chamber[c_i], direction);
    if !allow_shift {
      break;
    }
  }
  allow_shift
}

pub fn part2(puzzle: &Puzzle) -> u64 {
  let mut simulation = Simulation::new(puzzle.clone());
  simulation.run(1000000000000)
}

type Line = u8;
type Chamber = Vec<u8>;
type Rock = Vec<u8>;

fn make_line(input: &str) -> u8 {
  if input.len() != CHAMBER_WIDTH as usize {
    panic!("Expect the length of line to be 7 but received {input}");
  }
  let mut line = 0;
  for c in input.chars() {
    line <<= 1;
    match c {
      '#' => {
        line += 1;
      }
      '.' => {}
      _ => panic!("Unexpected character {c}")
    }
  }
  line
}

fn shift_left(rock: &mut Rock, skip_check: bool) -> bool {
  if skip_check || rock.iter().all(|l| l.leading_zeros() > 1) {
    rock.iter_mut().for_each(|l| *l <<= 1);
    true
  } else {
    false
  }
}

fn shift_right(rock: &mut Rock, skip_check: bool) -> bool {
  if skip_check || rock.iter().all(|l| l.trailing_zeros() > 0) {
    rock.iter_mut().for_each(|l| *l >>= 1);
    true
  } else {
    false
  }
}

fn can_move_left(rock_line: &Line, chamber_line: &Line) -> bool {
  rock_line.leading_zeros() > 1
    && ((rock_line << 1) & chamber_line == 0)
}

fn can_move_right(rock_line: &Line, chamber_line: &Line) -> bool {
  rock_line.trailing_zeros() > 0
    && ((rock_line >> 1) & chamber_line) == 0
}

fn can_shift(rock_line: &Line, chamber_line: &Line, direction: &char) -> bool {
  if direction == &DIRECTION_LEFT {
    can_move_left(rock_line, chamber_line)
  } else {
    can_move_right(rock_line, chamber_line)
  }
}

fn _line_to_string(line: &Line) -> String {
  let mut s = String::new();
  _line_into_string(line, &mut s);
  s
}

fn _line_into_string(line: &Line, out: &mut String) {
  for b in 0..CHAMBER_WIDTH {
    if (line << b) & _MOST_SIG_BIT_SET == 0 {
      out.push('.');
    } else {
      out.push('#');
    }
  }
}

fn _rock_to_string(lines: &Rock) -> String {
  let mut s = String::new();
  for line in lines {
    _line_into_string(&line, &mut s);
    s.push('\n');
  }
  s
}

fn _chamber_to_string(lines: &Chamber) -> String {
  let mut s = String::new();
  for line in lines.iter().rev() {
    _line_into_string(&line, &mut s);
    s.push('\n');
  }
  s
}

fn shift_rock(rock: &mut Vec<u8>, direction: &char, skip_check: bool) -> bool {
  if direction == &DIRECTION_LEFT {
    shift_left(rock, skip_check)
  } else {
    shift_right(rock, skip_check)
  }
}

#[derive(Clone)]
pub struct Puzzle {
  jets: Vec<char>,
  rocks: Vec<Rock>,
}

impl Puzzle {
  fn new(jets: Vec<char>) -> Puzzle {
    if jets.iter().any(|c| c != &DIRECTION_LEFT && c != &DIRECTION_RIGHT) {
      panic!("Unexpected jet patterns");
    }
    let mut rocks = Vec::new();
    let mut rock = Vec::new();
    rock.clear();
    rock.push(make_line("..####."));
    rocks.push(rock.clone());

    rock.clear();
    rock.push(make_line("...#..."));
    rock.push(make_line("..###.."));
    rock.push(make_line("...#..."));
    rocks.push(rock.clone());

    rock.clear();
    rock.push(make_line("....#.."));
    rock.push(make_line("....#.."));
    rock.push(make_line("..###.."));
    rocks.push(rock.clone());

    rock.clear();
    rock.push(make_line("..#...."));
    rock.push(make_line("..#...."));
    rock.push(make_line("..#...."));
    rock.push(make_line("..#...."));
    rocks.push(rock.clone());

    rock.clear();
    rock.push(make_line("..##..."));
    rock.push(make_line("..##..."));
    rocks.push(rock.clone());

    Puzzle {
      jets,
      rocks,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::day17::{Chamber, _chamber_to_string, generator, part1, _rock_to_string, shift_left, shift_right, Simulation, part2};

  const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

  #[test]
  fn test_generator() {
    let puzzle = generator(INPUT);
    assert_eq!(40, puzzle.jets.len());
    assert_eq!(&'>', puzzle.jets.first().unwrap());
    assert_eq!(&'>', puzzle.jets.last().unwrap());
    let mut itr = puzzle.jets.iter().cycle().skip(40);
    assert_eq!(&'>', itr.next().unwrap());
    assert_eq!(&'>', itr.next().unwrap());
    assert_eq!(&'>', itr.next().unwrap());
    assert_eq!(&'<', itr.next().unwrap());
  }

  #[test]
  fn test_rocks() {
    let puzzle = generator(INPUT);
    assert_eq!("..####.\n",
               _rock_to_string(&puzzle.rocks[0]));
    assert_eq!("...#...\n\
                ..###..\n\
                ...#...\n",
               _rock_to_string(&puzzle.rocks[1]));
    assert_eq!("....#..\n\
                ....#..\n\
                ..###..\n",
               _rock_to_string(&puzzle.rocks[2]));
    assert_eq!("..#....\n\
                ..#....\n\
                ..#....\n\
                ..#....\n",
               _rock_to_string(&puzzle.rocks[3]));
    assert_eq!("..##...\n\
                ..##...\n",
               _rock_to_string(&puzzle.rocks[4]));
  }

  #[test]
  fn test_shifts() {
    let puzzle = generator(INPUT);
    let mut rock = puzzle.rocks[0].clone();
    assert_eq!(true, shift_left(&mut rock, false));
    assert_eq!(".####..\n",
               _rock_to_string(&rock));
    assert_eq!(true, shift_left(&mut rock, false));
    assert_eq!("####...\n",
               _rock_to_string(&rock));
    assert_eq!(false, shift_left(&mut rock, false));
    assert_eq!("####...\n",
               _rock_to_string(&rock));

    rock = puzzle.rocks[1].clone();
    assert_eq!(true, shift_left(&mut rock, false));
    assert_eq!("..#....\n\
                .###...\n\
                ..#....\n",
               _rock_to_string(&rock));
    assert_eq!(true, shift_left(&mut rock, false));
    assert_eq!(".#.....\n\
                ###....\n\
                .#.....\n",
               _rock_to_string(&rock));
    assert_eq!(false, shift_left(&mut rock, false));
    assert_eq!(".#.....\n\
                ###....\n\
                .#.....\n",
               _rock_to_string(&rock));
    assert_eq!(true, shift_right(&mut rock, false));
    assert_eq!(true, shift_right(&mut rock, false));
    assert_eq!("...#...\n\
                ..###..\n\
                ...#...\n",
               _rock_to_string(&rock));
    assert_eq!(true, shift_right(&mut rock, false));
    assert_eq!(true, shift_right(&mut rock, false));
    assert_eq!(false, shift_right(&mut rock, false));
    assert_eq!(".....#.\n\
                ....###\n\
                .....#.\n",
               _rock_to_string(&rock));
  }

  #[test]
  fn test_handle_rock() {
    let puzzle = generator(INPUT);
    let mut chamber: Chamber = Vec::new();
    let mut simulation = Simulation::new(puzzle);
    let rock = simulation.next_rock();
    simulation.simulate_rock(rock, &mut chamber);
    assert_eq!(1, chamber.len());
    assert_eq!("..####.\n", _chamber_to_string(&chamber));
    let rock = simulation.next_rock();
    simulation.simulate_rock(rock, &mut chamber);
    assert_eq!(4, chamber.len());
    assert_eq!("...#...\n\
                ..###..\n\
                ...#...\n\
                ..####.\n",
               _chamber_to_string(&chamber));
    let rock = simulation.next_rock();
    simulation.simulate_rock(rock, &mut chamber);
    assert_eq!("..#....\n\
                ..#....\n\
                ####...\n\
                ..###..\n\
                ...#...\n\
                ..####.\n",
               _chamber_to_string(&chamber));
    let rock = simulation.next_rock();
    simulation.simulate_rock(rock, &mut chamber);
    assert_eq!("....#..\n\
                ..#.#..\n\
                ..#.#..\n\
                #####..\n\
                ..###..\n\
                ...#...\n\
                ..####.\n",
               _chamber_to_string(&chamber));
    let rock = simulation.next_rock();
    simulation.simulate_rock(rock, &mut chamber);
    assert_eq!("....##.\n\
                ....##.\n\
                ....#..\n\
                ..#.#..\n\
                ..#.#..\n\
                #####..\n\
                ..###..\n\
                ...#...\n\
                ..####.\n",
               _chamber_to_string(&chamber));
    let rock = simulation.next_rock();
    simulation.simulate_rock(rock, &mut chamber);
    assert_eq!(".####..\n\
                ....##.\n\
                ....##.\n\
                ....#..\n\
                ..#.#..\n\
                ..#.#..\n\
                #####..\n\
                ..###..\n\
                ...#...\n\
                ..####.\n",
               _chamber_to_string(&chamber));
  }

  #[test]
  fn test_part1() {
    let puzzle = generator(INPUT);
    let mut simulation = Simulation::new(puzzle.clone());
    assert_eq!(1, simulation.run(1));
    assert_eq!(4, simulation.run(2));
    assert_eq!(6, simulation.run(3));
    assert_eq!(7, simulation.run(4));
    assert_eq!(9, simulation.run(5));
    assert_eq!(10, simulation.run(6));
    assert_eq!(13, simulation.run(7));
    assert_eq!(15, simulation.run(8));
    assert_eq!(17, simulation.run(9));

    assert_eq!(3068, part1(&puzzle));
  }

  #[test]
  fn test_part2() {
    let puzzle = generator(INPUT);
    assert_eq!(1514285714288, part2(&puzzle));
  }
}