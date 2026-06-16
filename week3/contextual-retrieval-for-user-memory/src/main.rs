use std::collections::HashSet;

const TITLE: &str = r#"上下文感知用户记忆"#;
const SUMMARY: &str = r#"结合结构化事实卡和上下文检索，构造双层记忆。"#;
const CONCEPTS: &[&str] = &[r#"双层记忆"#, r#"事实卡"#, r#"主动服务"#];

#[derive(Debug, Clone)]
struct Document {
    id: &'static str,
    text: &'static str,
}

fn tokenize(text: &str) -> HashSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_ascii_lowercase())
        .collect()
}

fn score(query: &str, document: &Document) -> usize {
    let query_tokens = tokenize(query);
    let document_tokens = tokenize(document.text);
    query_tokens.intersection(&document_tokens).count()
}

fn corpus() -> Vec<Document> {
    vec![
        Document {
            id: "context",
            text: "agent context history reasoning tool observation memory",
        },
        Document {
            id: "retrieval",
            text: "bm25 embedding hybrid retrieval rerank chunk index",
        },
        Document {
            id: "training",
            text: "sft rl distillation reward sampling training",
        },
    ]
}

fn search(query: &str) -> Vec<(&'static str, usize)> {
    let mut hits = corpus()
        .iter()
        .map(|doc| (doc.id, score(query, doc)))
        .filter(|(_, score)| *score > 0)
        .collect::<Vec<_>>();
    hits.sort_by(|a, b| b.1.cmp(&a.1));
    hits
}

fn main() {
    println!("{}", TITLE);
    println!("{}", SUMMARY);
    println!("核心概念：{}", CONCEPTS.join(" / "));
    println!(
        "
查询：agent memory retrieval tool"
    );
    for (rank, (id, score)) in search("agent memory retrieval tool").iter().enumerate() {
        println!("  {}. {} score={}", rank + 1, id, score);
    }
    println!("
结论：当前示例先用可解释的 token/score 模型表达检索骨架，再逐步替换为真实 embedding、ANN 或 reranker。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_returns_relevant_hits() {
        let hits = search("embedding retrieval index");
        assert_eq!(hits[0].0, "retrieval");
    }
}
