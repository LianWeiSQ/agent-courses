const TITLE: &str = r#"持续预训练"#;
const SUMMARY: &str = r#"在领域语料上继续预训练，让模型吸收目标领域知识。"#;
const CONCEPTS: &[&str] = &[r#"持续预训练"#, r#"领域适应"#, r#"数据配方"#];

#[derive(Debug, Clone, Copy)]
enum Method {
    Sft,
    Rl,
    Distill,
}

fn simulated_score(method: Method, step: u32) -> f32 {
    let x = step as f32 / 10.0;
    match method {
        Method::Sft => 0.45 + 0.35 * x,
        Method::Rl => 0.30 + 0.50 * x * x,
        Method::Distill => 0.55 + 0.20 * x,
    }
    .min(0.98)
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!("核心概念：{}", CONCEPTS.join(" / "));
    println!(
        "
训练曲线模拟："
    );
    for step in [0, 3, 6, 10] {
        println!(
            "  step={} sft={:.0}% rl={:.0}% distill={:.0}%",
            step,
            simulated_score(Method::Sft, step) * 100.0,
            simulated_score(Method::Rl, step) * 100.0,
            simulated_score(Method::Distill, step) * 100.0
        );
    }
    println!(
        "
结论：后训练不是一个按钮，而是在数据、奖励、成本和可解释性之间做选择。"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn training_improves_score() {
        assert!(simulated_score(Method::Sft, 10) > simulated_score(Method::Sft, 0));
        assert!(simulated_score(Method::Rl, 10) > simulated_score(Method::Rl, 0));
    }
}
