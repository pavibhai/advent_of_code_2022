use std::collections::HashMap;

pub fn generator(input: &str) -> HashMap<String, u64> {
  compute_sizes(parse(input))
}

fn parse(input: &str) -> Vec<(String, Option<u64>)> {
  let mut pwd: Vec<&str> = Vec::new();
  let mut out: Vec<(String, Option<u64>)> = Vec::new();
  for line in input.lines() {
    match line {
      "$ cd .." => {
        pwd.pop();
      }
      l if l.starts_with("$ cd /") => {
        pwd.push("");
        out.push(("/".to_string(), None))
      }
      l if l.starts_with("$ cd ") => pwd.push(l.split_at(5).1),
      l if l.starts_with("dir ") => {
        out.push((mk_name(&pwd, l.split_at(4).1), None));
      }
      "$ ls" => {}
      f => {
        let mut parts = f.split_whitespace();
        let size: u64 = parts.next().unwrap().parse().expect("Expecting a number");
        out.push((mk_name(&pwd, parts.next().unwrap()), Some(size)));
      }
    }
  }
  out
}

pub fn part1(sizes: &HashMap<String, u64>) -> u64 {
  sizes.iter().filter_map(|(_, size)| if size < &100000 {
    Some(size)
  } else {
    None
  })
    .sum()
}

pub fn part2(sizes: &HashMap<String, u64>) -> u64 {
  let free_space = 70000000 - sizes["/"];
  if free_space >= 30000000 {
    return 0;
  }
  let need_space = 30000000 - free_space;
  let mut found = u64::MAX;
  for (_, size) in sizes {
    if size > &need_space {
      found = found.min(*size);
    }
  }
  found
}

fn compute_sizes(input: Vec<(String, Option<u64>)>) -> HashMap<String, u64> {
  let mut sizes: HashMap<String, u64> = HashMap::new();
  for (file, size) in input {
    if size.is_none() {
      continue;
    }
    let mut curr = &file[0..];
    while !curr.is_empty() {
      match curr.rfind('/') {
        Some(p) => {
          sizes.entry(curr[0..=p].to_string())
            .and_modify(|v| *v += size.unwrap())
            .or_insert(size.unwrap());
          curr = &curr[0..p];
        }
        None => panic!("Unexpected")
      }
    }
  }
  sizes
}

fn mk_name(parent: &[&str], name: &str) -> String {
  format!("{}/{}", parent.join("/"), name)
}

#[cfg(test)]
mod tests {
  use crate::day7::{generator, parse, part1, part2};

  fn input() -> String {
    vec![
      "$ cd /",
      "$ ls",
      "dir a",
      "14848514 b.txt",
      "8504156 c.dat",
      "dir d",
      "$ cd a",
      "$ ls",
      "dir e",
      "29116 f",
      "2557 g",
      "62596 h.lst",
      "$ cd e",
      "$ ls",
      "584 i",
      "$ cd ..",
      "$ cd ..",
      "$ cd d",
      "$ ls",
      "4060174 j",
      "8033020 d.log",
      "5626152 d.ext",
      "7214296 k",
    ].join("\n")
  }

  #[test]
  fn test_parse() {
    let files = parse(input().as_str());
    assert_eq!(14, files.len());
    assert_eq!(true, files.contains(&("/".to_string(), None)));
    assert_eq!(true, files.contains(&("/a".to_string(), None)));
    assert_eq!(true, files.contains(&("/c.dat".to_string(), Some(8504156))));
    assert_eq!(true, files.contains(&("/a/e".to_string(), None)));
    assert_eq!(true, files.contains(&("/a/g".to_string(), Some(2557))));
    assert_eq!(true, files.contains(&("/a/e/i".to_string(), Some(584))));
    assert_eq!(true, files.contains(&("/d/d.log".to_string(), Some(8033020))));
  }

  #[test]
  fn test_generator() {
    let sizes = generator(input().as_str());
    assert_eq!(4, sizes.len());
    assert_eq!(584, sizes["/a/e/"]);
    assert_eq!(94853, sizes["/a/"]);
    assert_eq!(24933642, sizes["/d/"]);
    assert_eq!(48381165, sizes["/"]);
  }

  #[test]
  fn test_part1() {
    let input = generator(input().as_str());
    assert_eq!(95437, part1(&input));
  }

  #[test]
  fn test_part2() {
    let input = generator(input().as_str());
    assert_eq!(24933642, part2(&input));
  }
}