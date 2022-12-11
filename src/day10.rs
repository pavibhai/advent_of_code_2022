pub fn generator(input: &str) -> Vec<i32> {
  let mut x: i32 = 1;
  let mut result: Vec<i32> = Vec::new();
  result.push(x);
  for s in input.lines() {
    match s {
      "noop" => {
        result.push(x);
      }
      add => {
        let mut splits = add.split_whitespace();
        if splits.next().unwrap() != "addx" {
          panic!("Expecting addx")
        }
        result.push(x);
        x += splits.next().unwrap().parse::<i32>().expect("Expecting number");
        result.push(x);
      }
    }
  }
  result
}

pub fn part1(cycles: &Vec<i32>) -> String {
  let mut sig_strength: i32 = 0;
  for c in 0..6 {
    let cycle: usize = 20 + (c * 40);
    sig_strength += cycle as i32 * cycles[cycle - 1];
  }
  sig_strength.to_string()
}

pub fn part2(cycles: &Vec<i32>) -> String {
  let mut result = String::new();
  for line in 0..6 {
    result.push('\n');
    for pixel in 0..40 {
      let c = (line * 40) + pixel;
      let x = cycles[c];
      if x.abs_diff(pixel as i32) < 2 {
        result.push('#');
      } else {
        result.push('.');
      }
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use crate::day10::{generator, part1, part2};

  fn simple_input() -> String {
    vec![
      "noop",
      "addx 3",
      "addx -5",
    ].join("\n")
  }

  fn input() -> String {
    vec![
      "addx 15",
      "addx -11",
      "addx 6",
      "addx -3",
      "addx 5",
      "addx -1",
      "addx -8",
      "addx 13",
      "addx 4",
      "noop",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx -35",
      "addx 1",
      "addx 24",
      "addx -19",
      "addx 1",
      "addx 16",
      "addx -11",
      "noop",
      "noop",
      "addx 21",
      "addx -15",
      "noop",
      "noop",
      "addx -3",
      "addx 9",
      "addx 1",
      "addx -3",
      "addx 8",
      "addx 1",
      "addx 5",
      "noop",
      "noop",
      "noop",
      "noop",
      "noop",
      "addx -36",
      "noop",
      "addx 1",
      "addx 7",
      "noop",
      "noop",
      "noop",
      "addx 2",
      "addx 6",
      "noop",
      "noop",
      "noop",
      "noop",
      "noop",
      "addx 1",
      "noop",
      "noop",
      "addx 7",
      "addx 1",
      "noop",
      "addx -13",
      "addx 13",
      "addx 7",
      "noop",
      "addx 1",
      "addx -33",
      "noop",
      "noop",
      "noop",
      "addx 2",
      "noop",
      "noop",
      "noop",
      "addx 8",
      "noop",
      "addx -1",
      "addx 2",
      "addx 1",
      "noop",
      "addx 17",
      "addx -9",
      "addx 1",
      "addx 1",
      "addx -3",
      "addx 11",
      "noop",
      "noop",
      "addx 1",
      "noop",
      "addx 1",
      "noop",
      "noop",
      "addx -13",
      "addx -19",
      "addx 1",
      "addx 3",
      "addx 26",
      "addx -30",
      "addx 12",
      "addx -1",
      "addx 3",
      "addx 1",
      "noop",
      "noop",
      "noop",
      "addx -9",
      "addx 18",
      "addx 1",
      "addx 2",
      "noop",
      "noop",
      "addx 9",
      "noop",
      "noop",
      "noop",
      "addx -1",
      "addx 2",
      "addx -37",
      "addx 1",
      "addx 3",
      "noop",
      "addx 15",
      "addx -21",
      "addx 22",
      "addx -6",
      "addx 1",
      "noop",
      "addx 2",
      "addx 1",
      "noop",
      "addx -10",
      "noop",
      "noop",
      "addx 20",
      "addx 1",
      "addx 2",
      "addx 2",
      "addx -6",
      "addx -11",
      "noop",
      "noop",
      "noop",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let cycles = generator(simple_input().as_str());
    assert_eq!(6, cycles.len());
    assert_eq!(&1, cycles.first().unwrap());
    assert_eq!(&-1, cycles.last().unwrap());

    let cycles = generator(input().as_str());
    assert_eq!(241, cycles.len());
  }

  #[test]
  fn test_part1() {
    let cycles = generator(input().as_str());
    assert_eq!("13140", part1(&cycles));
  }

  #[test]
  fn test_part2() {
    let expected = vec![
      "",
      "##..##..##..##..##..##..##..##..##..##..",
      "###...###...###...###...###...###...###.",
      "####....####....####....####....####....",
      "#####.....#####.....#####.....#####.....",
      "######......######......######......####",
      "#######.......#######.......#######.....",
    ].join("\n");
    let cycles = generator(input().as_str());
    assert_eq!(expected, part2(&cycles));
  }
}