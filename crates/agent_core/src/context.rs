#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMode {
    Full,
    NoHistory,
    NoReasoning,
    NoToolCalls,
    NoToolResults,
}

impl ContextMode {
    pub fn all() -> &'static [ContextMode] {
        &[
            ContextMode::Full,
            ContextMode::NoHistory,
            ContextMode::NoReasoning,
            ContextMode::NoToolCalls,
            ContextMode::NoToolResults,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            ContextMode::Full => "full",
            ContextMode::NoHistory => "no_history",
            ContextMode::NoReasoning => "no_reasoning",
            ContextMode::NoToolCalls => "no_tool_calls",
            ContextMode::NoToolResults => "no_tool_results",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextReport {
    pub mode: ContextMode,
    pub success_score: f32,
    pub iterations: u8,
    pub tool_calls: u8,
    pub failure_mode: &'static str,
}

pub fn evaluate_context_mode(mode: ContextMode) -> ContextReport {
    match mode {
        ContextMode::Full => ContextReport {
            mode,
            success_score: 0.95,
            iterations: 5,
            tool_calls: 4,
            failure_mode:
                "baseline: history, reasoning, tools, and observations reinforce one another",
        },
        ContextMode::NoHistory => ContextReport {
            mode,
            success_score: 0.72,
            iterations: 8,
            tool_calls: 7,
            failure_mode: "repeats work because prior tool calls are not visible",
        },
        ContextMode::NoReasoning => ContextReport {
            mode,
            success_score: 0.48,
            iterations: 10,
            tool_calls: 9,
            failure_mode: "lacks a plan, so it overuses tools and misses dependencies",
        },
        ContextMode::NoToolCalls => ContextReport {
            mode,
            success_score: 0.12,
            iterations: 2,
            tool_calls: 0,
            failure_mode: "can describe a plan but cannot interact with the environment",
        },
        ContextMode::NoToolResults => ContextReport {
            mode,
            success_score: 0.18,
            iterations: 10,
            tool_calls: 8,
            failure_mode: "acts blindly because observations never return to the model",
        },
    }
}

pub fn demo_context_ablation() -> String {
    let mut lines = vec![
        "Week 2 - Context Ablation".to_string(),
        "mode              score   iterations   tool calls   failure mode".to_string(),
        "----------------  ------  -----------  ----------   ------------".to_string(),
    ];

    for mode in ContextMode::all() {
        let report = evaluate_context_mode(*mode);
        lines.push(format!(
            "{:<16}  {:>5.0}%  {:>11}  {:>10}   {}",
            report.mode.label(),
            report.success_score * 100.0,
            report.iterations,
            report.tool_calls,
            report.failure_mode
        ));
    }

    lines.push(String::new());
    lines.push(
        "Takeaway: context is the Agent operating system; removing observations or actions breaks the loop fastest."
            .to_string(),
    );
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_context_scores_highest() {
        let full = evaluate_context_mode(ContextMode::Full);
        let no_tools = evaluate_context_mode(ContextMode::NoToolCalls);
        assert!(full.success_score > no_tools.success_score);
    }
}
