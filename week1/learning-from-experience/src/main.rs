const TITLE: &str = r#"经验学习：RL vs LLM"#;
const SUMMARY: &str =
    r#"通过寻宝游戏对比传统强化学习和语言模型上下文学习，理解样本效率与先验知识。"#;
const CONCEPTS: &[&str] = &[r#"强化学习"#, r#"上下文学习"#, r#"样本效率"#, r#"先验知识"#];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Room {
    Entrance,
    Storage,
    Hallway,
    Guard,
    Treasure,
}

#[derive(Debug, Default)]
struct Inventory {
    sword: bool,
    red_key: bool,
    crystal: bool,
    silver_sword: bool,
}

#[derive(Debug)]
struct Game {
    room: Room,
    inv: Inventory,
    guard_alive: bool,
    moves: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            room: Room::Entrance,
            inv: Inventory::default(),
            guard_alive: true,
            moves: 0,
        }
    }
}

impl Game {
    fn step(&mut self, action: &str) -> i32 {
        self.moves += 1;
        match (self.room, action) {
            (Room::Entrance, "take sword") => {
                self.inv.sword = true;
                2
            }
            (Room::Entrance, "east") => {
                self.room = Room::Storage;
                1
            }
            (Room::Storage, "take key") => {
                self.inv.red_key = true;
                5
            }
            (Room::Storage, "take crystal") => {
                self.inv.crystal = true;
                4
            }
            (Room::Storage, "craft") if self.inv.sword && self.inv.crystal => {
                self.inv.silver_sword = true;
                10
            }
            (Room::Storage, "west") => {
                self.room = Room::Entrance;
                0
            }
            (Room::Entrance, "north") if self.inv.red_key => {
                self.room = Room::Hallway;
                3
            }
            (Room::Hallway, "north") => {
                self.room = Room::Guard;
                1
            }
            (Room::Guard, "attack") if self.inv.silver_sword => {
                self.guard_alive = false;
                20
            }
            (Room::Guard, "east") if !self.guard_alive => {
                self.room = Room::Treasure;
                5
            }
            (Room::Treasure, "take treasure") => 100,
            _ => -2,
        }
    }
}

fn language_prior_plan() -> Vec<&'static str> {
    vec![
        "take sword",
        "east",
        "take key",
        "take crystal",
        "craft",
        "west",
        "north",
        "north",
        "attack",
        "east",
        "take treasure",
    ]
}

fn run_plan(actions: &[&str]) -> (i32, u32, bool) {
    let mut game = Game::default();
    let mut reward = 0;
    for action in actions {
        reward += game.step(action);
    }
    (reward, game.moves, game.room == Room::Treasure)
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!("核心概念：{}", CONCEPTS.join(" / "));
    let (reward, moves, victory) = run_plan(&language_prior_plan());
    println!(
        "
语言先验计划：moves={}, reward={}, victory={}",
        moves, reward, victory
    );
    println!("对照信号：随机试错需要大量 episode，而结构化经验可以直接压缩成行动计划。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prior_plan_reaches_treasure() {
        let (_, moves, victory) = run_plan(&language_prior_plan());
        assert!(victory);
        assert_eq!(moves, 11);
    }
}
