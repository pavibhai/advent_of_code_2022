pub fn generator(input: &str) -> Vec<char> {
  input.lines().next().unwrap().chars().collect()
}

pub fn part1(message: &Vec<char>) -> usize {
  find_marker(message, 4)
}

pub fn part2(message: &Vec<char>) -> usize {
  find_marker(message, 14)
}

fn find_marker(msg: &Vec<char>, size: usize) -> usize {
  let mut m = Marker::new(size);
  for (i, c) in msg.iter().enumerate() {
    if m.add(*c) && i > (size - 2) {
      return i + 1;
    }
  }
  panic!("Unexpected, a marker was not found in {:?}", msg);
}

fn _find_marker_contains(msg: &Vec<char>, size: usize) -> usize {
  let mut i = 0;
  for s in msg.windows(size) {
    if _no_duplicates(s, size) {
      return i + size;
    }
    i += 1;
  }
  panic!("Unexpected, a marker was not found in {:?}", msg);
}

fn _no_duplicates(chars: &[char], size: usize) -> bool {
  for i in 0..(size - 1) {
    if chars[(i + 1)..].contains(&chars[i]) { return false; }
  }
  true
}

struct Marker {
  chars: Vec<char>,
  idx: usize,
  counts: Vec<usize>,
  dups: usize,
}

impl Marker {
  fn new(size: usize) -> Marker {
    let mut chars = Vec::new();
    for _ in 0..size {
      chars.push('a');
    }
    let mut counts = Vec::new();
    counts.push(size);
    for _ in 1..26 {
      counts.push(0);
    }

    Marker {
      chars,
      idx: 0,
      counts,
      dups: size - 1,
    }
  }
  fn add(&mut self, c: char) -> bool {
    // Remove previous
    let mut cidx: usize = self.chars[self.idx] as usize - 'a' as usize;
    self.counts[cidx] -= 1;
    if self.counts[cidx] > 0 {
      self.dups -= 1;
    }
    // Add new
    self.chars[self.idx] = c;
    cidx = self.chars[self.idx] as usize - 'a' as usize;
    self.counts[cidx] += 1;
    if self.counts[cidx] > 1 {
      self.dups += 1;
    }
    self.idx = (self.idx + 1) % self.chars.len();
    self.dups == 0
  }
}

#[cfg(test)]
mod tests {
  use crate::day6::{generator, Marker, part1, part2};

  #[test]
  fn test_part1() {
    assert_eq!(5, part1(&generator("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(6, part1(&generator("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(10, part1(&generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(11, part1(&generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
  }

  #[test]
  fn test_part2() {
    assert_eq!(19, part2(&generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    assert_eq!(23, part2(&generator("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(23, part2(&generator("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(29, part2(&generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(26, part2(&generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
  }

  #[test]
  fn test_marker() {
    let mut m = Marker::new(4);
    assert_eq!(false, m.add('a'));
    assert_eq!(3, m.dups);
    assert_eq!(4, m.counts[0]);
    assert_eq!(false, m.add('b'));
    assert_eq!(2, m.dups);
    assert_eq!(3, m.counts[0]);
    assert_eq!(1, m.counts[1]);
    assert_eq!(false, m.add('c'));
    assert_eq!(1, m.dups);
    assert_eq!(2, m.counts[0]);
    assert_eq!(1, m.counts[2]);
    assert_eq!(true, m.add('d'));
    assert_eq!(0, m.dups);
    assert_eq!(1, m.counts[0]);
    assert_eq!(1, m.counts[3]);
    assert_eq!(false, m.add('d'));
    assert_eq!(1, m.dups);
    assert_eq!(0, m.counts[0]);
    assert_eq!(2, m.counts[3]);
  }
}