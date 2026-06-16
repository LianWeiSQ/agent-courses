const TITLE: &str = r#"协作工具 MCP 服务器"#;
const SUMMARY: &str = r#"连接浏览器、人类审批、通知和定时器，让 Agent 能协作。"#;
const CONCEPTS: &[&str] = &[r#"协作工具"#, r#"HITL"#, r#"通知"#];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Risk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy)]
struct Tool {
    name: &'static str,
    kind: &'static str,
    risk: Risk,
    keyword: &'static str,
}

fn registry() -> Vec<Tool> {
    vec![
        Tool {
            name: "web_search",
            kind: "perception",
            risk: Risk::Low,
            keyword: "search",
        },
        Tool {
            name: "file_edit",
            kind: "execution",
            risk: Risk::Medium,
            keyword: "edit",
        },
        Tool {
            name: "code_runner",
            kind: "execution",
            risk: Risk::High,
            keyword: "run",
        },
        Tool {
            name: "human_approval",
            kind: "collaboration",
            risk: Risk::Low,
            keyword: "approve",
        },
    ]
}

fn select_tools(task: &str) -> Vec<Tool> {
    let task = task.to_ascii_lowercase();
    registry()
        .into_iter()
        .filter(|tool| task.contains(tool.keyword))
        .collect()
}

fn risk_label(risk: Risk) -> &'static str {
    match risk {
        Risk::Low => "low",
        Risk::Medium => "medium",
        Risk::High => "high",
    }
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!("核心概念：{}", CONCEPTS.join(" / "));
    let task = "search docs, edit code, run tests, approve release";
    println!(
        "
任务：{}",
        task
    );
    for tool in select_tools(task) {
        println!(
            "  - {} [{} / risk={}]",
            tool.name,
            tool.kind,
            risk_label(tool.risk)
        );
    }
    println!(
        "
结论：工具设计的重点不是 API 形状，而是能力、风险和选择策略都显式化。"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_execution_tool() {
        let selected = select_tools("edit and run");
        assert!(selected.iter().any(|tool| tool.name == "file_edit"));
        assert!(selected.iter().any(|tool| tool.name == "code_runner"));
    }
}
