use std::collections::VecDeque;
use crate::day19::Robot::{ClayBot, GeodeBot, ObsidianBot, OreBot};

#[derive(Eq, PartialEq, Debug)]
enum Robot {
  OreBot,
  ClayBot,
  ObsidianBot,
  GeodeBot,
}

#[derive(Eq, PartialEq, Debug)]
struct BotCost {
  robot: Robot,
  ore: u32,
  clay: u32,
  obsidian: u32,
}

pub fn generator(input: &str) -> Vec<Blueprint> {
  input.lines().map(|l| Blueprint::from(l))
       .collect()
}

pub fn part1(blue_prints: &Vec<Blueprint>) -> u32 {
  blue_prints.iter().fold(0, |a, bp| {
    a + bp.id * bp.simulate(24)
  })
}

pub fn part2(blue_prints: &Vec<Blueprint>) -> u32 {
  blue_prints[0..3.min(blue_prints.len())].iter().map(|bp| bp.simulate(32)).product()
}

#[derive(Eq, PartialEq, Debug)]
pub struct Blueprint {
  id: u32,
  bot_costs: Vec<BotCost>,
  max_ore_cost: u32,
  max_clay_cost: u32,
  max_obsidian_cost: u32,
}

impl Blueprint {
  fn new(id: u32, bot_costs: Vec<BotCost>) -> Blueprint {
    let max_ore_cost = bot_costs.iter().map(|bc| bc.ore).max().unwrap();
    let max_clay_cost = bot_costs.iter().map(|bc| bc.clay).max().unwrap();
    let max_obsidian_cost = bot_costs.iter().map(|bc| bc.obsidian).max().unwrap();
    Blueprint {
      id,
      bot_costs,
      max_ore_cost,
      max_clay_cost,
      max_obsidian_cost,
    }
  }
  fn from(input: &str) -> Blueprint {
    let splits: Vec<&str> = input.split_whitespace().collect();
    let id = splits[1].trim_end_matches(':').parse().unwrap();
    let mut bot_costs = Vec::new();
    bot_costs.push(BotCost {
      robot: OreBot,
      ore: splits[6].parse().unwrap(),
      clay: 0,
      obsidian: 0,
    });
    bot_costs.push(BotCost {
      robot: ClayBot,
      ore: splits[12].parse().unwrap(),
      clay: 0,
      obsidian: 0,
    });
    bot_costs.push(BotCost {
      robot: ObsidianBot,
      ore: splits[18].parse().unwrap(),
      clay: splits[21].parse().unwrap(),
      obsidian: 0,
    });
    bot_costs.push(BotCost {
      robot: GeodeBot,
      ore: splits[27].parse().unwrap(),
      clay: 0,
      obsidian: splits[30].parse().unwrap(),
    });
    bot_costs.reverse();
    Blueprint::new(id, bot_costs)
  }

  fn simulate(&self, until_time: u32) -> u32 {
    let mut max_geodes: u32 = 0;
    let mut stack: VecDeque<Simulation> = VecDeque::new();
    stack.push_back(Simulation::new());
    while !stack.is_empty() {
      let curr = stack.pop_front().unwrap();
      max_geodes = max_geodes.max(curr.geode);
      if curr.time == until_time {
        continue;
      }
      // Create the possible choices
      for cost in &self.bot_costs {
        match cost.robot {
          OreBot if curr.ore_bots >= self.max_ore_cost
            || curr.ore / (until_time - curr.time) >= self.max_ore_cost - curr.ore_bots => continue,
          ClayBot if curr.clay_bots >= self.max_clay_cost
            || curr.clay / (until_time - curr.time) >= self.max_clay_cost - curr.clay_bots => continue,
          ObsidianBot if curr.obsidian_bots >= self.max_obsidian_cost
            || curr.obsidian / (until_time - curr.time) >= self.max_obsidian_cost - curr.obsidian_bots => continue,
          _ => {}
        }
        match self.make_robot(&curr, cost, &until_time) {
          Some(s) => {
            if cost.robot == GeodeBot && curr.time + 1 == s.time {
              stack.push_back(s);
              break;
            } else {
              stack.push_back(s);
            }
          }
          _ => {}
        }
      }
    }
    max_geodes
  }

  fn make_robot(&self, sim: &Simulation, cost: &BotCost, until_time: &u32) -> Option<Simulation> {
    if (cost.obsidian > 0 && sim.obsidian_bots < 1)
      || (cost.clay > 0 && sim.clay_bots < 1)
      || (cost.ore > 0 && sim.ore_bots < 1) {
      return None;
    }

    let mut sim = sim.clone();
    while &sim.time < until_time {
      if sim.obsidian < cost.obsidian
        || sim.clay < cost.clay
        || sim.ore < cost.ore {
        sim.gather();
      } else {
        // Create the bot
        sim.ore -= cost.ore;
        sim.clay -= cost.clay;
        sim.obsidian -= cost.obsidian;
        sim.gather();
        sim.add_bot(&cost.robot);
        break;
      }
    }
    Some(sim)
  }
}

#[derive(Clone, Debug)]
struct Simulation {
  time: u32,
  ore_bots: u32,
  ore: u32,
  clay_bots: u32,
  clay: u32,
  obsidian_bots: u32,
  obsidian: u32,
  geode_bots: u32,
  geode: u32,
}

impl Simulation {
  fn new() -> Simulation {
    Simulation {
      time: 0,
      ore_bots: 1,
      ore: 0,
      clay_bots: 0,
      clay: 0,
      obsidian_bots: 0,
      obsidian: 0,
      geode_bots: 0,
      geode: 0,
    }
  }

  fn gather(&mut self) {
    self.time += 1;
    self.ore += self.ore_bots;
    self.clay += self.clay_bots;
    self.obsidian += self.obsidian_bots;
    self.geode += self.geode_bots;
  }

  fn add_bot(&mut self, bot: &Robot) {
    match bot {
      OreBot => self.ore_bots += 1,
      ClayBot => self.clay_bots += 1,
      ObsidianBot => self.obsidian_bots += 1,
      GeodeBot => self.geode_bots += 1,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::day19::{Blueprint, BotCost, generator, part1, part2};
  use crate::day19::Robot::{ClayBot, GeodeBot, ObsidianBot, OreBot};

  fn input() -> String {
    vec![
      "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
      "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let bps = generator(input().as_str());
    assert_eq!(2, bps.len());
    let exp_bp = Blueprint::new(1,
                                vec![
                                  BotCost { robot: GeodeBot, ore: 2, clay: 0, obsidian: 7 },
                                  BotCost { robot: ObsidianBot, ore: 3, clay: 14, obsidian: 0 },
                                  BotCost { robot: ClayBot, ore: 2, clay: 0, obsidian: 0 },
                                  BotCost { robot: OreBot, ore: 4, clay: 0, obsidian: 0 },
                                ]);
    assert_eq!(exp_bp, bps[0]);
    let exp_bp = Blueprint::new(2,
                                vec![
                                  BotCost { robot: GeodeBot, ore: 3, clay: 0, obsidian: 12 },
                                  BotCost { robot: ObsidianBot, ore: 3, clay: 8, obsidian: 0 },
                                  BotCost { robot: ClayBot, ore: 3, clay: 0, obsidian: 0 },
                                  BotCost { robot: OreBot, ore: 2, clay: 0, obsidian: 0 },
                                ]);
    assert_eq!(exp_bp, bps[1]);
  }

  #[test]
  fn test_simulation() {
    let bp = Blueprint::new(1,
                            vec![
                              BotCost { robot: GeodeBot, ore: 2, clay: 0, obsidian: 7 },
                              BotCost { robot: ObsidianBot, ore: 3, clay: 14, obsidian: 0 },
                              BotCost { robot: ClayBot, ore: 2, clay: 0, obsidian: 0 },
                              BotCost { robot: OreBot, ore: 4, clay: 0, obsidian: 0 },
                            ]);
    assert_eq!(9, bp.simulate(24));
    let bp = Blueprint::new(2,
                            vec![
                              BotCost { robot: GeodeBot, ore: 3, clay: 0, obsidian: 12 },
                              BotCost { robot: ObsidianBot, ore: 3, clay: 8, obsidian: 0 },
                              BotCost { robot: ClayBot, ore: 3, clay: 0, obsidian: 0 },
                              BotCost { robot: OreBot, ore: 2, clay: 0, obsidian: 0 },
                            ]);
    assert_eq!(12, bp.simulate(24));
  }

  #[test]
  fn test_part1() {
    let bps = generator(input().as_str());
    assert_eq!(33, part1(&bps));
  }

  #[test]
  fn test_part2() {
    let bps = generator(input().as_str());
    assert_eq!(62 * 56, part2(&bps));
  }
}