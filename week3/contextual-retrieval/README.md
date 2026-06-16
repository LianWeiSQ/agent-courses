# 上下文感知检索

> 课程路径：`week3/contextual-retrieval`

## 这节课要解决什么

为 chunk 添加上下文锚点，缓解分块造成的语义丢失。

## 学完应该能说清楚

- 这个项目在 Agent = Model + Context + Tools 中属于哪一层。
- 课程示例里的核心状态、动作、工具或评估指标是什么。
- 如何用类型、结构体和小型测试把概念固定下来。

## 怎么运行

从仓库根目录运行：

```bash
cargo run -p agent-course-week3-contextual-retrieval
```

单独测试这一节：

```bash
cargo test -p agent-course-week3-contextual-retrieval
```

## 核心概念

- 上下文增强
- 语义锚定
- 检索优化

## 实现说明

代码入口在 `src/main.rs`。说明文档按学习路径、运行方式、核心概念和后续扩展建议组织。

## 下一步可以怎么扩展

- 把当前 demo 中的模拟数据换成真实模型、真实检索或真实工具。
- 为关键状态转移增加更多单元测试。
- 如果需要接外部 API，把 provider 放到独立模块，不要混进课程主流程。
