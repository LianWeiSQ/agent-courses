# Agent Courses Rust 重构版

这个仓库保留原课程的 `week1` 到 `week8` 目录结构，并把每个项目目录里的代码入口重构为 Rust。文章类内容也重新整理成更适合学习者阅读的说明：先讲目标，再给运行命令，再说明核心概念和扩展方向。

## 重要约定

- 原课程目录形态保留：`week1/...` 到 `week8/...`。
- 每个项目目录都是一个 Rust binary crate。
- 代码入口统一为 `src/main.rs`。
- 每个项目都有重写后的 `README.md`。
- 原来额外的 Markdown 文章路径也尽量保留，并改写为 Rust 版学习说明。

## 快速开始

```bash
cargo test
cargo run -p agent-course-week1-learning-from-experience
cargo run -p agent-course-week3-retrieval-pipeline
cargo run -p agent-course-week5-coding-agent
```

查看某个项目的包名时，可以直接看对应目录下的 `Cargo.toml`。

## 课程目录

| 路径 | 课程 | Rust 版目标 |
| --- | --- | --- |
| `week1/learning-from-experience` | 经验学习：RL vs LLM | 通过寻宝游戏对比传统强化学习和语言模型上下文学习，理解样本效率与先验知识。 |
| `week1/web-search-agent` | Web Search Agent | 把网络搜索建模为 Agent 的感知工具，并练习多轮检索与答案综合。 |
| `week1/search-codegen` | 搜索与代码分析 Agent | 组合搜索、代码执行和推理，把复杂分析任务拆成可验证步骤。 |
| `week1/context` | 上下文消融研究 | 移除历史、推理、工具调用或工具结果，观察 Agent 行为如何退化。 |
| `week2/local_llm_serving` | 本地 LLM 部署与工具调用 | 理解本地模型服务、Chat Template、流式输出和工具调用之间的关系。 |
| `week2/attention_visualization` | 注意力机制可视化 | 把 token 序列和注意力关系变成可观察对象，帮助理解模型如何使用上下文。 |
| `week2/kv-cache` | KV Cache 友好的上下文设计 | 演示上下文布局如何影响缓存复用、延迟和成本。 |
| `week2/context-compression` | 上下文压缩策略 | 比较摘要、抽取和结构化压缩，学习在 token 预算内保留关键信息。 |
| `week2/prompt-engineering` | 提示工程消融研究 | 用可控实验比较提示结构、语气、工具描述对 Agent 的影响。 |
| `week2/system-hint` | 系统提示优化 | 研究系统提示如何约束行为、提高一致性并减少错误动作。 |
| `week2/user-memory-evaluation` | 用户记忆评估框架 | 为用户记忆系统设计准确性、相关性和覆盖度评估。 |
| `week2/user-memory` | 用户长期记忆系统 | 用结构化记忆保存用户偏好、事实和跨会话上下文。 |
| `week2/log-sanitization` | 日志脱敏处理 | 在可调试性和隐私保护之间做工程权衡。 |
| `week3/dense-embedding` | 稠密嵌入向量检索服务 | 用向量相似度理解语义检索和近似最近邻索引。 |
| `week3/sparse-embedding` | BM25 稀疏检索引擎 | 从词频、倒排索引和 BM25 理解传统搜索。 |
| `week3/retrieval-pipeline` | 混合检索流水线 | 融合稠密检索、稀疏检索和重排序，提升复杂问题召回质量。 |
| `week3/multimodal-agent` | 多模态信息提取 | 比较原生多模态、文本提取和工具化分析三种路线。 |
| `week3/structured-index` | 结构化索引 | 用层次摘要和图结构表示知识内部关系。 |
| `week3/agentic-rag` | Agentic RAG | 让 Agent 主动规划、检索、验证和补充信息。 |
| `week3/agentic-rag-for-user-memory` | 用户记忆 Agentic RAG | 把 Agentic RAG 用在跨会话用户记忆检索。 |
| `week3/contextual-retrieval` | 上下文感知检索 | 为 chunk 添加上下文锚点，缓解分块造成的语义丢失。 |
| `week3/contextual-retrieval-for-user-memory` | 上下文感知用户记忆 | 结合结构化事实卡和上下文检索，构造双层记忆。 |
| `week3/structured-knowledge-extraction` | 结构化知识提取 | 从非结构化文本抽取因素、规则和可复用判断逻辑。 |
| `week3/gaia-experience` | 从成功经验中学习 | 把成功任务轨迹总结为可检索经验，并在新任务中复用。 |
| `week3/browser-use-rpa` | 工作流录制与回放 | 把重复浏览器操作抽象为参数化工具，降低推理成本。 |
| `week4/perception-tools` | 感知工具 MCP 服务器 | 封装搜索、文件、多模态和公共数据源，形成 Agent 感知层。 |
| `week4/execution-tools` | 执行工具 MCP 服务器 | 实现文件、代码、终端执行工具，并加入安全边界。 |
| `week4/collaboration-tools` | 协作工具 MCP 服务器 | 连接浏览器、人类审批、通知和定时器，让 Agent 能协作。 |
| `week4/agent-with-event-trigger` | 事件触发型 Agent | 用事件、HTTP API 和工具编排构建异步 Agent 系统。 |
| `week5/coding-agent` | 生产级 Coding Agent | 实现 inspect-plan-edit-verify-report 的代码编辑闭环。 |
| `week6/android-world` | Android 环境基准 | 理解移动 UI 任务如何评估 Agent 的观察、规划和执行。 |
| `week6/elo-leaderboard` | ELO 排行榜系统 | 用成对比较和 ELO 更新评估不同 Agent 的相对能力。 |
| `week7/AWorld-train` | 具身 Agent 训练 | 从环境交互轨迹中学习策略，理解具身任务训练闭环。 |
| `week7/AdaptThink` | 自适应推理深度 | 根据任务难度切换推理深度，在准确率和成本之间平衡。 |
| `week7/Intuitor` | 直觉推理训练 | 训练模型快速做出可靠判断，减少不必要的长推理。 |
| `week7/MiniMind-pretrain` | 小型模型预训练 | 用小模型理解预训练数据、优化循环和评估流程。 |
| `week7/MultilingualReasoning` | 多语言推理 | 提升模型在跨语言任务中的推理稳定性。 |
| `week7/SimpleVLA-RL` | 视觉-语言-动作 RL | 把视觉、语言和动作反馈放入强化学习训练。 |
| `week7/SpatialReasoning` | 空间推理训练 | 训练位置、方向、距离和几何关系推理能力。 |
| `week7/continued-pretraining` | 持续预训练 | 在领域语料上继续预训练，让模型吸收目标领域知识。 |
| `week7/feedback-guided-sampling` | 反馈引导采样 | 用奖励或偏好信号引导采样，提高候选解质量。 |
| `week7/learn-from-observation` | 观察学习 | 从示范轨迹学习动作模式，不依赖显式逐步标注。 |
| `week7/orpheus` | 音乐生成与理解 | 把音频/音乐生成任务纳入模型训练视角。 |
| `week7/prompt-distillation` | 提示蒸馏 | 把长提示中的策略压缩进模型行为，降低推理成本。 |
| `week7/retool` | 工具增强数学推理 | 让模型学会调用代码工具辅助数学推理。 |
| `week7/sesame` | 序列建模与评估 | 围绕序列建模任务组织训练、评估和调优。 |
| `week8/live-audio` | 实时语音对话 | 设计 ASR、LLM、TTS 和 WebSocket 的低延迟实时链路。 |

## 重构边界

这次重构不是另起一个抽象项目，而是在原章节目录里完成 Rust 化。为了让整套课程可以本地稳定编译，外部模型 API、浏览器、训练集和大文件资源没有直接搬进仓库；它们在 README 的扩展方向里作为下一步接入点保留。

## 验证

```bash
cargo fmt --all
cargo test
```

## License

MIT
