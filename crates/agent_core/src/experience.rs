use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Room {
    Entrance,
    Storage,
    Hallway,
    GuardRoom,
    TreasureRoom,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Room::Entrance => "entrance",
            Room::Storage => "storage",
            Room::Hallway => "hallway",
            Room::GuardRoom => "guard room",
            Room::TreasureRoom => "treasure room",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    RustySword,
    RedKey,
    MagicCrystal,
    SilverSword,
    Treasure,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Item::RustySword => "rusty sword",
            Item::RedKey => "red key",
            Item::MagicCrystal => "magic crystal",
            Item::SilverSword => "silver sword",
            Item::Treasure => "dragon's treasure",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub room: Room,
    pub inventory: Vec<Item>,
    pub strong_guard_alive: bool,
    pub treasure_taken: bool,
    pub moves: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            room: Room::Entrance,
            inventory: Vec::new(),
            strong_guard_alive: true,
            treasure_taken: false,
            moves: 0,
        }
    }
}

impl GameState {
    fn has(&self, item: Item) -> bool {
        self.inventory.contains(&item)
    }

    fn add(&mut self, item: Item) {
        if !self.has(item) {
            self.inventory.push(item);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepOutcome {
    pub feedback: String,
    pub reward: f64,
    pub done: bool,
}

#[derive(Debug, Clone)]
pub struct TreasureHunt {
    state: GameState,
    max_moves: u32,
}

impl Default for TreasureHunt {
    fn default() -> Self {
        Self::new()
    }
}

impl TreasureHunt {
    pub fn new() -> Self {
        Self {
            state: GameState::default(),
            max_moves: 40,
        }
    }

    pub fn reset(&mut self) {
        self.state = GameState::default();
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn state_description(&self) -> String {
        let inventory = if self.state.inventory.is_empty() {
            "nothing".to_string()
        } else {
            self.state
                .inventory
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        };

        format!(
            "You are in the {}. Inventory: {}. Guard alive: {}. Moves: {}.",
            self.state.room, inventory, self.state.strong_guard_alive, self.state.moves
        )
    }

    pub fn hidden_rules() -> &'static str {
        "Hidden mechanics: the red key opens the hallway, rusty sword + magic crystal crafts a silver sword, and only the silver sword defeats the strong guard."
    }

    pub fn available_actions(&self) -> Vec<String> {
        let mut actions = vec!["look around".to_string()];

        match self.state.room {
            Room::Entrance => {
                if !self.state.has(Item::RustySword) {
                    actions.push("take rusty sword".to_string());
                }
                actions.push("go east".to_string());
                actions.push("go north".to_string());
            }
            Room::Storage => {
                if !self.state.has(Item::RedKey) {
                    actions.push("take red key".to_string());
                }
                if !self.state.has(Item::MagicCrystal) {
                    actions.push("take magic crystal".to_string());
                }
                actions.push("try crafting".to_string());
                actions.push("go west".to_string());
            }
            Room::Hallway => {
                actions.push("go south".to_string());
                actions.push("go north".to_string());
            }
            Room::GuardRoom => {
                actions.push("attack guard".to_string());
                actions.push("go south".to_string());
                actions.push("go east".to_string());
            }
            Room::TreasureRoom => {
                if !self.state.treasure_taken {
                    actions.push("take treasure".to_string());
                }
                actions.push("go west".to_string());
            }
        }

        actions
    }

    pub fn execute(&mut self, action: &str) -> StepOutcome {
        self.state.moves += 1;
        let normalized = action.trim().to_ascii_lowercase();
        let mut outcome = match (self.state.room, normalized.as_str()) {
            (_, "look around") => StepOutcome {
                feedback: self.state_description(),
                reward: -0.1,
                done: false,
            },
            (Room::Entrance, "take rusty sword") if !self.state.has(Item::RustySword) => {
                self.state.add(Item::RustySword);
                StepOutcome {
                    feedback: "You pick up the rusty sword. It may be useful later.".to_string(),
                    reward: 2.0,
                    done: false,
                }
            }
            (Room::Entrance, "go east") => {
                self.state.room = Room::Storage;
                StepOutcome {
                    feedback: "You enter the storage room.".to_string(),
                    reward: 0.5,
                    done: false,
                }
            }
            (Room::Entrance, "go north") if self.state.has(Item::RedKey) => {
                self.state.room = Room::Hallway;
                StepOutcome {
                    feedback: "The red key unlocks the northern door.".to_string(),
                    reward: 3.0,
                    done: false,
                }
            }
            (Room::Entrance, "go north") => StepOutcome {
                feedback: "The northern door is locked by a red lock.".to_string(),
                reward: -2.0,
                done: false,
            },
            (Room::Storage, "take red key") if !self.state.has(Item::RedKey) => {
                self.state.add(Item::RedKey);
                StepOutcome {
                    feedback: "You take the red key.".to_string(),
                    reward: 5.0,
                    done: false,
                }
            }
            (Room::Storage, "take magic crystal") if !self.state.has(Item::MagicCrystal) => {
                self.state.add(Item::MagicCrystal);
                StepOutcome {
                    feedback: "You take the magic crystal.".to_string(),
                    reward: 4.0,
                    done: false,
                }
            }
            (Room::Storage, "try crafting")
                if self.state.has(Item::RustySword)
                    && self.state.has(Item::MagicCrystal)
                    && !self.state.has(Item::SilverSword) =>
            {
                self.state.add(Item::SilverSword);
                StepOutcome {
                    feedback: "The crystal fuses with the rusty sword. You craft a silver sword."
                        .to_string(),
                    reward: 10.0,
                    done: false,
                }
            }
            (Room::Storage, "try crafting") => StepOutcome {
                feedback: "Nothing useful happens. You probably need the right ingredients."
                    .to_string(),
                reward: -1.0,
                done: false,
            },
            (Room::Storage, "go west") => {
                self.state.room = Room::Entrance;
                StepOutcome {
                    feedback: "You return to the entrance.".to_string(),
                    reward: 0.2,
                    done: false,
                }
            }
            (Room::Hallway, "go south") => {
                self.state.room = Room::Entrance;
                StepOutcome {
                    feedback: "You walk back to the entrance.".to_string(),
                    reward: 0.0,
                    done: false,
                }
            }
            (Room::Hallway, "go north") => {
                self.state.room = Room::GuardRoom;
                StepOutcome {
                    feedback: "A strong guard blocks the treasure path.".to_string(),
                    reward: 1.0,
                    done: false,
                }
            }
            (Room::GuardRoom, "attack guard")
                if self.state.has(Item::SilverSword) && self.state.strong_guard_alive =>
            {
                self.state.strong_guard_alive = false;
                StepOutcome {
                    feedback: "The silver sword defeats the strong guard.".to_string(),
                    reward: 20.0,
                    done: false,
                }
            }
            (Room::GuardRoom, "attack guard") if self.state.strong_guard_alive => StepOutcome {
                feedback: "The guard shrugs off the attack. You need a better weapon.".to_string(),
                reward: -5.0,
                done: false,
            },
            (Room::GuardRoom, "attack guard") => StepOutcome {
                feedback: "The guard is already defeated.".to_string(),
                reward: -0.2,
                done: false,
            },
            (Room::GuardRoom, "go south") => {
                self.state.room = Room::Hallway;
                StepOutcome {
                    feedback: "You return to the hallway.".to_string(),
                    reward: 0.0,
                    done: false,
                }
            }
            (Room::GuardRoom, "go east") if !self.state.strong_guard_alive => {
                self.state.room = Room::TreasureRoom;
                StepOutcome {
                    feedback: "The path to the treasure room is clear.".to_string(),
                    reward: 5.0,
                    done: false,
                }
            }
            (Room::GuardRoom, "go east") => StepOutcome {
                feedback: "The guard blocks the way.".to_string(),
                reward: -3.0,
                done: false,
            },
            (Room::TreasureRoom, "take treasure") if !self.state.treasure_taken => {
                self.state.treasure_taken = true;
                self.state.add(Item::Treasure);
                StepOutcome {
                    feedback: "Victory! You claim the dragon's treasure.".to_string(),
                    reward: 100.0,
                    done: true,
                }
            }
            (Room::TreasureRoom, "go west") => {
                self.state.room = Room::GuardRoom;
                StepOutcome {
                    feedback: "You step back into the guard room.".to_string(),
                    reward: 0.0,
                    done: false,
                }
            }
            _ => StepOutcome {
                feedback: format!("'{action}' does not help here."),
                reward: -1.0,
                done: false,
            },
        };

        if self.state.moves >= self.max_moves && !outcome.done {
            outcome.feedback = "You ran out of time before finding the treasure.".to_string();
            outcome.reward -= 20.0;
            outcome.done = true;
        }

        outcome
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeResult {
    pub reward: f64,
    pub steps: u32,
    pub victory: bool,
}

#[derive(Debug, Clone, Copy, Eq)]
struct StateKey {
    room: Room,
    rusty_sword: bool,
    red_key: bool,
    magic_crystal: bool,
    silver_sword: bool,
    guard_alive: bool,
    treasure_taken: bool,
}

impl PartialEq for StateKey {
    fn eq(&self, other: &Self) -> bool {
        self.room == other.room
            && self.rusty_sword == other.rusty_sword
            && self.red_key == other.red_key
            && self.magic_crystal == other.magic_crystal
            && self.silver_sword == other.silver_sword
            && self.guard_alive == other.guard_alive
            && self.treasure_taken == other.treasure_taken
    }
}

impl Hash for StateKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.room.hash(state);
        self.rusty_sword.hash(state);
        self.red_key.hash(state);
        self.magic_crystal.hash(state);
        self.silver_sword.hash(state);
        self.guard_alive.hash(state);
        self.treasure_taken.hash(state);
    }
}

impl From<&GameState> for StateKey {
    fn from(state: &GameState) -> Self {
        Self {
            room: state.room,
            rusty_sword: state.has(Item::RustySword),
            red_key: state.has(Item::RedKey),
            magic_crystal: state.has(Item::MagicCrystal),
            silver_sword: state.has(Item::SilverSword),
            guard_alive: state.strong_guard_alive,
            treasure_taken: state.treasure_taken,
        }
    }
}

#[derive(Debug, Clone)]
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        let value = self.next_u64() >> 11;
        (value as f64) / ((1_u64 << 53) as f64)
    }

    fn choose_index(&mut self, len: usize) -> usize {
        if len <= 1 {
            0
        } else {
            (self.next_u64() as usize) % len
        }
    }
}

#[derive(Debug, Clone)]
pub struct QLearningAgent {
    q_values: HashMap<(StateKey, String), f64>,
    learning_rate: f64,
    discount: f64,
    epsilon: f64,
    epsilon_decay: f64,
    epsilon_min: f64,
    rng: Lcg,
}

impl Default for QLearningAgent {
    fn default() -> Self {
        Self::new(7)
    }
}

impl QLearningAgent {
    pub fn new(seed: u64) -> Self {
        Self {
            q_values: HashMap::new(),
            learning_rate: 0.25,
            discount: 0.97,
            epsilon: 1.0,
            epsilon_decay: 0.9985,
            epsilon_min: 0.05,
            rng: Lcg::new(seed),
        }
    }

    pub fn q_table_size(&self) -> usize {
        self.q_values.len()
    }

    fn q_value(&self, state: StateKey, action: &str) -> f64 {
        *self
            .q_values
            .get(&(state, action.to_string()))
            .unwrap_or(&0.0)
    }

    fn choose_action(&mut self, game: &TreasureHunt, training: bool) -> String {
        let actions = game.available_actions();
        if training && self.rng.next_f64() < self.epsilon {
            return actions[self.rng.choose_index(actions.len())].clone();
        }

        let state = StateKey::from(game.state());
        actions
            .into_iter()
            .max_by(|a, b| {
                self.q_value(state, a)
                    .partial_cmp(&self.q_value(state, b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or_else(|| "look around".to_string())
    }

    fn update(
        &mut self,
        state: StateKey,
        action: String,
        reward: f64,
        next_state: StateKey,
        next_actions: &[String],
        done: bool,
    ) {
        let current = self.q_value(state, &action);
        let future = if done {
            0.0
        } else {
            next_actions
                .iter()
                .map(|action| self.q_value(next_state, action))
                .fold(f64::NEG_INFINITY, f64::max)
                .max(0.0)
        };
        let target = reward + self.discount * future;
        let updated = current + self.learning_rate * (target - current);
        self.q_values.insert((state, action), updated);
    }

    pub fn train(&mut self, episodes: usize) -> TrainingSummary {
        let mut victories = 0;
        let mut rewards = Vec::with_capacity(episodes);

        for _ in 0..episodes {
            let mut game = TreasureHunt::new();
            let mut total_reward = 0.0;

            loop {
                let state = StateKey::from(game.state());
                let action = self.choose_action(&game, true);
                let outcome = game.execute(&action);
                let next_state = StateKey::from(game.state());
                let next_actions = game.available_actions();
                total_reward += outcome.reward;

                self.update(
                    state,
                    action,
                    outcome.reward,
                    next_state,
                    &next_actions,
                    outcome.done,
                );

                if outcome.done {
                    if game.state().treasure_taken {
                        victories += 1;
                    }
                    rewards.push(total_reward);
                    break;
                }
            }

            self.epsilon = (self.epsilon * self.epsilon_decay).max(self.epsilon_min);
        }

        TrainingSummary {
            episodes,
            victories,
            victory_rate: victories as f64 / episodes as f64,
            q_table_size: self.q_table_size(),
            average_reward: rewards.iter().sum::<f64>() / rewards.len().max(1) as f64,
        }
    }

    pub fn evaluate(&mut self, episodes: usize) -> TrainingSummary {
        let original_epsilon = self.epsilon;
        self.epsilon = 0.0;
        let mut victories = 0;
        let mut rewards = Vec::with_capacity(episodes);

        for _ in 0..episodes {
            let result = self.play_episode(false);
            if result.victory {
                victories += 1;
            }
            rewards.push(result.reward);
        }

        self.epsilon = original_epsilon;
        TrainingSummary {
            episodes,
            victories,
            victory_rate: victories as f64 / episodes as f64,
            q_table_size: self.q_table_size(),
            average_reward: rewards.iter().sum::<f64>() / rewards.len().max(1) as f64,
        }
    }

    pub fn play_episode(&mut self, training: bool) -> EpisodeResult {
        let mut game = TreasureHunt::new();
        let mut reward = 0.0;
        loop {
            let action = self.choose_action(&game, training);
            let outcome = game.execute(&action);
            reward += outcome.reward;
            if outcome.done {
                return EpisodeResult {
                    reward,
                    steps: game.state().moves,
                    victory: game.state().treasure_taken,
                };
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingSummary {
    pub episodes: usize,
    pub victories: usize,
    pub victory_rate: f64,
    pub q_table_size: usize,
    pub average_reward: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReasonedStep {
    pub action: &'static str,
    pub reason: &'static str,
}

pub fn language_prior_plan() -> Vec<ReasonedStep> {
    vec![
        ReasonedStep {
            action: "take rusty sword",
            reason: "Collect any tool-like object before exploring locked or guarded areas.",
        },
        ReasonedStep {
            action: "go east",
            reason: "Search adjacent rooms for keys and crafting ingredients.",
        },
        ReasonedStep {
            action: "take red key",
            reason: "A red key plausibly opens a red lock.",
        },
        ReasonedStep {
            action: "take magic crystal",
            reason: "Magic items often combine with weak equipment.",
        },
        ReasonedStep {
            action: "try crafting",
            reason: "The rusty sword and crystal are complementary ingredients.",
        },
        ReasonedStep {
            action: "go west",
            reason: "Return to the locked northern door.",
        },
        ReasonedStep {
            action: "go north",
            reason: "Use the red key to enter the hallway.",
        },
        ReasonedStep {
            action: "go north",
            reason: "Advance toward the guard after preparing a better weapon.",
        },
        ReasonedStep {
            action: "attack guard",
            reason: "The silver sword is the likely counter to a strong guard.",
        },
        ReasonedStep {
            action: "go east",
            reason: "The defeated guard no longer blocks the treasure room.",
        },
        ReasonedStep {
            action: "take treasure",
            reason: "Claim the terminal objective.",
        },
    ]
}

pub fn run_language_prior_demo() -> EpisodeResult {
    let mut game = TreasureHunt::new();
    let mut reward = 0.0;

    for step in language_prior_plan() {
        let outcome = game.execute(step.action);
        reward += outcome.reward;
        if outcome.done {
            break;
        }
    }

    EpisodeResult {
        reward,
        steps: game.state().moves,
        victory: game.state().treasure_taken,
    }
}

pub fn demo_learning_from_experience() -> String {
    let mut rl = QLearningAgent::new(42);
    let early = rl.train(100);
    let early_eval = rl.evaluate(50);
    let later = rl.train(4_900);
    let later_eval = rl.evaluate(50);
    let prior = run_language_prior_demo();

    format!(
        "\
Week 1 - Learning from Experience
Hidden rule summary: {}

Q-learning after 100 episodes:
  training victory rate: {:.1}%
  eval victory rate: {:.1}%
  q-table entries: {}

Q-learning after 5,000 total episodes:
  added training victory rate: {:.1}%
  eval victory rate: {:.1}%
  q-table entries: {}

Language-prior plan:
  victory: {}
  steps: {}
  reward: {:.1}

Takeaway: tabular RL eventually memorizes the maze, while an LLM-style prior can form a useful plan from sparse evidence.",
        TreasureHunt::hidden_rules(),
        early.victory_rate * 100.0,
        early_eval.victory_rate * 100.0,
        early.q_table_size,
        later.victory_rate * 100.0,
        later_eval.victory_rate * 100.0,
        later.q_table_size,
        prior.victory,
        prior.steps,
        prior.reward,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimal_language_prior_wins() {
        let result = run_language_prior_demo();
        assert!(result.victory);
        assert!(result.steps <= 12);
    }

    #[test]
    fn crafting_requires_ingredients() {
        let mut game = TreasureHunt::new();
        game.execute("go east");
        let failed = game.execute("try crafting");
        assert!(failed.reward < 0.0);

        let mut game = TreasureHunt::new();
        game.execute("take rusty sword");
        game.execute("go east");
        game.execute("take magic crystal");
        let crafted = game.execute("try crafting");
        assert!(crafted.feedback.contains("silver sword"));
    }

    #[test]
    fn q_learning_improves_with_training() {
        let mut agent = QLearningAgent::new(123);
        let early = agent.train(200);
        let later = agent.train(2_000);
        assert!(later.victory_rate >= early.victory_rate);
        assert!(agent.q_table_size() > 0);
    }
}
