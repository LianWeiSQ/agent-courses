const TITLE: &str = r#"用户记忆评估框架"#;
const SUMMARY: &str = r#"为用户记忆系统设计准确性、相关性和覆盖度评估。"#;
const CONCEPTS: &[&str] = &[r#"评估"#, r#"记忆"#, r#"指标"#];

fn expected_score(a: f64, b: f64) -> f64 {
    1.0 / (1.0 + 10_f64.powf((b - a) / 400.0))
}

fn update_elo(winner: f64, loser: f64, k: f64) -> (f64, f64) {
    let winner_new = winner + k * (1.0 - expected_score(winner, loser));
    let loser_new = loser + k * (0.0 - expected_score(loser, winner));
    (winner_new, loser_new)
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!("核心概念：{}", CONCEPTS.join(" / "));
    let (stronger, baseline) = update_elo(1500.0, 1500.0, 32.0);
    println!(
        "
一次对战后：stronger={:.1}, baseline={:.1}",
        stronger, baseline
    );
    println!("结论：评估代码要把任务、裁判、指标和不确定性分开建模。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winner_gains_rating() {
        let (winner, loser) = update_elo(1500.0, 1500.0, 32.0);
        assert!(winner > 1500.0);
        assert!(loser < 1500.0);
    }
}
