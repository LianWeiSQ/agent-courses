# 工作流录制与回放

> 课程路径：`week3/browser-use-rpa`

## 这节课要解决什么

把重复浏览器操作抽象为参数化工具，降低推理成本。

## 学完应该能说清楚

- 这个项目在 Agent = Model + Context + Tools 中属于哪一层。
- 课程示例里的核心状态、动作、工具或评估指标是什么。
- 如何用类型、结构体和小型测试把概念固定下来。

## 怎么运行

从仓库根目录运行：

```bash
cargo run -p agent-course-week3-browser-use-rpa
```

单独测试这一节：

```bash
cargo test -p agent-course-week3-browser-use-rpa
```

## 核心概念

- RPA
- 浏览器自动化
- 工具生成

## 实现说明

代码入口在 `src/main.rs`。说明文档按学习路径、运行方式、核心概念和后续扩展建议组织。

## 下一步可以怎么扩展

- 把当前 demo 中的模拟数据换成真实模型、真实检索或真实工具。
- 为关键状态转移增加更多单元测试。
- 如果需要接外部 API，把 provider 放到独立模块，不要混进课程主流程。
