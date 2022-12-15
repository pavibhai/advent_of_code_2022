use std::fmt::{Display, Formatter};
use crate::day14::Tile::{AIR, ROCK, SAND, SOURCE};

const SAND_SOURCE_X: i32 = 500;
const SAND_SOURCE_Y: i32 = 0;

pub fn generator(input: &str) -> Cave {
  let mut rock_lines: Vec<Line> = Vec::new();
  let mut min_pos = Position { x: 500, y: 0 };
  let mut max_pos = Position { x: 500, y: 0 };
  for line_positions in input.lines().map(|l| l.split(" -> ")) {
    let mut prev = None;
    for position in line_positions {
      let p = Position::from(position);
      max_pos.x = max_pos.x.max(p.x);
      max_pos.y = max_pos.y.max(p.y);
      min_pos.x = min_pos.x.min(p.x);
      min_pos.y = min_pos.y.min(p.y);
      if let Some(prev) = prev {
        rock_lines.push((prev, p))
      }
      prev = Some(p.clone());
    }
  }
  Cave::from(min_pos, max_pos, rock_lines)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Tile {
  SAND,
  AIR,
  ROCK,
  SOURCE,
}

impl Tile {
  fn to_char(&self) -> char {
    match self {
      SAND => 'o',
      AIR => '.',
      ROCK => '#',
      SOURCE => '+',
    }
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_char())
  }
}

#[derive(Clone)]
pub struct Cave {
  sand_source: Position,
  map: Vec<Vec<Tile>>,
  height: i32,
  width: i32,
}

impl Cave {
  fn from(min_pos: Position, max_pos: Position, lines: Vec<Line>) -> Cave {
    let height = max_pos.y - min_pos.y + 1;
    let width = max_pos.x - min_pos.x + 1;
    let mut map = vec![vec![AIR; width as usize]; height as usize];

    for (pos1, pos2) in lines {
      for x in pos1.x.min(pos2.x)..=pos1.x.max(pos2.x) {
        for y in pos1.y.min(pos2.y)..=pos1.y.max(pos2.y) {
          map[(y - min_pos.y) as usize][(x - min_pos.x) as usize] = ROCK;
        }
      }
    }
    let sand_source = Position { x: SAND_SOURCE_X - min_pos.x, y: SAND_SOURCE_Y - min_pos.y };
    map[sand_source.y as usize][sand_source.x as usize] = SOURCE;

    Cave {
      sand_source,
      map,
      height,
      width,
    }
  }

  fn fill_sand(&mut self) -> bool {
    let Position { x: mut c_x, y: mut c_y } = self.sand_source;

    loop {
      if c_x == 0 // Opening on left
        || c_x + 1 == self.width // Opening right
        || c_y + 1 == self.height { // Opening below
        return false;
      }
      // Move left
      if &self.map[(c_y + 1) as usize][c_x as usize] == &AIR {
        c_y += 1;
      } else if &self.map[(c_y + 1) as usize][(c_x - 1) as usize] == &AIR {
        c_x -= 1;
        c_y += 1;
      } else if &self.map[(c_y + 1) as usize][(c_x + 1) as usize] == &AIR {
        c_x += 1;
        c_y += 1;
      } else {
        self.map[c_y as usize][c_x as usize] = SAND;
        return true;
      }
    }
  }
}

impl Display for Cave {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();
    for line in self.map.iter() {
      for tile in line.iter() {
        output.push(tile.to_char());
      }
      output.push('\n');
    }
    write!(f, "{output}")
  }
}

type Line = (Position, Position);

#[derive(Debug, Clone, Copy)]
pub struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn from(input: &str) -> Position {
    let mut coordinates = input.split(',');
    Position {
      x: coordinates.next().unwrap().parse().unwrap(),
      y: coordinates.next().unwrap().parse().unwrap(),
    }
  }
}

pub fn part1(cave: &Cave) -> u32 {
  let mut cave = cave.clone();
  let mut times = 0;
  while cave.fill_sand() {
    times += 1;
  }
  times
}

pub fn part2(cave: &Cave) -> u32 {
  let height: i32 = cave.height + 1;
  let width: i32 = ((2 * height) + 1) as i32;
  let mut map = vec![vec![AIR; width as usize]; height as usize];
  let sand_source = Position { x: (width / 2) as i32, y: 0 };
  map[sand_source.y as usize][sand_source.x as usize] = SAND;
  let x_adjust = cave.sand_source.x - sand_source.x;
  let mut times = 1;
  for y in 1..height {
    for x in (sand_source.x - y)..=(sand_source.x + y) {
      let cave_pos_air = y >= cave.height
        || (x + x_adjust) < 0
        || (x + x_adjust) >= cave.width
        || &cave.map[y as usize][(x + x_adjust) as usize] == &AIR;
      if cave_pos_air
        && (&map[(y - 1) as usize][x as usize] == &SAND
          || (x - 1) >= 0 && &map[(y - 1) as usize][(x - 1) as usize] == &SAND
          || (x + 1) < width && &map[(y - 1) as usize][(x + 1) as usize] == &SAND) {
        map[y as usize][x as usize] = SAND;
        times += 1
      }
    }
  }
  times
}

#[cfg(test)]
mod tests {
  use crate::day14::{generator, part1, part2};

  fn input() -> String {
    vec![
      "498,4 -> 498,6 -> 496,6",
      "503,4 -> 502,4 -> 502,9 -> 494,9",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let cave = generator(input().as_str());
    let expected_map = "......+...\n\
                        ..........\n\
                        ..........\n\
                        ..........\n\
                        ....#...##\n\
                        ....#...#.\n\
                        ..###...#.\n\
                        ........#.\n\
                        ........#.\n\
                        #########.\n";
    assert_eq!(expected_map, cave.to_string());
  }

  #[test]
  fn test_fills() {
    let mut cave = generator(input().as_str());
    assert_eq!(true, cave.fill_sand());
    let expected = "......+...\n\
                    ..........\n\
                    ..........\n\
                    ..........\n\
                    ....#...##\n\
                    ....#...#.\n\
                    ..###...#.\n\
                    ........#.\n\
                    ......o.#.\n\
                    #########.\n";
    assert_eq!(expected, cave.to_string());
    assert_eq!(true, cave.fill_sand());
    let expected = "......+...\n\
                    ..........\n\
                    ..........\n\
                    ..........\n\
                    ....#...##\n\
                    ....#...#.\n\
                    ..###...#.\n\
                    ........#.\n\
                    .....oo.#.\n\
                    #########.\n";
    assert_eq!(expected, cave.to_string());
    assert_eq!(true, cave.fill_sand());
    assert_eq!(true, cave.fill_sand());
    assert_eq!(true, cave.fill_sand());
    let expected = "......+...\n\
                    ..........\n\
                    ..........\n\
                    ..........\n\
                    ....#...##\n\
                    ....#...#.\n\
                    ..###...#.\n\
                    ......o.#.\n\
                    ....oooo#.\n\
                    #########.\n";
    assert_eq!(expected, cave.to_string());

    for _ in 5..22 {
      cave.fill_sand();
    }
    let expected = "......+...\n\
                    ..........\n\
                    ......o...\n\
                    .....ooo..\n\
                    ....#ooo##\n\
                    ....#ooo#.\n\
                    ..###ooo#.\n\
                    ....oooo#.\n\
                    ...ooooo#.\n\
                    #########.\n";
    assert_eq!(expected, cave.to_string());

    assert_eq!(true, cave.fill_sand());
    assert_eq!(true, cave.fill_sand());
    let expected = "......+...\n\
                    ..........\n\
                    ......o...\n\
                    .....ooo..\n\
                    ....#ooo##\n\
                    ...o#ooo#.\n\
                    ..###ooo#.\n\
                    ....oooo#.\n\
                    .o.ooooo#.\n\
                    #########.\n";
    assert_eq!(expected, cave.to_string());
    assert_eq!(false, cave.fill_sand());
    assert_eq!(expected, cave.to_string());
  }

  #[test]
  fn test_part1() {
    let cave = generator(input().as_str());
    assert_eq!(24, part1(&cave));
  }

  #[test]
  fn test_part2() {
    let cave = generator(input().as_str());
    assert_eq!(93, part2(&cave));
  }
}