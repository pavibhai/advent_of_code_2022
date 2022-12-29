use std::cell::{RefCell};

const DECRYPTION_KEY: i64 = 811589153;

pub fn generator(input: &str) -> Vec<i64> {
  input.lines()
       .map(|l| l.parse().unwrap())
       .collect()
}

pub fn part1(numbers: &Vec<i64>) -> i64 {
  let mut numbers = Numbers::from(numbers.clone());
  numbers.mix();
  let numbers = numbers.to_vec();
  let zero_pos = numbers.iter().position(|n| n == &0).unwrap();
  let mut result = 0;
  for i in [1000, 2000, 3000] {
    result += numbers[(zero_pos + i) % numbers.len()];
  }
  result
}

pub fn part2(numbers: &Vec<i64>) -> i64 {
  let mut numbers = Numbers::from(numbers.iter().map(|n| n * DECRYPTION_KEY).collect());
  for _ in 0..10 {
    numbers.mix();
  }
  let numbers = numbers.to_vec();
  let zero_pos = numbers.iter().position(|n| n == &0).unwrap();
  let mut result = 0;
  for i in [1000, 2000, 3000] {
    result += numbers[(zero_pos + i) % numbers.len()];
  }
  result
}

struct Numbers {
  numbers: Vec<i64>,
  links: Vec<RefCell<(usize, usize)>>,
}

impl Numbers {
  fn from(numbers: Vec<i64>) -> Numbers {
    let mut links = Vec::new();
    for i in 0..numbers.len() {
      links.push(RefCell::from(((i as i64 - 1).rem_euclid(numbers.len() as i64) as usize,
                                (i + 1) % numbers.len())));
    }
    Numbers {
      numbers,
      links,
    }
  }
  fn mix(&mut self) {
    for i in 0..self.numbers.len() {
      self.move_steps(i);
    }
  }

  fn move_steps(&mut self, i: usize) {
    let mut steps = self.numbers[i] % (self.numbers.len() - 1) as i64;
    let mut this = self.links[i].borrow_mut();
    match steps {
      _ if steps < 0 => {
        // Move left
        let mut left = self.links[this.0].borrow_mut();
        left.1 = this.1;
        self.links[this.1].borrow_mut().0 = this.0;
        while steps < 0 {
          left = self.links[left.0].borrow_mut();
          steps += 1;
        }
        let mut right = self.links[left.1].borrow_mut();
        this.0 = right.0;
        this.1 = left.1;
        left.1 = i;
        right.0 = i;
      }
      _ if steps > 0 => {
        // Move right
        let mut right = self.links[this.1].borrow_mut();
        right.0 = this.0;
        self.links[this.0].borrow_mut().1 = this.1;
        while steps > 0 {
          right = self.links[right.1].borrow_mut();
          steps -= 1;
        }
        let mut left = self.links[right.0].borrow_mut();
        this.0 = right.0;
        this.1 = left.1;
        left.1 = i;
        right.0 = i;
      }
      _ => {}
    }
  }

  fn to_vec(&self) -> Vec<i64> {
    let mut result = Vec::new();
    result.push(self.numbers[0]);
    let mut next_idx = self.links[0].borrow().1;
    while next_idx != 0 {
      result.push(self.numbers[next_idx]);
      next_idx = self.links[next_idx].borrow().1;
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use crate::day20::{generator, Numbers, part1, part2};

  fn input() -> String {
    vec![
      "1",
      "2",
      "-3",
      "3",
      "-2",
      "0",
      "4",
    ].join("\n")
  }

  fn to_pairs(input: Vec<i64>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = input.windows(2)
                                           .map(|w| (*w.first().unwrap(), *w.last().unwrap()))
                                           .collect();
    result.push((*input.last().unwrap(), *input.first().unwrap()));
    result.sort();
    result
  }

  #[test]
  fn test_generator() {
    let numbers = generator(input().as_str());
    assert_eq!(7, numbers.len());
    assert_eq!(&1, numbers.first().unwrap());
    assert_eq!(&4, numbers.last().unwrap());

    let mut numbers = Numbers::from(numbers);
    assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], numbers.to_vec());
    numbers.move_steps(0);
    assert_eq!(to_pairs(vec![2, 1, -3, 3, -2, 0, 4]), to_pairs(numbers.to_vec()));
    numbers.move_steps(1);
    assert_eq!(to_pairs(vec![1, -3, 2, 3, -2, 0, 4]), to_pairs(numbers.to_vec()));
    numbers.move_steps(2);
    assert_eq!(to_pairs(vec![1, 2, 3, -2, -3, 0, 4]), to_pairs(numbers.to_vec()));
    numbers.move_steps(3);
    assert_eq!(to_pairs(vec![1, 2, -2, -3, 0, 3, 4]), to_pairs(numbers.to_vec()));
    numbers.move_steps(4);
    assert_eq!(to_pairs(vec![1, 2, -3, 0, 3, 4, -2]), to_pairs(numbers.to_vec()));
    numbers.move_steps(5);
    assert_eq!(to_pairs(vec![1, 2, -3, 0, 3, 4, -2]), to_pairs(numbers.to_vec()));
    numbers.move_steps(6);
    assert_eq!(to_pairs(vec![1, 2, -3, 4, 0, 3, -2]), to_pairs(numbers.to_vec()));
  }

  #[test]
  fn test_part1() {
    let numbers = generator(input().as_str());
    assert_eq!(3, part1(&numbers));
  }

  #[test]
  fn test_part2() {
    let numbers = generator(input().as_str());
    assert_eq!(1623178306, part2(&numbers));
  }
}