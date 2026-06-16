# Tests

## 阅读目标

- 理解 Coding Agent 的测试不只是单元测试，还包括文件编辑、命令执行和验证闭环。
- 对照 `week5/coding-agent/src/main.rs`，看工具选择和风险分级如何被写成可测试代码。
- 后续可以把这里扩展为真实 fixture、临时目录和命令沙箱测试。

## 实现关注点

- 用类型明确工具风险。
- 用测试证明工具选择符合任务描述。
- 保持 inspect -> plan -> edit -> verify -> report 的工程闭环。

## 建议练习

1. 运行 `cargo test -p agent-course-week5-coding-agent`。
2. 添加一个 `grep_search` 工具。
3. 写测试证明搜索任务会优先选择 `grep_search`。

