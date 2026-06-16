# Agent Courses - Rust Edition

这是一套面向 AI Agent 工程实践的 Rust 版课程代码。它从原始 Python 课程项目重构而来，目标不是把每个脚本逐字翻译，而是把课程里的核心概念整理成更容易运行、理解和继续扩展的 Rust workspace。

## 你会学到什么

这套课程围绕一个简单但很有用的公式展开：

```text
Agent = Model + Context + Tools
```

- **Model**：负责理解、推理和决策。
- **Context**：保存系统提示、历史、工具调用、观察结果和用户记忆。
- **Tools**：让 Agent 搜索、执行、编辑、协作和感知外部世界。

本仓库把这些概念拆成 8 周，每周都有一个可以运行的 Rust demo。

## 快速开始

先确认已安装 Rust：

```bash
rustc --version
cargo --version
```

查看课程地图：

```bash
cargo run -p course_cli -- map
```

运行第一周 demo：

```bash
cargo run -p course_cli -- run week1
```

运行所有 demo：

```bash
cargo run -p course_cli -- run all
```

跑测试：

```bash
cargo test
```

## 课程结构

```text
.
├── crates
│   ├── agent_core      # Agent 课程核心库
│   └── course_cli      # 命令行课程入口
└── docs
    └── rust-porting-notes.md
```

### Week 1 - Agent 基础

运行：

```bash
cargo run -p course_cli -- run week1
```

内容：

- 寻宝游戏环境
- Q-learning 训练循环
- LLM-style language prior 的对照计划
- 样本效率对比

### Week 2 - 上下文工程

运行：

```bash
cargo run -p course_cli -- run week2
```

内容：

- full context
- no history
- no reasoning
- no tool calls
- no tool results

这部分用消融实验解释：为什么上下文不是“prompt 文本”，而是 Agent 的操作系统。

### Week 3 - 检索与记忆

运行：

```bash
cargo run -p course_cli -- run week3
```

内容：

- tokenizer
- BM25 稀疏检索
- 简化 hybrid retrieval
- 检索结果排序

### Week 4-5 - 工具生态与 Coding Agent

运行：

```bash
cargo run -p course_cli -- run week4
cargo run -p course_cli -- run week5
```

内容：

- perception / execution / collaboration 三类工具
- 工具风险分级
- keyword-based tool selection
- coding agent 的 inspect -> plan -> edit -> verify -> report 工作流

### Week 6 - Agent 评估

运行：

```bash
cargo run -p course_cli -- run week6
```

内容：

- benchmark surface 建模
- Terminal-Bench / SWE-Bench / GAIA / Android World 风格任务
- ELO 相对评分

### Week 7 - 模型后训练

运行：

```bash
cargo run -p course_cli -- run week7
```

内容：

- continued pretraining
- SFT
- RL
- prompt distillation
- feedback-guided sampling

### Week 8 - 实时多模态交互

运行：

```bash
cargo run -p course_cli -- run week8
```

内容：

- VAD
- streaming ASR
- LLM turn
- streaming TTS
- WebSocket transport
- 低延迟预算

## 常用命令

```bash
cargo run -p course_cli -- list       # 查看 8 周标题
cargo run -p course_cli -- map        # 查看完整课程地图
cargo run -p course_cli -- show week3 # 查看某一周的项目
cargo run -p course_cli -- run week3  # 运行某一周 demo
cargo run -p course_cli -- run all    # 运行全部 demo
cargo test                            # 测试核心逻辑
```

## 当前重构范围

已经完成：

- 初始化 Rust workspace。
- 把 8 周课程整理成 Rust manifest。
- 用 Rust 实现 Week 1 的寻宝环境、Q-learning 和 language-prior 对照。
- 用 Rust 实现 Week 2 上下文消融模型。
- 用 Rust 实现 Week 3 BM25 / hybrid retrieval demo。
- 用 Rust 实现 Week 4-5 工具注册表、工具选择和 Coding Agent 工作流。
- 用 Rust 实现 Week 6 ELO 评估 demo。
- 用 Rust 实现 Week 7 后训练方法 playbook。
- 用 Rust 实现 Week 8 实时语音链路延迟预算。

后续可以继续扩展：

- 接入真实 LLM provider。
- 接入真实 web search / browser automation / MCP server。
- 把每个 week 拆成更完整的独立 crate。
- 给检索模块加入真实 embedding 和向量索引。
- 给 Coding Agent 加入真实文件编辑和命令执行沙箱。

## 为什么用 Rust 重构

Rust 很适合做 Agent 系统的底座：

- 类型系统让状态、工具、风险和结果更清楚。
- 无需大型运行时也能写出可靠 CLI 和服务。
- 对执行工具、沙箱、并发和低延迟交互更友好。
- 很适合把“课程 demo”逐步演进成生产系统组件。

## License

MIT

