#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingMethod {
    SupervisedFineTuning,
    ReinforcementLearning,
    ContinuedPretraining,
    PromptDistillation,
    FeedbackGuidedSampling,
}

impl TrainingMethod {
    pub fn label(self) -> &'static str {
        match self {
            TrainingMethod::SupervisedFineTuning => "SFT",
            TrainingMethod::ReinforcementLearning => "RL",
            TrainingMethod::ContinuedPretraining => "continued pretraining",
            TrainingMethod::PromptDistillation => "prompt distillation",
            TrainingMethod::FeedbackGuidedSampling => "feedback-guided sampling",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingRecommendation {
    pub method: TrainingMethod,
    pub when_to_use: &'static str,
    pub risk: &'static str,
}

pub fn training_playbook() -> Vec<TrainingRecommendation> {
    vec![
        TrainingRecommendation {
            method: TrainingMethod::ContinuedPretraining,
            when_to_use: "the model lacks domain language or factual background",
            risk: "can overwrite general capability if the domain data is narrow or noisy",
        },
        TrainingRecommendation {
            method: TrainingMethod::SupervisedFineTuning,
            when_to_use: "you have high-quality demonstrations of the desired behavior",
            risk: "can imitate style without learning robust recovery",
        },
        TrainingRecommendation {
            method: TrainingMethod::ReinforcementLearning,
            when_to_use: "success can be scored and exploration reveals better strategies",
            risk: "reward hacking and expensive rollout generation",
        },
        TrainingRecommendation {
            method: TrainingMethod::PromptDistillation,
            when_to_use: "a long prompt works but is costly at inference time",
            risk: "distilled behavior may be harder to inspect and patch",
        },
        TrainingRecommendation {
            method: TrainingMethod::FeedbackGuidedSampling,
            when_to_use: "you need better candidates without changing model weights yet",
            risk: "sampling may optimize local preferences instead of final task quality",
        },
    ]
}

pub fn simulated_learning_curve(method: TrainingMethod, steps: usize) -> Vec<f64> {
    (0..steps)
        .map(|step| {
            let progress = step as f64 / steps.max(1) as f64;
            match method {
                TrainingMethod::ContinuedPretraining => 0.35 + 0.35 * progress.sqrt(),
                TrainingMethod::SupervisedFineTuning => 0.45 + 0.40 * progress,
                TrainingMethod::ReinforcementLearning => 0.30 + 0.55 * progress.powf(1.6),
                TrainingMethod::PromptDistillation => 0.50 + 0.25 * progress,
                TrainingMethod::FeedbackGuidedSampling => {
                    0.42 + 0.20 * (1.0 - (-4.0 * progress).exp())
                }
            }
            .min(0.98)
        })
        .collect()
}

pub fn demo_post_training() -> String {
    let mut lines = vec![
        "Week 7 - Post-training".to_string(),
        "method playbook:".to_string(),
    ];

    for item in training_playbook() {
        let curve = simulated_learning_curve(item.method, 5);
        let last = curve.last().copied().unwrap_or(0.0);
        lines.push(format!(
            "  - {}: use when {}; final simulated score {:.0}%; risk: {}",
            item.method.label(),
            item.when_to_use,
            last * 100.0,
            item.risk
        ));
    }

    lines.push(String::new());
    lines.push("Takeaway: SFT teaches imitation, RL optimizes scored behavior, and distillation moves prompt-time skill into cheaper runtime behavior.".to_string());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learning_curve_has_requested_length() {
        let curve = simulated_learning_curve(TrainingMethod::SupervisedFineTuning, 8);
        assert_eq!(curve.len(), 8);
        assert!(curve[7] > curve[0]);
    }
}
