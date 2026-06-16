# Rust Porting Notes

## Porting Strategy

The source course contains many Python projects, external APIs, large datasets, model-serving examples, and media artifacts. This Rust edition focuses on the durable engineering concepts:

- state machines over ad hoc scripts
- typed context and tool contracts
- deterministic examples that run without API keys
- small tests that explain the expected behavior
- a CLI that makes the course discoverable

## Mapping From Source Course To Rust Crates

| Source area | Rust module | Current scope |
| --- | --- | --- |
| Week 1 learning-from-experience | `agent_core::experience` | Treasure hunt environment, Q-learning, language-prior demo |
| Week 1/2 context projects | `agent_core::context` | Context ablation scoring model |
| Week 3 retrieval projects | `agent_core::retrieval` | Tokenizer, BM25, simple hybrid retrieval |
| Week 4 tools + Week 5 coding agent | `agent_core::tools` | Tool registry, risk model, selection, coding loop |
| Week 6 evaluation | `agent_core::benchmarks` | Benchmark surface model and ELO update |
| Week 7 post-training | `agent_core::training` | Training method playbook and simulated curves |
| Week 8 live-audio | `agent_core::interaction` | Streaming audio pipeline and latency budget |

## What Was Intentionally Not Copied

- Python virtual environments and generated output.
- Large media/data files.
- API-key-specific provider implementations.
- One-off notebook-style scripts that do not add a stable course concept.

Those can be rebuilt in Rust as separate crates once the desired provider and runtime choices are fixed.

## Suggested Next Milestones

1. Add a real LLM trait and provider adapters.
2. Add a safe execution sandbox for command and code tools.
3. Replace the simple hybrid retriever with embeddings and an ANN index.
4. Add MCP-compatible tool descriptors.
5. Split each week into a standalone crate once examples grow beyond one module.

