use std::cell::RefCell;
use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<(char, i32)> {
  input.lines()
    .map(|l| {
      let mut splits = l.split_whitespace();
      (splits.next().unwrap().chars().next().unwrap(),
       splits.next().unwrap().parse().expect("Expecting a number"))
    }).collect()
}

pub fn part1(moves: &Vec<(char, i32)>) -> usize {
  let mut visit_pos: HashSet<(i32, i32)> = HashSet::new();
  let mut tail: (i32, i32) = (0, 0);
  let mut head: (i32, i32) = (0, 0);
  visit_pos.insert(tail);

  for (dir, steps) in moves {
    move_head(&mut head, dir, steps);
    move_knot_w_record(&mut tail, &head, &mut visit_pos);
  }
  visit_pos.len()
}

fn move_head(head: &mut (i32, i32), dir: &char, steps: &i32) {
  match dir {
    'R' => head.0 += steps,
    'L' => head.0 -= steps,
    'U' => head.1 += steps,
    'D' => head.1 -= steps,
    _ => panic!("Unexpected direction {dir}"),
  }
}

fn record_position(pos: &(i32, i32), positions: &mut HashSet<(i32, i32)>) {
  positions.insert(*pos);
}

fn dont_record_position(_pos: &(i32, i32), _positions: &mut HashSet<(i32, i32)>) {
  // do nothing
}

fn move_knot_w_record(knot: &mut (i32, i32), prev: &(i32, i32),
                      visited_pos: &mut HashSet<(i32, i32)>) -> bool {
  move_knot(knot, prev, visited_pos, record_position)
}

fn move_knot_wo_record(knot: &mut (i32, i32), prev: &(i32, i32),
                       visited_pos: &mut HashSet<(i32, i32)>) -> bool {
  move_knot(knot, prev, visited_pos, dont_record_position)
}

fn move_knot(knot: &mut (i32, i32), prev: &(i32, i32),
             visited_pos: &mut HashSet<(i32, i32)>,
             record: fn(&(i32, i32), &mut HashSet<(i32, i32)>)) -> bool {
  let x_diff = knot.0.abs_diff(prev.0);
  let y_diff = knot.1.abs_diff(prev.1);
  match (x_diff, y_diff) {
    (x_diff, y_diff) if x_diff < 2 && y_diff < 2 => {
      // Touching, no movement required
      return false;
    }
    (0, y_diff) => {
      // Same row
      let y_c = (prev.1 - knot.1) / y_diff as i32;
      for _ in 0..(y_diff - 1) {
        knot.1 += y_c;
        record(knot, visited_pos);
      }
    }
    (x_diff, 0) => {
      // Same col
      let x_c = (prev.0 - knot.0) / x_diff as i32;
      for _ in 0..(x_diff - 1) {
        knot.0 += x_c;
        record(knot, visited_pos);
      }
    }
    (mut x_diff, mut y_diff) => {
      let x_c = (prev.0 - knot.0) / x_diff as i32;
      let y_c = (prev.1 - knot.1) / y_diff as i32;
      while x_diff > 0 && y_diff > 0 && !(x_diff == 1 && y_diff == 1) {
        knot.0 += x_c;
        knot.1 += y_c;
        record(knot, visited_pos);
        x_diff = knot.0.abs_diff(prev.0);
        y_diff = knot.1.abs_diff(prev.1);
      }
      while knot.0.abs_diff(prev.0) > 1 {
        knot.0 += x_c;
        record(knot, visited_pos);
      }
      while knot.1.abs_diff(prev.1) > 1 {
        knot.1 += y_c;
        record(knot, visited_pos);
      }
    }
  }
  true
}

pub fn part2(moves: &Vec<(char, i32)>) -> usize {
  let mut visit_pos: HashSet<(i32, i32)> = HashSet::new();
  let knots_count = 10;
  let knots: Vec<RefCell<(i32, i32)>> = vec![RefCell::from((0, 0)); knots_count];
  visit_pos.insert(*knots.last().unwrap().borrow());
  let mut needs_move;

  for (dir, steps) in moves {
    for _ in 0..*steps {
      move_head(&mut knots.first().unwrap().borrow_mut(), dir, &1);
      needs_move = true;
      for i in 1..knots_count - 1 {
        needs_move = move_knot_wo_record(&mut knots.get(i).unwrap().borrow_mut(),
                                         &knots.get(i - 1).unwrap().borrow(),
                                         &mut visit_pos);
        if !needs_move {
          break;
        }
      }
      if needs_move {
        move_knot_w_record(&mut knots.last().unwrap().borrow_mut(),
                           &knots.get(knots_count - 2).unwrap().borrow(),
                           &mut visit_pos);
      }
    }
  }
  visit_pos.len()
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;
  use crate::day9::{generator, move_knot_wo_record, part1, part2};

  fn input() -> String {
    vec![
      "R 4",
      "U 4",
      "L 3",
      "D 1",
      "R 4",
      "D 1",
      "L 5",
      "R 2",
    ].join("\n")
  }

  fn input_2() -> String {
    vec![
      "R 5",
      "U 8",
      "L 8",
      "D 3",
      "R 17",
      "D 10",
      "L 25",
      "U 20",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let moves = generator(input().as_str());
    assert_eq!(8, moves.len());
    assert_eq!(&('R', 4), moves.first().unwrap());
    assert_eq!(&('R', 2), moves.last().unwrap());
  }

  #[test]
  fn test_part1() {
    let moves = generator(input().as_str());
    assert_eq!(13, part1(&moves));
  }

  #[test]
  fn test_part2() {
    let moves = generator(input().as_str());
    assert_eq!(1, part2(&moves));

    let moves = generator(input_2().as_str());
    assert_eq!(36, part2(&moves));
  }

  #[test]
  fn test_moves() {
    let prev = (5, 4);
    let mut curr = (0, 0);
    let mut visits: HashSet<(i32, i32)> = HashSet::new();
    move_knot_wo_record(&mut curr, &prev, &mut visits);
    assert_eq!(&(4, 4), &curr);
  }
}