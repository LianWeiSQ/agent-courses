# Docker Deployment

## 阅读目标

- 理解 Week 4 工具服务为什么需要可重复部署环境。
- 对照 `week4/*/src/main.rs`，看感知工具、执行工具、协作工具和事件触发 Agent 如何被拆成独立 Rust 入口。
- 后续如果要恢复真实 MCP/HTTP 服务，可以从这里补 Dockerfile、compose 文件和运行时权限策略。

## Rust 版关注点

- 先保持每个工具课程目录可独立编译。
- 再把外部依赖放进明确的服务边界。
- 对执行类工具保留安全审批、超时和日志脱敏设计。

## 建议练习

1. 运行 `cargo test -p agent-course-week4-execution-tools`。
2. 为执行工具添加一个新的风险等级。
3. 再考虑如何把该工具封装进容器运行。

