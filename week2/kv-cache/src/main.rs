const TITLE: &str = r#"KV Cache 友好的上下文设计"#;
const SUMMARY: &str = r#"演示上下文布局如何影响缓存复用、延迟和成本。"#;
const CONCEPTS: &[&str] = &[r#"KV Cache"#, r#"上下文布局"#, r#"性能优化"#];

#[derive(Debug, Clone, Copy)]
struct ContextMode {
    name: &'static str,
    has_history: bool,
    has_reasoning: bool,
    has_tool_calls: bool,
    has_observations: bool,
}

impl ContextMode {
    fn score(self) -> u8 {
        let mut score = 20;
        if self.has_history {
            score += 15;
        }
        if self.has_reasoning {
            score += 20;
        }
        if self.has_tool_calls {
            score += 25;
        }
        if self.has_observations {
            score += 20;
        }
        score
    }
}

fn modes() -> Vec<ContextMode> {
    vec![
        ContextMode {
            name: "full",
            has_history: true,
            has_reasoning: true,
            has_tool_calls: true,
            has_observations: true,
        },
        ContextMode {
            name: "no_history",
            has_history: false,
            has_reasoning: true,
            has_tool_calls: true,
            has_observations: true,
        },
        ContextMode {
            name: "no_reasoning",
            has_history: true,
            has_reasoning: false,
            has_tool_calls: true,
            has_observations: true,
        },
        ContextMode {
            name: "no_tool_calls",
            has_history: true,
            has_reasoning: true,
            has_tool_calls: false,
            has_observations: true,
        },
        ContextMode {
            name: "no_tool_results",
            has_history: true,
            has_reasoning: true,
            has_tool_calls: true,
            has_observations: false,
        },
    ]
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!("核心概念：{}", CONCEPTS.join(" / "));
    println!(
        "
上下文消融："
    );
    for mode in modes() {
        println!("  - {:<15} score={}", mode.name, mode.score());
    }
    println!(
        "
结论：上下文不是一段 prompt，而是 Agent 的运行时状态。历史、推理、动作和观察缺一都会改变行为。"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_context_scores_highest() {
        let list = modes();
        let full = list[0].score();
        assert!(list.iter().all(|mode| full >= mode.score()));
    }
}
