# Android 环境基准

> 课程路径：`week6/android-world`

## 这节课要解决什么

理解移动 UI 任务如何评估 Agent 的观察、规划和执行。

## 学完应该能说清楚

- 这个项目在 Agent = Model + Context + Tools 中属于哪一层。
- 课程示例里的核心状态、动作、工具或评估指标是什么。
- 这个能力如何帮助 Agent 完成真实任务，以及在哪些场景容易失败。

## 课堂演示

从仓库根目录运行：

```bash
cargo run -p agent-course-week6-android-world
```

单独测试这一节：

```bash
cargo test -p agent-course-week6-android-world
```

## 核心概念

- Android
- UI 自动化
- 评估

## 课堂练习

先运行演示，观察 Agent 的状态、工具选择、检索结果或评估信号，再尝试改动任务输入，比较输出如何变化。

## 课后练习

- 把当前演示中的模拟数据换成真实模型、真实检索或真实工具。
- 补充一个失败案例，解释 Agent 为什么会做错。
- 如果接入外部 API，把 provider 当成工具边界，保持课程主流程清晰。
