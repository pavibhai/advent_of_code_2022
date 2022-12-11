use std::cell::RefCell;
use regex::Regex;
use crate::day11::Operation::{ADD, MUL, POW};

const MONKEY_FORMAT: &str = "(?s:Monkey (\\d+):\n\
                             \\s*Starting items: (\\d+(?:, \\d+)*)\n\
                             \\s*Operation: new = (old\\s.\\s\\S+)\n\
                             \\s*Test: divisible by (\\d+)\n\
                             \\s*If true: throw to monkey (\\d+)\n\
                             \\s*If false: throw to monkey (\\d+))";
const WORRY_DIVISOR: u64 = 3;

pub fn generator(input: &str) -> Vec<RefCell<Monkey>> {
  let rex = Regex::new(MONKEY_FORMAT).unwrap();
  let sections = input.split("\n\n");
  sections.map(|s| {
    let captures = rex.captures(s).unwrap();
    RefCell::from(Monkey::new(captures.get(1).unwrap().as_str().parse().expect("Expecting number"),
                              captures.get(2).unwrap().as_str().split(", ").map(|x| x.parse().unwrap()).collect(),
                              get_operation(captures.get(3).unwrap().as_str()),
                              captures.get(4).unwrap().as_str().parse().expect("Expecting number"),
                              captures.get(5).unwrap().as_str().parse().expect("Expecting number"),
                              captures.get(6).unwrap().as_str().parse().expect("Expecting number")))
  }).collect()
}

fn get_operation(input: &str) -> Operation {
  let mut parts = input.split_whitespace();
  if let Some("old") = parts.next() {
    match (parts.next().unwrap(), parts.next().unwrap()) {
      ("+", "old") => MUL(2),
      ("+", o) => ADD(o.parse().expect("Expecting operand as number")),
      ("*", "old") => POW(2),
      ("*", o) => MUL(o.parse().expect("Expecting operand as number")),
      _ => panic!()
    }
  } else {
    panic!("Unexpected value for operation {input}")
  }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Operation {
  MUL(u32),
  ADD(u32),
  POW(u32),
}

impl Operation {
  fn bin_op(&self, op1: u64) -> u64 {
    match self {
      MUL(op2) => op1 * *op2 as u64,
      ADD(op2) => op1 + *op2 as u64,
      POW(op2) => op1.pow(*op2),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Monkey {
  id: usize,
  items: Vec<u64>,
  mod_items: Vec<Vec<u64>>,
  op: Operation,
  divisor: u64,
  true_to: usize,
  false_to: usize,
  inspect_counter: u64,
}

impl Monkey {
  fn new(id: usize, items: Vec<u64>, op: Operation, divisor: u64, true_to: usize, false_to: usize)
         -> Monkey {
    if true_to == id || false_to == id {
      panic!("Targets are expected to be different than source");
    }
    Monkey {
      id,
      items,
      mod_items: Vec::new(),
      op,
      divisor,
      true_to,
      false_to,
      inspect_counter: 0,
    }
  }
  fn add_item(&mut self, worry: u64) {
    self.items.push(worry);
  }

  fn inspect(&mut self, worry_divisor: u64) -> Vec<(usize, u64)> {
    let result = self.items.iter().map(|w| {
      let new_w = self.op.bin_op(*w) / worry_divisor;
      match new_w % self.divisor {
        0 => (self.true_to, new_w),
        _ => (self.false_to, new_w),
      }
    }).collect();
    self.inspect_counter += self.items.len() as u64;
    self.items.clear();
    result
  }

  fn initialize_mods(&mut self, divisors: &Vec<u64>) {
    self.mod_items = self.items.iter().map(|i| {
      divisors.iter().map(|d| {
        (i % d) as u64
      }).collect()
    }).collect();
  }

  fn add_mod_item(&mut self, mod_item: Vec<u64>) {
    self.mod_items.push(mod_item);
  }

  fn inspect_mods(&mut self, divisors: &Vec<u64>) -> Vec<(usize, Vec<u64>)> {
    let mut result: Vec<(usize, Vec<u64>)> = Vec::new();
    for mut mod_item in self.mod_items.drain(..) {
      mod_item.iter_mut().enumerate().for_each(|(i, w)| {
        *w = self.op.bin_op(*w) % divisors[i];
      });
      match mod_item[self.id] {
        0 => result.push((self.true_to, mod_item)),
        _ => result.push((self.false_to, mod_item)),
      }
    }
    self.inspect_counter += result.len() as u64;
    result
  }
}

pub fn part1(monkeys: &Vec<RefCell<Monkey>>) -> u64 {
  let monkeys = monkeys.clone();
  for _ in 0..20 {
    for monkey in monkeys.iter() {
      for (to, worry) in monkey.borrow_mut().inspect(WORRY_DIVISOR) {
        monkeys[to].borrow_mut().add_item(worry);
      }
    }
  }
  let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.borrow().inspect_counter).collect();
  inspections.sort();
  inspections.last().unwrap() * inspections[inspections.len() - 2]
}

pub fn part2(monkeys: &Vec<RefCell<Monkey>>) -> u64 {
  let monkeys = monkeys.clone();
  let divisors: Vec<u64> = monkeys.iter().map(|m| m.borrow().divisor).collect();
  monkeys.iter().for_each(|m| m.borrow_mut().initialize_mods(&divisors));
  for _ in 0..10000 {
    for monkey in monkeys.iter() {
      for (to, worry) in monkey.borrow_mut().inspect_mods(&divisors) {
        monkeys[to].borrow_mut().add_mod_item(worry);
      }
    }
  }
  let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.borrow().inspect_counter).collect();
  inspections.sort();
  inspections.last().unwrap() * inspections[inspections.len() - 2]
}

#[cfg(test)]
mod tests {
  use regex::Regex;
  use crate::day11::{generator, MONKEY_FORMAT, part1, part2, WORRY_DIVISOR};
  use crate::day11::Operation::{MUL, POW};

  fn input() -> String {
    vec![
      "Monkey 0:",
      "Starting items: 79, 98",
      "Operation: new = old * 19",
      "Test: divisible by 23",
      "If true: throw to monkey 2",
      "If false: throw to monkey 3",
      "",
      "Monkey 1:",
      "Starting items: 54, 65, 75, 74",
      "Operation: new = old + 6",
      "Test: divisible by 19",
      "If true: throw to monkey 2",
      "If false: throw to monkey 0",
      "",
      "Monkey 2:",
      "Starting items: 79, 60, 97",
      "Operation: new = old * old",
      "Test: divisible by 13",
      "If true: throw to monkey 1",
      "If false: throw to monkey 3",
      "",
      "Monkey 3:",
      "Starting items: 74",
      "Operation: new = old + 3",
      "Test: divisible by 17",
      "If true: throw to monkey 0",
      "If false: throw to monkey 1",
      "",
    ].join("\n")
  }

  #[test]
  fn test_format() {
    let rex = Regex::new(MONKEY_FORMAT).unwrap();
    let i1 = "Monkey 0:\n
                Starting items: 79, 98\n
                Operation: new = old * 19\n
                Test: divisible by 23\n
                  If true: throw to monkey 2\n
                  If false: throw to monkey 3";
    assert_eq!(true, rex.is_match(i1));
    let captures = rex.captures(i1).unwrap();
    assert_eq!("0", captures.get(1).unwrap().as_str());
    assert_eq!("79, 98", captures.get(2).unwrap().as_str());
    assert_eq!("old * 19", captures.get(3).unwrap().as_str());
    assert_eq!("23", captures.get(4).unwrap().as_str());
    assert_eq!("2", captures.get(5).unwrap().as_str());
    assert_eq!("3", captures.get(6).unwrap().as_str());
  }

  #[test]
  fn test_generator() {
    let monkeys = generator(input().as_str());
    assert_eq!(4, monkeys.len());
    let monkey = monkeys.first().unwrap().borrow();
    assert_eq!(0, monkey.id);
    assert_eq!(vec![79, 98], monkey.items);
    assert_eq!(MUL(19), monkey.op);
    assert_eq!(23, monkey.divisor);
    assert_eq!(2, monkey.true_to);
    assert_eq!(3, monkey.false_to);

    let monkey = &monkeys[2].borrow();
    assert_eq!(2, monkey.id);
    assert_eq!(vec![79, 60, 97], monkey.items);
    assert_eq!(POW(2), monkey.op);
    assert_eq!(13, monkey.divisor);
    assert_eq!(1, monkey.true_to);
    assert_eq!(3, monkey.false_to);
  }

  #[test]
  fn test_inspect() {
    let monkeys = generator(input().as_str());
    let mut monkey = monkeys.first().unwrap().borrow_mut();
    assert_eq!(vec![(3, 500), (3, 620)], monkey.inspect(WORRY_DIVISOR));

    let mut monkey = monkeys.get(1).unwrap().borrow_mut();
    assert_eq!(vec![(0, 20), (0, 23), (0, 27), (0, 26)], monkey.inspect(WORRY_DIVISOR));
  }

  #[test]
  fn test_part1() {
    let monkeys = generator(input().as_str());
    assert_eq!(10605, part1(&monkeys));
  }

  #[test]
  fn test_mod_inspect() {
    let monkeys = generator(input().as_str());
    let divisors: Vec<u64> = monkeys.iter().map(|m| m.borrow().divisor).collect();
    assert_eq!(vec![23, 19, 13, 17], divisors);
    monkeys.iter().for_each(|m| m.borrow_mut().initialize_mods(&divisors));
    let mut monkey = monkeys.first().unwrap().borrow_mut();
    let mod_items = &monkey.mod_items;
    assert_eq!(2, mod_items.len());
    assert_eq!(&vec![10, 3, 1, 11], mod_items.first().unwrap());
    assert_eq!(&vec![6, 3, 7, 13], mod_items.last().unwrap());
    let result = monkey.inspect_mods(&divisors);
    assert_eq!(true, monkey.mod_items.is_empty());
    assert_eq!(2, result.len());
    assert_eq!(&(3, vec![(79 * 19) % 23, (79 * 19) % 19, (79 * 19) % 13, (79 * 19) % 17]),
               result.first().unwrap());
  }

  #[test]
  fn test_part2() {
    let monkeys = generator(input().as_str());
    assert_eq!(2713310158, part2(&monkeys));
  }

  #[test]
  fn test_mod_arithmetic() {
    let x = 20;
    let divisor = 7;
    let remainder = x % divisor;

    for c in -1..=1 {
      // addition
      assert_eq!((x + divisor + c) % divisor, (remainder + divisor + c) % divisor);
      assert_eq!((x + (divisor * 3) + c) % divisor, (remainder + (divisor * 3) + c) % divisor);
    }

    for c in 0..divisor * 3 {
      // multiplication
      assert_eq!((x * c) % divisor, (remainder * c) % divisor);
    }
    assert_eq!((x * x) % divisor, (remainder * x) % divisor);
    assert_eq!((x * x) % divisor, (remainder * remainder) % divisor);
  }
}