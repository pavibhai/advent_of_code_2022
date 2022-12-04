pub fn part1(plays: &Vec<(usize, usize)>) -> usize {
  let counts: Vec<usize> = count_plays(plays);
  score_plays(&counts, &score_play)
}

pub fn part2(plays: &Vec<(usize, usize)>) -> usize {
  let counts: Vec<usize> = count_plays(plays);
  score_plays(&counts, &score_result)
}

fn score_plays(counts: &Vec<usize>, score: &dyn Fn(usize) -> usize) -> usize {
  counts.iter().enumerate()
    .map(|(p, c)| score(p) * c)
    .sum()
}

fn count_plays(plays: &Vec<(usize, usize)>) -> Vec<usize> {
  let mut counts: Vec<usize> = Vec::new();
  for _ in 0..9 {
    counts.push(0)
  }

  for (f, l) in plays {
    counts[to_idx(f, l)] += 1;
  }
  counts
}

pub fn generator(input: &str) -> Vec<(usize, usize)> {
  input.lines()
    .map(|p| {
      let mut chars = p.chars();
      let first = parse_play(chars.next());
      chars.next();
      let last = parse_play(chars.next());
      if chars.next().is_some() { panic!("Expecting only 3 characters in {}", p) }
      (first, last)
    }
    )
    .collect()
}

fn parse_play(p: Option<char>) -> usize {
  match p {
    Some('A') | Some('X') => 1,
    Some('B') | Some('Y') => 2,
    Some('C') | Some('Z') => 3,
    d => panic!("Unexpected char {:?}", d),
  }
}

fn to_idx(f: &usize, l: &usize) -> usize {
  (f - 1) * 3 + (l - 1)
}

fn from_idx(idx: usize) -> (usize, usize) {
  ((idx / 3) + 1, (idx % 3) + 1)
}

fn score_play(play: usize) -> usize {
  let (f, l) = from_idx(play);
  score(f, l)
}

fn score(f: usize, l: usize) -> usize {
  match f as isize - l as isize {
    0 => l + 3, // draw
    -1 | 2 => l + 6, // win
    _ => l + 0, // lose
  }
}

fn score_result(play: usize) -> usize {
  let (f, l) = from_idx(play);
  let p = match l {
    1 if f == 1 => 3,
    1 => f - 1, // to lose
    2 => f, // to draw
    3 if f == 3 => 1,
    3 => f + 1,
    _ => panic!("Unexpected value {}", l)
  };
  score(f, p)
}

#[cfg(test)]
mod tests {
  use crate::day2::{count_plays, score_play, score_plays, score_result, to_idx, generator};

  fn input() -> String {
    vec![
      "A Y",
      "B X",
      "C Z",
    ].join("\n")
  }

  #[test]
  fn test_part1() {
    let counts = count_plays(&generator(input().as_str()));
    assert_eq!(score_plays(&counts, &score_play), 15);
  }

  #[test]
  fn test_part2() {
    assert_eq!(score_result(to_idx(&1, &2)), 4);
    assert_eq!(score_result(to_idx(&2, &1)), 1);
    assert_eq!(score_result(to_idx(&3, &3)), 7);
    /*    let counts = count_plays(&parse(input().as_str()));
        assert_eq!(score_plays(&counts, &score_result), 12);*/
  }
}