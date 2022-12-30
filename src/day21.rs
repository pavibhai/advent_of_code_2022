use std::collections::HashMap;
use crate::day21::Node::{Num, Op};
use crate::day21::Operator::{Add, Sub, Mul, Div};

const HUMAN_CODE: &str = "humn";

pub fn generator(input: &str) -> Node {
  let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
  for line in input.lines() {
    let mut splits = line.split(": ");
    map.insert(splits.next().unwrap(), splits.next().unwrap().split_whitespace().collect());
  }
  make_node("root", &map)
}

pub fn part1(root: &Node) -> i64 {
  root.evaluate()
}

pub fn part2(root: &Node) -> i64 {
  let mut path = Vec::new();
  root.path_to_human(&mut path);
  match (root, path.pop().unwrap()) {
    (Op(_, l, r), '<') => {
      l.compute_human_value(&mut path, r.evaluate())
    }
    (Op(_, l, r), '>') => {
      r.compute_human_value(&mut path, l.evaluate())
    }
    _ => panic!("Unexpected path"),
  }
}

fn make_node(node: &str, map: &HashMap<&str, Vec<&str>>) -> Node {
  let tokens = &map[node];
  match tokens[..] {
    [n] => Num(node == HUMAN_CODE, n.parse().unwrap()),
    [left, op, right] => {
      Op(Operator::from(op), Box::new(make_node(left, map)), Box::new(make_node(right, map)))
    }
    _ => panic!("Unexpected tokens {:?}", tokens),
  }
}

pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
}

impl Operator {
  fn from(input: &str) -> Operator {
    match input {
      "+" => Add,
      "-" => Sub,
      "*" => Mul,
      "/" => Div,
      _ => panic!("Unexpected operation {input}"),
    }
  }
  fn evaluate(&self, left: i64, right: i64) -> i64 {
    match self {
      Add => left + right,
      Sub => left - right,
      Mul => left * right,
      Div => left / right,
    }
  }

  fn evaluate_left(&self, expect: i64, right: i64) -> i64 {
    match self {
      Add => expect - right,
      Sub => expect + right,
      Mul => expect / right,
      Div => expect * right,
    }
  }

  fn evaluate_right(&self, expect: i64, left: i64) -> i64 {
    match self {
      Add => expect - left,
      Sub => left - expect,
      Mul => expect / left,
      Div => left / expect,
    }
  }
}

pub enum Node {
  Op(Operator, Box<Node>, Box<Node>),
  Num(bool, i64),
}

impl Node {
  fn evaluate(&self) -> i64 {
    match self {
      Op(o, o_l, o_r) => o.evaluate(o_l.evaluate(), o_r.evaluate()),
      Num(_, n) => *n
    }
  }

  fn path_to_human(&self, path: &mut Vec<char>) -> bool {
    match self {
      Num(x, _) => *x,
      Op(_, l, r) => {
        if l.path_to_human(path) {
          path.push('<');
          true
        } else if r.path_to_human(path) {
          path.push('>');
          true
        } else {
          false
        }
      }
    }
  }

  fn compute_human_value(&self, path: &mut Vec<char>, expect: i64) -> i64 {
    if path.is_empty() {
      return expect;
    }
    match (self, path.pop().unwrap()) {
      (Op(o, l, r), '<') => {
        l.compute_human_value(path, o.evaluate_left(expect, r.evaluate()))
      }
      (Op(o, l, r), '>') => {
        r.compute_human_value(path, o.evaluate_right(expect, l.evaluate()))
      }
      _ => panic!("Unexpected path"),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::day21::{generator, part1, part2};
  use crate::day21::Node::Op;

  fn input() -> String {
    vec![
      "root: pppw + sjmn",
      "dbpl: 5",
      "cczh: sllz + lgvd",
      "zczc: 2",
      "ptdq: humn - dvpt",
      "dvpt: 3",
      "lfqf: 4",
      "humn: 5",
      "ljgn: 2",
      "sjmn: drzm * dbpl",
      "sllz: 4",
      "pppw: cczh / lfqf",
      "lgvd: ljgn * ptdq",
      "drzm: hmdt - zczc",
      "hmdt: 32",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let root = generator(input().as_str());
    match root {
      Op(f, _, _) => assert_eq!(f.evaluate(3, 5), 3 + 5),
      _ => panic!()
    }
  }

  #[test]
  fn test_part1() {
    let root = generator(input().as_str());
    assert_eq!(152, part1(&root));
  }

  #[test]
  fn test_part2() {
    let mut result: Vec<char> = Vec::new();
    let root = generator(input().as_str());
    root.path_to_human(&mut result);
    assert_eq!(vec!['<', '>', '>', '<', '<'], result);
    assert_eq!(301, part2(&root));
  }
}