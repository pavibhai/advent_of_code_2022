fn to_dec(c: char) -> i32 {
  match c {
    _ if c.is_digit(10) => c.to_digit(10).unwrap() as i32,
    '-' => -1,
    '=' => -2,
    _ => panic!("Unexpected input {c}"),
  }
}

fn to_char(n: i32) -> char {
  match n {
    0 => '0',
    1 => '1',
    2 => '2',
    -1 => '-',
    -2 => '=',
    _ => panic!("Unexpected value {n}"),
  }
}

pub fn generator(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|l| l.chars().collect()).collect()
}

fn handle_value(mut n: i32) -> (i32, i32) {
  let mut cf = n / 5;
  match n % 5 {
    r @ (0 | 1 | 2 | -1 | -2) => n = r,
    r @ (3 | -3) => {
      cf = cf + (1 * r.signum());
      n = -2 * r.signum();
    }
    r @ (4 | -4) => {
      cf = cf + (1 * r.signum());
      n = -1 * r.signum();
    }
    r => panic!("Unexpected {r}"),
  }
  (cf, n)
}

pub fn part1(numbers: &Vec<Vec<char>>) -> String {
  let mut result: Vec<char> = Vec::new();
  let max_length = numbers.iter().map(|n| n.len()).max().unwrap();
  let mut cf = 0;
  for x in 0..max_length {
    let mut n = cf;
    for number in numbers {
      if number.len() > x {
        n += to_dec(number[number.len() - 1 - x]);
      }
    }
    let h = handle_value(n);
    cf = h.0;
    result.push(to_char(h.1))
  }
  while cf != 0 {
    let h = handle_value(cf);
    result.push(to_char(h.1));
    cf = h.0;
  }
  result.iter().rev().collect()
}

pub fn part2(_numbers: &Vec<Vec<char>>) -> String {
  "yay".to_string()
}

#[cfg(test)]
mod tests {
  use crate::day25::{generator, handle_value, part1};

  fn input() -> String {
    vec![
      "1=-0-2",
      "12111",
      "2=0=",
      "21",
      "2=01",
      "111",
      "20012",
      "112",
      "1=-1=",
      "1-12",
      "12",
      "1=",
      "122",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let numbers = generator(input().as_str());
    assert_eq!(13, numbers.len());
    assert_eq!("1=-0-2", numbers[0].iter().collect::<String>());
    assert_eq!("122", numbers[12].iter().collect::<String>());
  }

  #[test]
  fn test_handle_value() {
    assert_eq!((0, 1), handle_value(1));
    assert_eq!((0, 2), handle_value(2));
    assert_eq!((1, -2), handle_value(3));
    assert_eq!((1, -1), handle_value(4));
    assert_eq!((1, 0), handle_value(5));
    assert_eq!((1, 1), handle_value(6));

    assert_eq!((0, -1), handle_value(-1));
    assert_eq!((0, -2), handle_value(-2));
    assert_eq!((-1, 2), handle_value(-3));
    assert_eq!((-1, 1), handle_value(-4));
    assert_eq!((-1, 0), handle_value(-5));
    assert_eq!((-1, -1), handle_value(-6));
  }

  #[test]
  fn test_part1() {
    let numbers = generator(input().as_str());
    assert_eq!("2=-1=0", part1(&numbers));
  }
}