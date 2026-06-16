# Tests

## 阅读目标

- 理解 Coding Agent 的测试不只是单元测试，还包括文件编辑、命令执行和验证闭环。
- 观察 Coding Agent 如何在工具选择、风险分级和验证步骤之间做权衡。
- 后续可以加入真实任务 fixture、临时目录和命令沙箱练习。

## 课堂关注点

- 明确每个工具的风险等级。
- 用测试证明工具选择符合任务描述。
- 保持 inspect -> plan -> edit -> verify -> report 的工程闭环。

## 建议练习

1. 运行 `cargo test -p agent-course-week5-coding-agent`。
2. 添加一个 `grep_search` 工具。
3. 写测试证明搜索任务会优先选择 `grep_search`。

