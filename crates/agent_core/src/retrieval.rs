use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub body: String,
}

impl Document {
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub id: String,
    pub title: String,
    pub score: f64,
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_ascii_lowercase())
        .collect()
}

#[derive(Debug, Clone)]
pub struct Bm25Index {
    documents: Vec<Document>,
    term_frequency: Vec<HashMap<String, usize>>,
    document_frequency: HashMap<String, usize>,
    average_document_length: f64,
}

impl Bm25Index {
    pub fn new(documents: Vec<Document>) -> Self {
        let mut term_frequency = Vec::with_capacity(documents.len());
        let mut document_frequency = HashMap::new();
        let mut total_len = 0_usize;

        for document in &documents {
            let mut counts = HashMap::new();
            let tokens = tokenize(&format!("{} {}", document.title, document.body));
            total_len += tokens.len();

            for token in tokens {
                *counts.entry(token).or_insert(0) += 1;
            }

            for token in counts.keys() {
                *document_frequency.entry(token.clone()).or_insert(0) += 1;
            }

            term_frequency.push(counts);
        }

        let average_document_length = if documents.is_empty() {
            0.0
        } else {
            total_len as f64 / documents.len() as f64
        };

        Self {
            documents,
            term_frequency,
            document_frequency,
            average_document_length,
        }
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<SearchHit> {
        let query_terms = tokenize(query);
        let mut hits = self
            .documents
            .iter()
            .enumerate()
            .map(|(index, document)| SearchHit {
                id: document.id.clone(),
                title: document.title.clone(),
                score: self.score_document(index, &query_terms),
            })
            .filter(|hit| hit.score > 0.0)
            .collect::<Vec<_>>();

        hits.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(top_k);
        hits
    }

    fn score_document(&self, index: usize, query_terms: &[String]) -> f64 {
        let k1 = 1.5;
        let b = 0.75;
        let frequencies = &self.term_frequency[index];
        let document_len = frequencies.values().sum::<usize>() as f64;
        let corpus_len = self.documents.len() as f64;

        query_terms
            .iter()
            .map(|term| {
                let tf = *frequencies.get(term).unwrap_or(&0) as f64;
                if tf == 0.0 {
                    return 0.0;
                }
                let df = *self.document_frequency.get(term).unwrap_or(&0) as f64;
                let idf = ((corpus_len - df + 0.5) / (df + 0.5) + 1.0).ln();
                let denominator =
                    tf + k1 * (1.0 - b + b * document_len / self.average_document_length.max(1.0));
                idf * (tf * (k1 + 1.0)) / denominator
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct HybridRetriever {
    bm25: Bm25Index,
}

impl HybridRetriever {
    pub fn new(documents: Vec<Document>) -> Self {
        Self {
            bm25: Bm25Index::new(documents),
        }
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<SearchHit> {
        let lexical_hits = self.bm25.search(query, usize::MAX);
        let query_tokens = tokenize(query).into_iter().collect::<HashSet<_>>();
        let mut hits = lexical_hits
            .into_iter()
            .map(|mut hit| {
                let document = self
                    .bm25
                    .documents
                    .iter()
                    .find(|document| document.id == hit.id)
                    .expect("search hit should reference an indexed document");
                hit.score = 0.75 * hit.score
                    + 0.25
                        * soft_semantic_overlap(
                            &query_tokens,
                            &tokenize(&format!("{} {}", document.title, document.body)),
                        );
                hit
            })
            .collect::<Vec<_>>();

        hits.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(top_k);
        hits
    }
}

fn soft_semantic_overlap(query: &HashSet<String>, document_tokens: &[String]) -> f64 {
    if query.is_empty() || document_tokens.is_empty() {
        return 0.0;
    }

    let document = document_tokens.iter().cloned().collect::<HashSet<_>>();
    let exact = query.intersection(&document).count() as f64;
    let fuzzy = query
        .iter()
        .filter(|query_token| {
            document
                .iter()
                .any(|doc_token| first_three(query_token) == first_three(doc_token))
        })
        .count() as f64;

    (exact + 0.35 * fuzzy) / query.len() as f64
}

fn first_three(text: &str) -> &str {
    text.get(..text.len().min(3)).unwrap_or(text)
}

pub fn sample_documents() -> Vec<Document> {
    vec![
        Document::new(
            "week1",
            "Learning from Experience",
            "Q-learning requires many episodes, while language priors can reason from sparse experience.",
        ),
        Document::new(
            "week2",
            "Context Engineering",
            "System prompts, history, reasoning traces, tool calls, and observations shape agent behavior.",
        ),
        Document::new(
            "week3",
            "Hybrid Retrieval",
            "BM25 sparse retrieval, dense embeddings, reranking, and contextual chunks improve recall.",
        ),
        Document::new(
            "week5",
            "Coding Agent",
            "A coding agent edits files, searches code, executes commands, tracks todos, and verifies changes.",
        ),
    ]
}

pub fn demo_retrieval() -> String {
    let retriever = HybridRetriever::new(sample_documents());
    let hits = retriever.search("agent search tool context memory", 3);
    let mut lines = vec![
        "Week 3 - Retrieval Pipeline".to_string(),
        "query: agent search tool context memory".to_string(),
        "top hits:".to_string(),
    ];

    for (rank, hit) in hits.iter().enumerate() {
        lines.push(format!(
            "  {}. {} ({}) score={:.3}",
            rank + 1,
            hit.title,
            hit.id,
            hit.score
        ));
    }

    lines.push(String::new());
    lines.push(
        "Takeaway: sparse lexical matching is understandable; hybrid scoring makes it more tolerant."
            .to_string(),
    );
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_normalizes_text() {
        assert_eq!(tokenize("Agent, Tools!"), vec!["agent", "tools"]);
    }

    #[test]
    fn bm25_finds_relevant_document() {
        let index = Bm25Index::new(sample_documents());
        let hits = index.search("coding files commands", 1);
        assert_eq!(hits[0].id, "week5");
    }

    #[test]
    fn hybrid_retriever_returns_ranked_hits() {
        let retriever = HybridRetriever::new(sample_documents());
        let hits = retriever.search("context history tool observations", 2);
        assert_eq!(hits[0].id, "week2");
    }
}
