#[derive(Debug, Clone, PartialEq)]
pub struct PipelineStage {
    pub name: &'static str,
    pub target_latency_ms: u32,
    pub responsibility: &'static str,
}

pub fn live_audio_pipeline() -> Vec<PipelineStage> {
    vec![
        PipelineStage {
            name: "voice_activity_detection",
            target_latency_ms: 30,
            responsibility: "detect speech boundaries without waiting for full utterances",
        },
        PipelineStage {
            name: "streaming_asr",
            target_latency_ms: 180,
            responsibility: "turn partial audio into partial text",
        },
        PipelineStage {
            name: "llm_turn",
            target_latency_ms: 600,
            responsibility: "produce the next conversational action or tool call",
        },
        PipelineStage {
            name: "streaming_tts",
            target_latency_ms: 240,
            responsibility: "start audio playback before the full sentence is complete",
        },
        PipelineStage {
            name: "websocket_transport",
            target_latency_ms: 50,
            responsibility: "move audio and events between browser and server",
        },
    ]
}

pub fn total_latency_budget(stages: &[PipelineStage]) -> u32 {
    stages.iter().map(|stage| stage.target_latency_ms).sum()
}

pub fn demo_live_interaction() -> String {
    let stages = live_audio_pipeline();
    let mut lines = vec![
        "Week 8 - Live Audio Interaction".to_string(),
        "latency budget:".to_string(),
    ];

    for stage in &stages {
        lines.push(format!(
            "  - {}: {}ms ({})",
            stage.name, stage.target_latency_ms, stage.responsibility
        ));
    }

    lines.push(format!("total budget: {}ms", total_latency_budget(&stages)));
    lines.push(String::new());
    lines.push("Takeaway: real-time agents are systems problems; streaming boundaries matter as much as model quality.".to_string());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_budget_is_under_realtime_threshold() {
        let stages = live_audio_pipeline();
        assert!(total_latency_budget(&stages) <= 1_200);
    }
}
