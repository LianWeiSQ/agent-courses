const TITLE: &str = r#"日志脱敏处理"#;
const SUMMARY: &str = r#"在可调试性和隐私保护之间做工程权衡。"#;
const CONCEPTS: &[&str] = &[r#"隐私"#, r#"日志"#, r#"安全"#];

#[derive(Debug, Clone)]
struct Stage {
    name: &'static str,
    output: &'static str,
}

fn lesson_pipeline() -> Vec<Stage> {
    vec![
        Stage {
            name: "define",
            output: "明确任务边界和成功标准",
        },
        Stage {
            name: "model",
            output: "把课程概念转成可执行的数据结构",
        },
        Stage {
            name: "run",
            output: "运行一个确定性 demo，观察状态变化",
        },
        Stage {
            name: "extend",
            output: "替换模拟层，接入真实服务或数据",
        },
    ]
}

fn score_signal(completed_steps: usize, total_steps: usize) -> f32 {
    if total_steps == 0 {
        0.0
    } else {
        completed_steps as f32 / total_steps as f32
    }
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!(
        "
核心概念：{}",
        CONCEPTS.join(" / ")
    );
    println!(
        "
Rust demo pipeline:"
    );
    let stages = lesson_pipeline();
    for (index, stage) in stages.iter().enumerate() {
        println!("  {}. {} -> {}", index + 1, stage.name, stage.output);
    }
    println!(
        "
完成度信号：{:.0}%",
        score_signal(stages.len(), stages.len()) * 100.0
    );
    println!("迁移说明：这个版本保留原课程目录，把 Python 脚本重构为 Rust 的类型化、可测试入口。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_has_learning_steps() {
        assert!(lesson_pipeline().len() >= 3);
        assert_eq!(score_signal(2, 4), 0.5);
    }
}
