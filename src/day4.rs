pub fn generator(input: &str) -> Vec<((i32, i32), (i32, i32))> {
  input.lines()
    .map(|line| {
      let mut tokens = line.split_terminator(&[',', '-'])
        .map(|s| s.parse().expect("Expecting a number"));
      ((tokens.next().unwrap(), tokens.next().unwrap()), (tokens.next().unwrap(), tokens.next().unwrap()))
    }).collect()
}

pub fn part1(sas: &Vec<((i32, i32), (i32, i32))>) -> i32 {
  let mut c = 0;
  for (sa1, sa2) in sas {
    if includes(sa1, sa2) {
      c += 1;
    }
  }
  c
}

pub fn part2(sas: &Vec<((i32, i32), (i32, i32))>) -> i32 {
  let mut c = 0;
  for (sa1, sa2) in sas {
    if overlaps(sa1, sa2) {
      c += 1;
    }
  }
  c
}

fn includes(sa1: &(i32, i32), sa2: &(i32, i32)) -> bool {
  let l = sa1.0 - sa2.0;
  let h = sa2.1 - sa1.1;
  l == 0 || h == 0 || (l < 0) == (h < 0)
}

fn overlaps(sa1: &(i32, i32), sa2: &(i32, i32)) -> bool {
  let l = sa1.0 - sa2.1;
  let h = sa2.0 - sa1.1;
  (l <= 0) == (h <= 0)
}

#[cfg(test)]
mod tests {
  use crate::day4::{generator, includes, overlaps, part1, part2};

  fn input() -> String {
    vec![
      "2-4,6-8",
      "2-3,4-5",
      "5-7,7-9",
      "2-8,3-7",
      "6-6,4-6",
      "2-6,4-8",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let sa = generator(input().as_str());
    assert_eq!(6, sa.len());
    assert_eq!(&((2, 4), (6, 8)), sa.first().unwrap());
    assert_eq!(&((2, 6), (4, 8)), sa.last().unwrap());
  }

  #[test]
  fn test_part1() {
    assert_eq!(2, part1(&generator(input().as_str())))
  }

  #[test]
  fn test_part2() {
    assert_eq!(4, part2(&generator(input().as_str())))
  }

  #[test]
  fn test_includes() {
    assert_eq!(true, includes(&(1,1), &(1,1)));
    assert_eq!(true, includes(&(1,2), &(1,1)));
    assert_eq!(true, includes(&(1,1), &(1,2)));
    assert_eq!(true, includes(&(0,2), &(1,1)));
    assert_eq!(true, includes(&(1,1), &(0,2)));
    assert_eq!(false, includes(&(1,2), &(2,3)));
    assert_eq!(false, includes(&(2,3), &(1,2)));
    assert_eq!(false, includes(&(1,2), &(3,4)));
    assert_eq!(false, includes(&(3,4), &(1,2)));
    assert_eq!(false, includes(&(1,3), &(2,4)));
    assert_eq!(false, includes(&(2,4), &(1,3)));
  }

  #[test]
  fn test_overlaps() {
    assert_eq!(true, overlaps(&(1,1), &(1,1)));
    assert_eq!(true, overlaps(&(1,2), &(1,1)));
    assert_eq!(true, overlaps(&(1,1), &(1,2)));
    assert_eq!(true, overlaps(&(0,2), &(1,1)));
    assert_eq!(true, overlaps(&(1,1), &(0,2)));
    assert_eq!(true, overlaps(&(1,2), &(2,3)));
    assert_eq!(true, overlaps(&(2,3), &(1,2)));
    assert_eq!(false, overlaps(&(1,2), &(3,4)));
    assert_eq!(false, overlaps(&(3,4), &(1,2)));
    assert_eq!(true, overlaps(&(1,3), &(2,4)));
    assert_eq!(true, overlaps(&(2,4), &(1,3)));
  }
}