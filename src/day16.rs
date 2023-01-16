use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};

pub fn generator(input: &str) -> Puzzle {
  let (valves, mut distances, start) = parse(input);
  compute_distances(&valves, &mut distances, start);
  Puzzle::new(valves, distances, start)
}

pub fn part1(puzzle: &Puzzle) -> u32 {
  puzzle.compute_max_pressure(30)
}

pub fn part2(puzzle: &Puzzle) -> u32 {
  puzzle.compute_max_pressure_with_helper(26)
}

fn compute_distances(valves: &Vec<Valve>, distances: &mut Vec<Vec<u32>>, start: u32) {
  let mut queue = VecDeque::new();
  for from in 0..(valves.len() - 1) as u32 {
    if valves[from as usize].rate == 0 && from != start {
      // No need to compute for valves with flow rate of zero unless it is the start
      continue;
    }
    for to in (from + 1)..valves.len() as u32 {
      // Find distance between from and to
      if valves[to as usize].rate == 0 && to != start {
        // We don't care about distances between values with no rate of flow or if already
        // determined
        continue;
      }
      queue.clear();
      queue.push_back((from, 0));
      while !queue.is_empty() {
        let (curr, distance) = queue.pop_front().unwrap();
        if curr == to {
          break;
        }
        for curr_next in 0..valves.len() as u32 {
          if distances[curr as usize][curr_next as usize] == u32::MAX
            || curr_next == curr
            || curr_next == from {
            // Ignore distances that are not yet determined and ignore curr
            continue;
          }
          // Record distance between from and curr_next
          let d = distance + distances[curr as usize][curr_next as usize];
          distances[from as usize][curr_next as usize] = distances[from as usize][curr_next as usize].min(d);
          distances[curr_next as usize][from as usize] = distances[curr_next as usize][from as usize].min(d);
          queue.push_back((curr_next, distances[from as usize][curr_next as usize]));
        }
      }
    }
  }
}

fn parse(input: &str) -> (Vec<Valve>, Vec<Vec<u32>>, u32) {
  let mut values: Vec<(&str, u16, Vec<&str>)> = Vec::new();
  for line in input.lines() {
    let mut splits = line.split_whitespace();
    splits.next();
    let code = splits.next().unwrap();
    splits.next();
    splits.next();
    let rate: u16 = splits.next().unwrap().split_terminator(&['=', ';'])
                          .skip(1).next().unwrap().parse().expect("Expecting a number for rate");
    splits.next();
    splits.next();
    splits.next();
    splits.next();
    let connections: Vec<&str> = splits.map(|s| {
      s.split(',').next().unwrap()
    }).collect();
    values.push((code, rate, connections));
  }
  values.sort_by_key(|(_, rate, _)| Reverse(*rate));
  let mut map: HashMap<&str, u32> = HashMap::new();
  let mut valves: Vec<Valve> = Vec::new();
  for (id, (code, rate, _)) in values.iter().enumerate() {
    map.insert(code, id as u32);
    valves.push(Valve { id: id as u32, rate: *rate as u32, code: code.to_string() })
  }
  let mut distances: Vec<Vec<u32>> = vec![vec![u32::MAX; valves.len()]; valves.len()];
  for (id, (_, _, links)) in values.iter().enumerate() {
    for link in links {
      let link_id = map[link];
      distances[id][id] = 0;
      distances[id as usize][link_id as usize] = 1;
    }
  }
  (valves, distances, map["AA"])
}

#[derive(Debug, Eq, PartialEq)]
struct Valve {
  id: u32,
  code: String,
  rate: u32,
}

impl Valve {}

#[derive(Clone)]
struct BitMap {
  map: u64,
}

impl BitMap {
  fn _new() -> BitMap {
    BitMap { map: 0 }
  }

  fn _set(&mut self, idx: u32) {
    if idx < u64::BITS {
      self.map |= 1 << idx;
    } else {
      panic!()
    }
  }

  fn unset(&mut self, idx: u32) {
    if idx < u64::BITS {
      self.map &= u64::MAX ^ (1 << idx);
    } else {
      panic!()
    }
  }

  fn is_set(&self, idx: u32) -> bool {
    if idx < u64::BITS {
      self.map & (1 << idx) > 0
    } else {
      panic!()
    }
  }
}

pub struct Puzzle {
  valves: Vec<Valve>,
  distances: Vec<Vec<u32>>,
  start: u32,
  pressure_valves: u32,
}

impl Puzzle {
  fn new(valves: Vec<Valve>, distances: Vec<Vec<u32>>, start: u32) -> Puzzle {
    let pressure_valves = valves.iter().find(|v| v.rate == 0).unwrap().id;
    for i in 0..pressure_valves {
      let mut min = u32::MAX;
      for j in 0..pressure_valves {
        if i != j {
          min = min.min(distances[i as usize][j as usize]);
        }
      }
    }
    Puzzle {
      valves,
      distances,
      start,
      pressure_valves,
    }
  }
  fn compute_max_pressure(&self, until_time: u32) -> u32 {
    let mut max_pressure = 0;
    let mut stack: Vec<(u32, u32, Entry)> = Vec::new();
    stack.push((self.start, 0, Entry::new(self.pressure_valves)));
    while !stack.is_empty() {
      let (prev, time, entry) = stack.pop().unwrap();
      if entry.max_pressure_ignoring_travel(time, until_time, self) < max_pressure {
        continue;
      }
      let stack_size = stack.len();
      for i in (0..self.pressure_valves).rev() {
        if let Some(e) = process_entry(prev, time, &entry, i, until_time, &self) {
          stack.push(e);
        }
      }
      if stack_size == stack.len() {
        // No entries added
        max_pressure = max_pressure.max(entry.pressure);
      }
    }
    max_pressure
  }

  fn compute_max_pressure_with_helper(&self, until_time: u32) -> u32 {
    let mut max_pressure = 0;
    let mut stack: Vec<((u32, u32), (u32, u32), Entry)> = Vec::new();
    stack.push(((self.start, 0), (self.start, 0), Entry::new(self.pressure_valves)));
    while !stack.is_empty() {
      let ((prev1, time1), (prev2, time2), entry) = stack.pop().unwrap();
      if entry.max_pressure_ignoring_travel_with_helper(time1, time2, until_time, self) < max_pressure {
        continue;
      }
      let stack_size = stack.len();
      for i in (0..self.pressure_valves).rev() {
        // Move the one closest to it
        let r1 = process_entry(prev1, time1, &entry, i, until_time, &self);
        let r2 = process_entry(prev2, time2, &entry, i, until_time, &self);

        if r1.is_some() && self.distances[prev1 as usize][i as usize] < self.distances[prev2 as usize][i as usize] {
          let (p, t, e) = r1.unwrap();
          stack.push(((p, t), (prev2, time2), e));
        } else if r2.is_some() {
          let (p, t, e) = r2.unwrap();
          stack.push(((prev1, time1), (p, t), e));
        }
      }
      if stack_size == stack.len() {
        // No entries added
        max_pressure = max_pressure.max(entry.pressure);
      }
    }
    max_pressure
  }
}

fn process_entry(prev: u32, time: u32, entry: &Entry, to: u32, until_time: u32, puzzle: &Puzzle)
                 -> Option<(u32, u32, Entry)> {
  if !entry.remaining.is_set(to as u32) {
    return None;
  }
  let t = time + 1 + puzzle.distances[prev as usize][to as usize];
  if t < until_time {
    let mut entry = entry.clone();
    entry.remaining.unset(to);
    entry.pressure += (until_time - t) * puzzle.valves[to as usize].rate;
    Some((to, t, entry))
  } else {
    None
  }
}

#[derive(Clone)]
struct Entry {
  pressure: u32,
  remaining: BitMap,
}

impl Entry {
  fn new(pressure_valves: u32) -> Entry {
    let mut map: u64 = 0;
    for _ in 0..pressure_valves {
      map <<= 1;
      map += 1;
    }

    Entry {
      pressure: 0,
      remaining: BitMap { map },
    }
  }
  fn max_pressure_ignoring_travel(&self, time: u32, until_time: u32, puzzle: &Puzzle) -> u32 {
    let mut t = time + 2;
    let mut p = self.pressure;
    for i in 0..puzzle.pressure_valves {
      if !self.remaining.is_set(i) {
        continue;
      }
      if t < until_time {
        p += puzzle.valves[i as usize].rate as u32 * (until_time - t) as u32;
        t += 2;
      } else {
        break;
      }
    }
    p
  }

  fn max_pressure_ignoring_travel_with_helper(&self, mut time1: u32, mut time2: u32,
                                              until_time: u32, puzzle: &Puzzle) -> u32 {
    // Now we have two times received as input, for you and the helper
    let mut p = self.pressure;
    let mut itr = (0..puzzle.pressure_valves).into_iter();
    time1 += 2;
    time2 += 2;
    while time1 < until_time || time2 < until_time {
      match itr.next() {
        Some(i) if self.remaining.is_set(i) => {
          if time1 < time2 {
            p += puzzle.valves[i as usize].rate as u32 * (until_time - time1) as u32;
            time1 += 2;
          } else {
            p += puzzle.valves[i as usize].rate as u32 * (until_time - time2) as u32;
            time2 += 2;
          }
        }
        None => break,
        _ => {}
      }
    }
    p
  }
}

#[cfg(test)]
mod tests {
  use crate::day16::{BitMap, compute_distances, Entry, generator, parse, part2, Valve};

  fn input() -> String {
    vec![
      "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
      "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
      "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
      "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
      "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
      "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
      "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
      "Valve HH has flow rate=22; tunnel leads to valve GG",
      "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
      "Valve JJ has flow rate=21; tunnel leads to valve II",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let (valves, mut distances, start) = parse(input().as_str());
    let find_id = |code: &str| -> u32 {
      valves.iter().find(|v| v.code == code).unwrap().id
    };
    assert_eq!(10, valves.len());
    let mut idx = find_id("AA");
    assert_eq!(idx, start);
    assert_eq!(&Valve { id: idx as u32, rate: 0, code: "AA".to_string() }, &valves[idx as usize]);
    assert_eq!(0, distances[idx as usize][idx as usize]);
    assert_eq!(1, distances[idx as usize][find_id("BB") as usize]);
    assert_eq!(u32::MAX, distances[idx as usize][find_id("CC") as usize]);
    assert_eq!(1, distances[idx as usize][find_id("DD") as usize]);
    assert_eq!(1, distances[find_id("DD") as usize][idx as usize]);

    idx = find_id("JJ");
    assert_eq!(&Valve { id: idx, rate: 21, code: "JJ".to_string() }, &valves[idx as usize]);
    assert_eq!(u32::MAX, distances[idx as usize][find_id("AA") as usize]);
    assert_eq!(1, distances[idx as usize][find_id("II") as usize]);

    compute_distances(&valves, &mut distances, start);
    assert_eq!(0, distances[start as usize][start as usize]);
    assert_eq!(1, distances[start as usize][find_id("BB") as usize]);
    assert_eq!(2, distances[start as usize][find_id("CC") as usize]);
    assert_eq!(1, distances[start as usize][find_id("DD") as usize]);
    assert_eq!(2, distances[start as usize][find_id("EE") as usize]);
    assert_eq!(u32::MAX, distances[start as usize][find_id("FF") as usize]);
    assert_eq!(u32::MAX, distances[start as usize][find_id("GG") as usize]);
    assert_eq!(5, distances[start as usize][find_id("HH") as usize]);
    assert_eq!(1, distances[start as usize][find_id("II") as usize]);
    assert_eq!(2, distances[start as usize][find_id("JJ") as usize]);
    idx = find_id("HH");
    assert_eq!(0, distances[idx as usize][idx as usize]);
    assert_eq!(5, distances[idx as usize][start as usize]);
    assert_eq!(6, distances[idx as usize][find_id("II") as usize]);
    assert_eq!(7, distances[idx as usize][find_id("JJ") as usize]);
    assert_eq!(2, distances[find_id("CC") as usize][find_id("EE") as usize]);
    assert_eq!(4, distances[find_id("JJ") as usize][find_id("EE") as usize]);
  }

  #[test]
  fn test_bitmap() {
    let mut bitmap = BitMap::_new();
    assert_eq!(false, bitmap.is_set(0));
    bitmap._set(0);
    assert_eq!(true, bitmap.is_set(0));
    assert_eq!(false, bitmap.is_set(5));
    bitmap._set(5);
    assert_eq!(true, bitmap.is_set(5));
    assert_eq!(false, bitmap.is_set(3));
    assert_eq!(false, bitmap.is_set(15));
    bitmap._set(15);
    assert_eq!(true, bitmap.is_set(15));
  }

  #[test]
  fn test_part1() {
    let puzzle = generator(input().as_str());
    let mut map = 0;
    for _ in 0..puzzle.pressure_valves {
      map <<= 1;
      map += 1;
    }
    let entry = Entry { pressure: 0, remaining: BitMap { map } };
    assert_eq!(2024, entry.max_pressure_ignoring_travel(0, 30, &puzzle));
    assert_eq!(1651, puzzle.compute_max_pressure(30));
  }

  #[test]
  fn test_part2() {
    let puzzle = generator(input().as_str());
    let mut map = 0;
    for _ in 0..puzzle.pressure_valves {
      map <<= 1;
      map += 1;
    }
    let entry = Entry { pressure: 0, remaining: BitMap { map } };
    assert_eq!(1858, entry.max_pressure_ignoring_travel_with_helper(0, 0, 26, &puzzle));
    assert_eq!(1707, part2(&puzzle));
  }
}