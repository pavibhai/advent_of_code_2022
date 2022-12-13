use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;
use crate::day13::PacketItem::{INT, LIST};

pub fn generator(input: &str) -> Vec<Packet> {
  input.lines().filter_map(|x| {
    if x.is_empty() {
      None
    } else {
      Some(make_packet(x))
    }
  }).collect()
}

pub fn part1(packets: &Vec<Packet>) -> usize {
  packets.chunks(2).enumerate().filter_map(|(idx, chunk)| {
    if chunk.first().unwrap().cmp(chunk.last().unwrap()).is_lt() {
      Some(idx + 1)
    } else {
      None
    }
  }).sum()
}

pub fn part2(packets: &Vec<Packet>) -> usize {
  let mut packets = packets.clone();
  let start_mark = make_packet("[[2]]");
  let end_mark = make_packet("[[6]]");
  packets.push(start_mark.clone());
  packets.push(end_mark.clone());
  packets.sort();

  let s_idx = match packets.binary_search(&start_mark) {
    Ok(i) => i + 1,
    _ => panic!("Did not find starting marker")
  };
  let e_idx = match packets.binary_search(&end_mark) {
    Ok(i) => i + 1,
    _ => panic!("Did not find starting marker")
  };
  s_idx * e_idx
}

type Packet = PacketItem;

fn make_packet(input: &str) -> Packet {
  match PacketItem::from(input) {
    l @ LIST(_) => l,
    r => panic!("Expecting a list, but received {:?}", r)
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PacketItem {
  INT(u32),
  LIST(Vec<PacketItem>),
}

impl Display for PacketItem {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();
    self.make_output(&mut output);
    write!(f, "{output}")
  }
}

impl PartialOrd<Self> for PacketItem {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for PacketItem {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (INT(s), INT(o)) => s.cmp(o),
      (LIST(s), LIST(o)) => {
        for i in 0..s.len().min(o.len()) {
          match s[i].cmp(&o[i]) {
            Ordering::Equal => continue,
            ne => return ne
          }
        }
        s.len().cmp(&o.len())
      }
      (s @ INT(_), LIST(o)) => {
        for i in 0..o.len().min(1 as usize) {
          match s.cmp(&o[i]) {
            Ordering::Equal => continue,
            ne => return ne
          }
        }
        1.cmp(&o.len())
      }
      (LIST(s), o @ INT(_)) => {
        for i in 0..s.len().min(1) {
          match s[i].cmp(&o) {
            Ordering::Equal => continue,
            ne => return ne
          }
        }
        s.len().cmp(&1)
      }
    }
  }
}

impl PacketItem {
  fn from(input: &str) -> PacketItem {
    let mut chars = input.chars();
    let mut itr = chars.borrow_mut().peekable();
    let result = PacketItem::make_list(&mut itr);
    match itr.next() {
      Some(c) => panic!("Unexpected character {c} after end of packet"),
      None => result
    }
  }

  fn make_output(&self, output: &mut String) {
    match self {
      INT(v) => {
        output.push_str(&v.to_string());
      }
      LIST(l) => {
        output.push('[');
        if !l.is_empty() {
          for item in l {
            item.make_output(output);
            output.push(',');
          }
          output.pop();
        }
        output.push(']');
      }
    }
  }

  fn make_list(input: &mut Peekable<&mut Chars>) -> PacketItem {
    match input.next() {
      Some('[') => {}
      x => panic!("Unexpected start: {:?} to a list", x),
    }

    let mut buffer: Vec<PacketItem> = Vec::new();
    loop {
      match input.peek() {
        Some(',') => {
          input.next();
        }
        Some(']') => {
          input.next();
          break;
        }
        Some('[') => {
          buffer.push(PacketItem::make_list(input));
        }
        Some(c) if c.is_digit(10) => buffer.push(PacketItem::make_int(input)),
        x => panic!("Unexpected {:?}", x),
      }
    }
    LIST(buffer)
  }

  fn make_int(input: &mut Peekable<&mut Chars>) -> PacketItem {
    let mut n: u32 = 0;
    loop {
      match input.peek() {
        Some(c) if c.is_digit(10) => {
          n *= 10;
          n += c.to_digit(10).unwrap();
          input.next();
        }
        _ => break,
      }
    }
    INT(n)
  }
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;
  use crate::day13::{PacketItem, make_packet, generator, part1, part2};
  use crate::day13::PacketItem::LIST;

  fn input() -> String {
    vec![
      "[1,1,3,1,1]",
      "[1,1,5,1,1]",
      "",
      "[[1],[2,3,4]]",
      "[[1],4]",
      "",
      "[9]",
      "[[8,7,6]]",
      "",
      "[[4,4],4,4]",
      "[[4,4],4,4,4]",
      "",
      "[7,7,7,7]",
      "[7,7,7]",
      "",
      "[]",
      "[3]",
      "",
      "[[[]]]",
      "[[]]",
      "",
      "[1,[2,[3,[4,[5,6,7]]]],8,9]",
      "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    assert_eq!(LIST(vec![PacketItem::INT(1), PacketItem::INT(2)]), make_packet("[1,2]"));
    assert_eq!("[[1],[2,3,4]]", make_packet("[[1],[2,3,4]]").to_string()
    );

    let packets = generator(input().as_str());
    assert_eq!(16, packets.len());
    assert_eq!("[1,1,3,1,1]", packets.first().unwrap().to_string());
    assert_eq!("[1,[2,[3,[4,[5,6,0]]]],8,9]", packets.last().unwrap().to_string());
  }

  #[test]
  fn test_part1() {
    assert_eq!(make_packet("[1,1,3,1,1]").cmp(&make_packet("[1,1,5,1,1]")), Ordering::Less);
    assert_eq!(make_packet("[[1],[2,3,4]]").cmp(&make_packet("[[1],4]")), Ordering::Less);
    assert_eq!(make_packet("[9]").cmp(&make_packet("[[8,7,6]]")), Ordering::Greater);

    let packets = generator(input().as_str());
    assert_eq!(13, part1(&packets));
  }

  #[test]
  fn test_part2() {
    let packets = generator(input().as_str());
    assert_eq!(140, part2(&packets));
  }
}
