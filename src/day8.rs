
pub fn generator(input: &str) -> Vec<Vec<i8>> {
  let trees: Vec<Vec<i8>> = input.lines()
    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
    .collect();
  if trees.iter().find(|t| t.len() != trees.len()).is_some() {
    panic!("Unexpected, input is not a square")
  }
  trees
}

fn compute_max(trees: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
  let size = trees.len();
  let mut maxs: Vec<Vec<i8>> = vec![vec![i8::MAX; size]; size];
  // Handle rows
  for i in 0..size {
    let mut m_lr = -1;
    let mut m_rl = -1;
    let mut m_ud = -1;
    let mut m_du = -1;
    let mut r;
    let mut c;
    // left -> right
    for j in 0..size {
      r = i;
      c = j;
      // left -> right
      maxs[r][c] = maxs[r][c].min(m_lr);
      m_lr = m_lr.max(trees[r][c]);
      // up -> down
      maxs[c][r] = maxs[c][r].min(m_ud);
      m_ud = m_ud.max(trees[c][r]);

      r = size - 1 - i;
      c = size - 1 - j;
      // right -> left
      maxs[r][c] = maxs[r][c].min(m_rl);
      m_rl = m_rl.max(trees[r][c]);
      // down -> up
      maxs[c][r] = maxs[c][r].min(m_du);
      m_du = m_du.max(trees[c][r]);
    }
  }
  maxs
}

fn compute_scenic_score(trees: &Vec<Vec<i8>>) -> Vec<Vec<u32>> {
  let size = trees.len();
  let mut scenic_score: Vec<Vec<u32>> = vec![vec![1; size]; size];
  // Handle rows
  let mut m_lr: Vec<u32> = vec![0; 10];
  let mut m_rl: Vec<u32> = vec![0; 10];
  let mut m_ud: Vec<u32> = vec![0; 10];
  let mut m_du: Vec<u32> = vec![0; 10];
  for i in 0..size {
    m_lr.fill(0);
    m_rl.fill(0);
    m_ud.fill(0);
    m_du.fill(0);
    let mut r: usize;
    let mut c: usize;
    // left -> right
    for j in 0..size {
      r = i;
      c = j;
      // left -> right
      scenic_score[r][c] = scenic_score[r][c] *
        (j as u32 - m_lr[trees[r][c] as usize..].iter().max().unwrap());
      m_lr[trees[r][c] as usize] = j as u32;
      // up -> down
      scenic_score[c][r] = scenic_score[c][r] *
        (j as u32 - m_ud[trees[c][r] as usize..].iter().max().unwrap());
      m_ud[trees[c][r] as usize] = j as u32;

      r = size - 1 - i;
      c = size - 1 - j;
      // right -> left
      scenic_score[r][c] = scenic_score[r][c] *
        (j as u32 - m_rl[trees[r][c] as usize..].iter().max().unwrap());
      m_rl[trees[r][c] as usize] = j as u32;
      // down -> up
      scenic_score[c][r] = scenic_score[c][r] *
        (j as u32 - m_du[trees[c][r] as usize..].iter().max().unwrap());
      m_du[trees[c][r] as usize] = j as u32;
    }
  }
  scenic_score
}

pub fn part1(trees: &Vec<Vec<i8>>) -> isize {
  let size = trees.len();
  let maxs = compute_max(&trees);
  let mut count = 0;
  for i in 0..size {
    for j in 0..size {
      if trees[i][j] > maxs[i][j] {
        count += 1;
      }
    }
  }
  count
}

pub fn part2(trees: &Vec<Vec<i8>>) -> u32 {
  let scenic_scores = compute_scenic_score(&trees);
  *scenic_scores.iter().flatten().max().unwrap()
}

#[cfg(test)]
mod tests {
  use crate::day8::{compute_max, compute_scenic_score, generator, part1, part2};

  fn input() -> String {
    vec![
      "30373",
      "25512",
      "65332",
      "33549",
      "35390",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let trees = generator(input().as_str());
    assert_eq!(5, trees.len());
    assert_eq!(None, trees.iter().find(|l| l.len() != 5));
  }

  #[test]
  fn test_maxs() {
    let trees = generator(input().as_str());
    let maxs = compute_max(&trees);

    assert_eq!(vec![-1; 5], maxs[0]);
    assert_eq!(vec![-1, 0, 2, 2, -1], maxs[1]);
    assert_eq!(vec![-1, 3, 3, 2, -1], maxs[2]);
    assert_eq!(vec![-1, 3, 3, 5, -1], maxs[3]);
    assert_eq!(vec![-1; 5], maxs[4]);
  }

  #[test]
  fn test_part1() {
    let trees = generator(input().as_str());
    assert_eq!(21, part1(&trees));
  }

  #[test]
  fn test_scenic_score() {
    let trees = generator(input().as_str());
    let scores = compute_scenic_score(&trees);
    assert_eq!(4, scores[1][2]);
    assert_eq!(8, scores[3][2]);
  }

  #[test]
  fn test_part2() {
    let trees = generator(input().as_str());
    assert_eq!(8, part2(&trees));
  }
}