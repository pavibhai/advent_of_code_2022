use std::fmt::{Display, Formatter};
use std::ops::Neg;
use regex::Regex;

const LINE_PATTERN: &str = "Sensor at x=([^,]+), y=([^:]+): closest beacon is at x=([^,]+), y=(\\S+)";

pub fn generator(input: &str) -> Vec<Sensor> {
  let mut result: Vec<Sensor> = Vec::new();
  let line_pattern = Regex::new(LINE_PATTERN).unwrap();
  for line in input.lines() {
    let matches = line_pattern.captures(line).unwrap();
    result.push(Sensor::new(
      matches.get(1).unwrap().as_str().parse().expect("Expecting number"),
      matches.get(2).unwrap().as_str().parse().expect("Expecting number"),
      matches.get(3).unwrap().as_str().parse().expect("Expecting number"),
      matches.get(4).unwrap().as_str().parse().expect("Expecting number"))
    );
  }
  result
}

pub fn part1(sensors: &Vec<Sensor>) -> u64 {
  no_beacons(sensors, 2000000)
}

fn no_beacons(sensors: &Vec<Sensor>, y: i32) -> u64 {
  let mut no_beacons: Vec<(i32, i32)> = Vec::new();
  for sensor in sensors {
    sensor.no_beacons(y, &mut no_beacons);
  }
  no_beacons.sort_by_key(|x| x.0);
  let mut count = 0;
  let mut max_x = i32::MIN;
  for (x1, x2) in no_beacons {
    if x2 <= max_x {
      continue;
    } else if x1 <= max_x {
      count += x2 - max_x;
    } else {
      count += x2 - x1 + 1;
    }
    max_x = max_x.max(x2);
  }
  count as u64
}

pub fn part2(sensors: &Vec<Sensor>) -> u64 {
  let low = 0;
  let high = 4000000;
  let pos = find_missing_beacon(sensors, low, high);
  println!("{:?}", pos);
  (high as u64 * pos.0 as u64) + pos.1 as u64
}

fn find_missing_beacon(sensors: &Vec<Sensor>, min: i32, max: i32) -> (i32, i32) {
  let mut same_pairs: Vec<(Line,Line)> = Vec::new();
  let mut opp_pairs: Vec<(Line,Line)> = Vec::new();
  for x in 0..sensors.len() {
    for y in x + 1..(sensors.len() - 1) {
      match sensors[x].lines(&sensors[y]) {
        Some((l1, l2, -1)) => opp_pairs.push((l1, l2)),
        Some((l1, l2, 1)) => same_pairs.push((l1, l2)),
        _ => {},
      }
    }
  }

  // Find intersection points
  for (sl1, sl2) in &same_pairs {
    for (ol1, ol2) in &opp_pairs {
      let p11 = sl1.meet_at(ol1);
      if p11.is_none() {
        continue;
      }
      let p12 = sl1.meet_at(ol2);
      if p12.is_none() {
        continue;
      }
      let p21 = sl2.meet_at(ol1);
      if p21.is_none() {
        continue;
      }
      let p22 = sl2.meet_at(ol2);
      if p22.is_none() {
        continue;
      }
      let x = p11.unwrap().x.min(p12.unwrap().x.min(p21.unwrap().x.min(p22.unwrap().x))) + 1;
      let y = p11.unwrap().y.min(p12.unwrap().y.min(p21.unwrap().y.min(p22.unwrap().y))) + 1;
      let p = Position{x, y};
      let mut found = true;
      for sensor in sensors {
        if sensor.position.distance_between(&p) <= sensor.range {
          found = false;
          break;
        }
      }
      if found {
        return (x, y)
      }
    }
  }

  //TODO Scan the edges

  panic!("Did not find missing beacon");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
  pub(crate) x: i32,
  pub(crate) y: i32,
}

impl Position {
  fn distance_between(&self, other: &Position) -> u32 {
    self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

#[derive(Debug)]
pub struct Sensor {
  position: Position,
  beacon: Position,
  range: u32,
}

impl Sensor {
  fn new(s_x: i32, s_y: i32, b_x: i32, b_y: i32) -> Sensor {
    let sensor = Position { x: s_x, y: s_y };
    let beacon = Position { x: b_x, y: b_y };
    let manhattan_distance = sensor.distance_between(&beacon);
    Sensor {
      position: sensor,
      beacon,
      range: manhattan_distance,
    }
  }

  fn range(&self, y: i32, min: i32, max: i32, strengths: &mut Vec<(i32, i32, u32)>) {
    let y_diff = y.abs_diff(self.position.y);
    let delta_distance = self.range as i32 - y_diff as i32;
    if delta_distance < 0 {
      return;
    }
    let mut x_min = self.position.x - delta_distance;
    let mut x_max = self.position.x + delta_distance;
    let mut depth = if y < self.position.y {
      y_diff * 2
    } else {
      self.range - y_diff - (self.position.x - x_min) as u32
    };
    let mut min_strength = depth;
    if x_min < min {
      min_strength = min_strength.min(depth + (min - x_min) as u32);
      x_min = min;
    }
    if x_max > max {
      min_strength = min_strength.min(depth + (x_max - max) as u32);
      x_max = max;
    }
    if x_min <= x_max {
      strengths.push((x_min, x_max, min_strength));
    }
  }

  fn no_beacons(&self, y: i32, result: &mut Vec<(i32, i32)>) {
    let delta_distance = self.range as i32 - y.abs_diff(self.position.y) as i32;
    if delta_distance < 0 {
      return;
    }
    let x_range = (self.position.x - delta_distance, self.position.x + delta_distance);
    if self.beacon.x < x_range.0 || self.beacon.x > x_range.1 || y != self.beacon.y {
      result.push(x_range);
    } else {
      if x_range.0 == self.beacon.x && x_range.1 == self.beacon.x {
        // nothing to add this is a beacon
      } else if x_range.0 == self.beacon.x {
        result.push((x_range.0 + 1, x_range.1));
      } else if x_range.1 == self.beacon.x {
        result.push((x_range.0, x_range.1 - 1));
      } else {
        result.push((x_range.0, self.beacon.x - 1));
        result.push((self.beacon.x + 1, x_range.1));
      }
    }
  }

  fn lines(&self, other: &Sensor) -> Option<(Line, Line, i32)> {
    if self.position.x == other.position.x
      || self.position.y == other.position.y
      || (self.position.distance_between(&other.position) as i32 - self.range as i32 - other.range as i32) != 2 {
      return None;
    }
    let x_dir = (other.position.x - self.position.x).signum();
    let y_dir = (other.position.y - self.position.y).signum();
    let s1 = Position { x: self.position.x + (x_dir * self.range as i32), y: self.position.y };
    let s2 = Position { x: self.position.x, y: self.position.y + (y_dir * self.range as i32) };
    let o1 = Position { x: other.position.x + (x_dir.neg() * other.range as i32), y: other.position.y };
    let o2 = Position { x: other.position.x, y: other.position.y + (y_dir.neg() * other.range as i32) };
    Some((Line::new(s1, s2), Line::new(o1, o2), x_dir * y_dir))
  }
}

#[derive(Debug, Eq, PartialEq)]
struct Line {
  start: Position,
  end: Position,
  x_dir: i32,
  y_dir: i32,
}

fn meet_at(same_dir: &Line, opp_dir: &Line) -> Option<Position> {
  let x_diff = same_dir.start.x.abs_diff(opp_dir.start.x);
  let (s_pos, o_pos) = if same_dir.start.x < opp_dir.start.x {
    (same_dir.move_forward(x_diff), opp_dir.start)
  } else {
    (same_dir.start, opp_dir.move_forward(x_diff))
  };
  assert_eq!(s_pos.x, o_pos.x);
  if s_pos.x > same_dir.end.x
    || s_pos.y > same_dir.end.y
    || o_pos.x > opp_dir.end.x
    || o_pos.y < opp_dir.end.y {
    return None;
  }

  let rem_change = (s_pos.y.abs_diff(o_pos.y) as i32) / 2;
  if s_pos.x + rem_change <= same_dir.end.x && s_pos.y + rem_change <= same_dir.end.y
    && o_pos.x + rem_change <= opp_dir.end.x && o_pos.y - rem_change >= opp_dir.end.y {
    Some(Position { x: s_pos.x + rem_change, y: s_pos.y + rem_change })
  } else {
    None
  }
}

impl Line {
  fn meet_at(&self, other: &Line) -> Option<Position> {
    if self.x_dir * self.y_dir == other.x_dir * other.y_dir
      || self.start.x > other.end.x
      || self.end.x < other.start.x {
      // Same direction, will not intersect
      return None;
    }

    if self.x_dir * self.y_dir == 1 {
      meet_at(self, other)
    } else {
      meet_at(other, self)
    }
  }

  fn move_forward(&self, times: u32) -> Position {
    Position {
      x: self.start.x + self.x_dir * times as i32,
      y: self.start.y + self.y_dir * times as i32,
    }
  }

  fn new(p1: Position, p2: Position) -> Line {
    if p1.x < p2.x {
      Line {
        start: p1,
        end: p2,
        x_dir: (p2.x - p1.x).signum(),
        y_dir: (p2.y - p1.y).signum(),
      }
    } else {
      Line {
        start: p2,
        end: p1,
        x_dir: (p1.x - p2.x).signum(),
        y_dir: (p1.y - p2.y).signum(),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::day15::{find_missing_beacon, generator, Line, no_beacons, Position, Sensor};

  fn input() -> String {
    vec![
      "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
      "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
      "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
      "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
      "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
      "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
      "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
      "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
      "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
      "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
      "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
      "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
      "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
      "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let sensors = generator(input().as_str());
    assert_eq!(14, sensors.len());
  }

  #[test]
  fn test_no_beacons() {
    let sensor = Sensor::new(8, 7, 2, 10);
    assert_eq!(12, no_beacons(&vec![sensor], 10));

    let sensors = generator(input().as_str());
    assert_eq!(26, no_beacons(&sensors, 10));
  }

  #[test]
  fn test_distress_beacon() {
    let sensor = Sensor::new(8, 7, 2, 10);
    let mut ranges = Vec::new();
    sensor.range(7, -1, 20, &mut ranges);
    assert_eq!(&(-1, 17, 0), ranges.last().unwrap());

    sensor.range(7, 0, 20, &mut ranges);
    assert_eq!(&(0, 17, 0), ranges.last().unwrap());

    sensor.range(8, 0, 20, &mut ranges);
    assert_eq!(&(0, 16, 0), ranges.last().unwrap());

    sensor.range(6, 0, 20, &mut ranges);
    assert_eq!(&(0, 16, 2), ranges.last().unwrap());

    let sensor = Sensor::new(20, 1, 15, 3);
    assert_eq!(7, sensor.range);
    sensor.range(6, 0, 20, &mut ranges);
    assert_eq!(&(18, 20, 0), ranges.last().unwrap());

    let sensors = generator(input().as_str());
    assert_eq!((14, 11), find_missing_beacon(&sensors, 0, 20));
  }

  #[test]
  fn test_potential_lines() {
    let s1 = Sensor::new(12, 14, 16, 14);
    assert_eq!(4, s1.range);
    let s2 = Sensor::new(16, 7, 16, 12);
    assert_eq!(5, s2.range);

    let lines = s1.lines(&s2);
    assert_eq!(true, lines.is_some());
    let (l1, l2, d) = lines.unwrap();
    assert_eq!(-1, d);
    assert_eq!(Line::new(Position { x: 16, y: 14 }, Position { x: 12, y: 10 }), l1);
    assert_eq!(Line::new(Position { x: 11, y: 7 }, Position { x: 16, y: 12 }), l2);

    let s3 = Sensor::new(8, 7, 17, 7);
    assert_eq!(9, s3.range);
    let s4 = Sensor::new(20, 14, 28, 14);
    assert_eq!(8, s4.range);

    let lines = s3.lines(&s4);
    assert_eq!(true, lines.is_some());
    let (l1, l2, d) = lines.unwrap();
    assert_eq!(1, d);
    assert_eq!(Line::new(Position { x: 17, y: 7 }, Position { x: 8, y: 16 }), l1);
    assert_eq!(Line::new(Position { x: 12, y: 14 }, Position { x: 20, y: 6 }), l2);

    assert_eq!(None, s3.lines(&s1));

    let l1 = Line::new(Position { x: 17, y: 7 }, Position { x: 8, y: 16 });
    let l2 = Line::new(Position { x: 11, y: 7 }, Position { x: 16, y: 12 });
    let m = l1.meet_at(&l2);
    assert_eq!(Position { x: 14, y: 10 }, m.unwrap());

    let l1 = Line::new(Position { x: 12, y: 10 }, Position { x: 16, y: 14 });
    let l2 = Line::new(Position { x: 8, y: 16 }, Position { x: 17, y: 7 });
    let m = l1.meet_at(&l2);
    assert_eq!(Position { x: 13, y: 11 }, m.unwrap());

    let l1 = Line::new(Position { x: 12, y: 10 }, Position { x: 16, y: 14 });
    let l2 = Line::new(Position { x: 12, y: 14 }, Position { x: 20, y: 6 });
    let m = l1.meet_at(&l2);
    assert_eq!(Position { x: 14, y: 12 }, m.unwrap());

    let l1 = Line::new(Position { x: 11, y: 7 }, Position { x: 16, y: 12 });
    let l2 = Line::new(Position { x: 12, y: 14 }, Position { x: 20, y: 6 });
    let m = l1.meet_at(&l2);
    assert_eq!(Position { x: 15, y: 11 }, m.unwrap());
  }
}