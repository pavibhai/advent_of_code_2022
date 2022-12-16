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
    sensor.no_beacons(y, &mut no_beacons, false);
  }
  no_beacons.sort();
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
  let high: u32 = 4000000;
  let pos = find_beacon_pos(&sensors, 0, high);
  (high as u64 * pos.0 as u64) + pos.1 as u64
}

fn find_beacon_pos(sensors: &Vec<Sensor>, low: u32, high: u32) -> (i32, i32) {
  let mut exclusions = Vec::new();
  for y in low..=high {
    exclusions.clear();
    for sensor in sensors {
      sensor.no_beacons(y as i32, &mut exclusions, true);
    }
    exclusions.retain_mut(|(x1, x2)| {
      if *x2 < low as i32 || *x1 > high as i32 {
        false
      } else {
        *x1 = (*x1).max(low as i32);
        *x2 = (*x2).min(high as i32);
        true
      }
    });
    exclusions.sort();
    let mut curr_max = exclusions.first().unwrap().0;
    for (x1, x2) in &exclusions {
      if curr_max < *x1 {
        return (curr_max + 1, y as i32);
      }
      curr_max = curr_max.max(*x2);
    }
  }
  (0, 0)
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
  pub(crate) x: i32,
  pub(crate) y: i32,
}

pub struct Sensor {
  position: Position,
  beacon: Position,
  manhattan_distance: u32,
}

impl Sensor {
  fn new(s_x: i32, s_y: i32, b_x: i32, b_y: i32) -> Sensor {
    let sensor = Position { x: s_x, y: s_y };
    let beacon = Position { x: b_x, y: b_y };
    let manhattan_distance = sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y);
    Sensor {
      position: sensor,
      beacon,
      manhattan_distance,
    }
  }

  fn no_beacons(&self, y: i32, result: &mut Vec<(i32, i32)>, include_beacon: bool) {
    let delta_distance = self.manhattan_distance as i32 - y.abs_diff(self.position.y) as i32;
    if delta_distance < 0 {
      return;
    }
    let x_range = (self.position.x - delta_distance, self.position.x + delta_distance);
    if include_beacon || self.beacon.x < x_range.0 || self.beacon.x > x_range.1 || y != self.beacon.y {
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
}

#[cfg(test)]
mod tests {
  use crate::day15::{find_beacon_pos, generator, no_beacons, Sensor};

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
  fn  test_distress_beacon() {
    let sensors = generator(input().as_str());
    assert_eq!((14, 11), find_beacon_pos(&sensors, 0, 20));
  }
}