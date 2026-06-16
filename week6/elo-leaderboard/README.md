# ELO 排行榜系统

> Rust 重构版路径：`week6/elo-leaderboard`

## 这节课要解决什么

用成对比较和 ELO 更新评估不同 Agent 的相对能力。

## 学完应该能说清楚

- 这个项目在 Agent = Model + Context + Tools 中属于哪一层。
- 原始 Python 示例里的核心状态、动作、工具或评估指标是什么。
- Rust 版如何用类型、结构体和小型测试把概念固定下来。

## Rust 版怎么运行

从仓库根目录运行：

```bash
cargo run -p agent-course-week6-elo-leaderboard
```

单独测试这一节：

```bash
cargo test -p agent-course-week6-elo-leaderboard
```

## 核心概念

- ELO
- 排行榜
- 相对评估

## 重构说明

这一版保留原课程目录，不再放 Python 脚本。涉及代码的部分已经改为 Rust CLI demo，入口在 `src/main.rs`。涉及文章说明的部分重写为学习路径、运行方式、核心概念和后续扩展建议。

## 下一步可以怎么扩展

- 把当前 demo 中的模拟数据换成真实模型、真实检索或真实工具。
- 为关键状态转移增加更多单元测试。
- 如果需要接外部 API，把 provider 放到独立模块，不要混进课程主流程。
