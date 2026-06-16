const TITLE: &str = r#"实时语音对话"#;
const SUMMARY: &str = r#"设计 ASR、LLM、TTS 和 WebSocket 的低延迟实时链路。"#;
const CONCEPTS: &[&str] = &[r#"ASR"#, r#"TTS"#, r#"WebSocket"#, r#"实时交互"#];

#[derive(Debug, Clone, Copy)]
struct Stage {
    name: &'static str,
    latency_ms: u32,
}

fn pipeline() -> Vec<Stage> {
    vec![
        Stage {
            name: "vad",
            latency_ms: 30,
        },
        Stage {
            name: "streaming_asr",
            latency_ms: 180,
        },
        Stage {
            name: "llm_turn",
            latency_ms: 600,
        },
        Stage {
            name: "streaming_tts",
            latency_ms: 240,
        },
        Stage {
            name: "websocket",
            latency_ms: 50,
        },
    ]
}

fn total_latency() -> u32 {
    pipeline().iter().map(|stage| stage.latency_ms).sum()
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!("核心概念：{}", CONCEPTS.join(" / "));
    println!(
        "
实时链路："
    );
    for stage in pipeline() {
        println!("  - {}: {}ms", stage.name, stage.latency_ms);
    }
    println!("总预算：{}ms", total_latency());
    println!("结论：实时 Agent 是系统工程，流式边界和模型质量一样重要。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latency_budget_is_realtime() {
        assert!(total_latency() <= 1200);
    }
}
