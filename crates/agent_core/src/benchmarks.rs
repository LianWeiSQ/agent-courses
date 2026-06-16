#[derive(Debug, Clone, PartialEq)]
pub struct AgentRating {
    pub name: String,
    pub rating: f64,
}

impl AgentRating {
    pub fn new(name: impl Into<String>, rating: f64) -> Self {
        Self {
            name: name.into(),
            rating,
        }
    }
}

pub fn expected_score(player: f64, opponent: f64) -> f64 {
    1.0 / (1.0 + 10_f64.powf((opponent - player) / 400.0))
}

pub fn update_elo(winner: &mut AgentRating, loser: &mut AgentRating, k_factor: f64) {
    let winner_expected = expected_score(winner.rating, loser.rating);
    let loser_expected = expected_score(loser.rating, winner.rating);
    winner.rating += k_factor * (1.0 - winner_expected);
    loser.rating += k_factor * (0.0 - loser_expected);
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkTask {
    pub name: &'static str,
    pub environment: &'static str,
    pub measures: &'static str,
}

pub fn benchmark_tasks() -> Vec<BenchmarkTask> {
    vec![
        BenchmarkTask {
            name: "Terminal-Bench style task",
            environment: "containerized terminal",
            measures: "end-to-end command execution and debugging",
        },
        BenchmarkTask {
            name: "SWE-Bench style task",
            environment: "real repository patch",
            measures: "issue resolution through tests",
        },
        BenchmarkTask {
            name: "GAIA style task",
            environment: "tool-augmented research",
            measures: "multi-step reasoning with verifiable answer",
        },
        BenchmarkTask {
            name: "Android World style task",
            environment: "mobile UI",
            measures: "screen navigation and app task completion",
        },
    ]
}

pub fn demo_benchmarks() -> String {
    let mut baseline = AgentRating::new("baseline-agent", 1_500.0);
    let mut improved = AgentRating::new("tool-rich-agent", 1_500.0);
    update_elo(&mut improved, &mut baseline, 32.0);

    let mut lines = vec![
        "Week 6 - Agent Evaluation".to_string(),
        "benchmark surfaces:".to_string(),
    ];

    for task in benchmark_tasks() {
        lines.push(format!(
            "  - {}: {} ({})",
            task.name, task.environment, task.measures
        ));
    }

    lines.push(String::new());
    lines.push("one ELO comparison after tool-rich-agent wins:".to_string());
    lines.push(format!("  baseline-agent: {:.1}", baseline.rating));
    lines.push(format!("  tool-rich-agent: {:.1}", improved.rating));
    lines.push(String::new());
    lines.push(
        "Takeaway: absolute task scores and relative ELO answer different evaluation questions."
            .to_string(),
    );
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elo_winner_gains_points() {
        let mut winner = AgentRating::new("winner", 1_500.0);
        let mut loser = AgentRating::new("loser", 1_500.0);
        update_elo(&mut winner, &mut loser, 32.0);
        assert!(winner.rating > 1_500.0);
        assert!(loser.rating < 1_500.0);
    }
}
