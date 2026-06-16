#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Perception,
    Execution,
    Collaboration,
}

impl ToolKind {
    pub fn label(self) -> &'static str {
        match self {
            ToolKind::Perception => "perception",
            ToolKind::Execution => "execution",
            ToolKind::Collaboration => "collaboration",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

impl RiskLevel {
    pub fn label(self) -> &'static str {
        match self {
            RiskLevel::Low => "low",
            RiskLevel::Medium => "medium",
            RiskLevel::High => "high",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: &'static str,
    pub kind: ToolKind,
    pub risk: RiskLevel,
    pub description: &'static str,
    pub keywords: &'static [&'static str],
}

#[derive(Debug, Clone, Default)]
pub struct ToolRegistry {
    tools: Vec<ToolSpec>,
}

impl ToolRegistry {
    pub fn new(tools: Vec<ToolSpec>) -> Self {
        Self { tools }
    }

    pub fn standard() -> Self {
        Self::new(vec![
            ToolSpec {
                name: "web_search",
                kind: ToolKind::Perception,
                risk: RiskLevel::Low,
                description: "Search public web pages and summarize candidate sources.",
                keywords: &["search", "latest", "news", "web", "research"],
            },
            ToolSpec {
                name: "document_parse",
                kind: ToolKind::Perception,
                risk: RiskLevel::Low,
                description: "Extract text and tables from local documents.",
                keywords: &["pdf", "doc", "file", "extract", "parse"],
            },
            ToolSpec {
                name: "code_interpreter",
                kind: ToolKind::Execution,
                risk: RiskLevel::Medium,
                description: "Run bounded code for calculation and data transformation.",
                keywords: &["calculate", "python", "code", "analyze", "plot"],
            },
            ToolSpec {
                name: "file_edit",
                kind: ToolKind::Execution,
                risk: RiskLevel::Medium,
                description: "Apply scoped edits to files after reading the target context.",
                keywords: &["edit", "modify", "refactor", "write", "fix"],
            },
            ToolSpec {
                name: "shell_session",
                kind: ToolKind::Execution,
                risk: RiskLevel::High,
                description: "Run terminal commands with explicit safety checks.",
                keywords: &["test", "build", "install", "run", "terminal"],
            },
            ToolSpec {
                name: "human_approval",
                kind: ToolKind::Collaboration,
                risk: RiskLevel::Low,
                description: "Ask a human reviewer before irreversible or expensive actions.",
                keywords: &["approval", "confirm", "human", "review", "payment"],
            },
            ToolSpec {
                name: "browser_replay",
                kind: ToolKind::Collaboration,
                risk: RiskLevel::Medium,
                description: "Replay a parameterized browser workflow.",
                keywords: &["browser", "form", "website", "rpa", "click"],
            },
        ])
    }

    pub fn tools(&self) -> &[ToolSpec] {
        &self.tools
    }

    pub fn select(&self, task: &str, limit: usize) -> Vec<ToolSpec> {
        let task = task.to_ascii_lowercase();
        let mut scored = self
            .tools
            .iter()
            .map(|tool| {
                let score = tool
                    .keywords
                    .iter()
                    .filter(|keyword| task.contains(**keyword))
                    .count();
                (score, tool)
            })
            .filter(|(score, _)| *score > 0)
            .collect::<Vec<_>>();

        scored.sort_by(|(left_score, left_tool), (right_score, right_tool)| {
            right_score
                .cmp(left_score)
                .then_with(|| risk_rank(left_tool.risk).cmp(&risk_rank(right_tool.risk)))
        });

        scored
            .into_iter()
            .take(limit)
            .map(|(_, tool)| tool.clone())
            .collect()
    }
}

fn risk_rank(risk: RiskLevel) -> u8 {
    match risk {
        RiskLevel::Low => 0,
        RiskLevel::Medium => 1,
        RiskLevel::High => 2,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodingAgentStep {
    pub name: &'static str,
    pub purpose: &'static str,
}

pub fn coding_agent_loop() -> Vec<CodingAgentStep> {
    vec![
        CodingAgentStep {
            name: "inspect",
            purpose: "Read relevant files, tests, and project conventions before editing.",
        },
        CodingAgentStep {
            name: "plan",
            purpose: "Choose the smallest coherent change that satisfies the task.",
        },
        CodingAgentStep {
            name: "edit",
            purpose: "Apply scoped file changes and preserve unrelated user work.",
        },
        CodingAgentStep {
            name: "verify",
            purpose: "Run the nearest useful build, test, lint, or smoke check.",
        },
        CodingAgentStep {
            name: "report",
            purpose: "Summarize changes, evidence, and remaining risks.",
        },
    ]
}

pub fn demo_tool_ecosystem() -> String {
    let task = "Research latest browser automation options, edit the project, then run tests.";
    let registry = ToolRegistry::standard();
    let selected = registry.select(task, 4);

    let mut lines = vec![
        "Week 4 - Tool Ecosystem".to_string(),
        format!("task: {task}"),
        "selected tools:".to_string(),
    ];

    for tool in selected {
        lines.push(format!(
            "  - {} [{} / risk:{}] {}",
            tool.name,
            tool.kind.label(),
            tool.risk.label(),
            tool.description
        ));
    }

    lines.push(String::new());
    lines.push("Takeaway: tools are Agent hands; the registry should make capability, risk, and selection explicit.".to_string());
    lines.join("\n")
}

pub fn demo_coding_agent() -> String {
    let mut lines = vec![
        "Week 5 - Coding Agent".to_string(),
        "production loop:".to_string(),
    ];

    for step in coding_agent_loop() {
        lines.push(format!("  - {}: {}", step.name, step.purpose));
    }

    lines.push(String::new());
    lines.push("tooling baseline:".to_string());
    for tool in ToolRegistry::standard().tools() {
        if matches!(tool.name, "file_edit" | "shell_session" | "document_parse") {
            lines.push(format!(
                "  - {} [{} / risk:{}] {}",
                tool.name,
                tool.kind.label(),
                tool.risk.label(),
                tool.description
            ));
        }
    }

    lines.push(String::new());
    lines.push("Takeaway: a coding agent is less about one clever prompt and more about a disciplined inspect/edit/verify loop.".to_string());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_tools_by_task_keywords() {
        let registry = ToolRegistry::standard();
        let selected = registry.select("edit files and run tests", 3);
        let names = selected.iter().map(|tool| tool.name).collect::<Vec<_>>();
        assert!(names.contains(&"file_edit"));
        assert!(names.contains(&"shell_session"));
    }
}
