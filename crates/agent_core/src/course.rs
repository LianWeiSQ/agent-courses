#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Project {
    pub slug: &'static str,
    pub title: &'static str,
    pub outcome: &'static str,
    pub concepts: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Week {
    pub number: u8,
    pub title: &'static str,
    pub goal: &'static str,
    pub projects: &'static [Project],
}

pub fn course_manifest() -> &'static [Week] {
    &COURSE
}

pub fn find_week(id: &str) -> Option<&'static Week> {
    let id = id.trim().to_ascii_lowercase();
    let normalized = id.strip_prefix("week").unwrap_or(&id);
    normalized
        .parse::<u8>()
        .ok()
        .and_then(|number| COURSE.iter().find(|week| week.number == number))
}

const WEEK1: &[Project] = &[
    Project {
        slug: "learning-from-experience",
        title: "经验学习：RL vs LLM",
        outcome: "用寻宝游戏对比 Q-learning 与 LLM in-context learning 的样本效率。",
        concepts: &["强化学习", "上下文学习", "样本效率", "先验知识"],
    },
    Project {
        slug: "web-search-agent",
        title: "Web Search Agent",
        outcome: "把搜索能力封装成 Agent 可调用的感知工具。",
        concepts: &["网络搜索", "工具调用", "信息整合"],
    },
    Project {
        slug: "search-codegen",
        title: "搜索 + 代码分析 Agent",
        outcome: "组合搜索、推理和代码执行完成复杂分析。",
        concepts: &["代码生成", "沙箱执行", "原生工具"],
    },
    Project {
        slug: "context",
        title: "上下文消融实验",
        outcome: "系统性移除上下文组件，观察 Agent 行为退化。",
        concepts: &["上下文管理", "ReAct", "消融研究"],
    },
];

const WEEK2: &[Project] = &[
    Project {
        slug: "local-llm-serving",
        title: "本地 LLM 部署与工具调用",
        outcome: "理解本地模型、chat template、流式输出与工具调用。",
        concepts: &["模型部署", "工具调用", "流式响应"],
    },
    Project {
        slug: "attention-visualization",
        title: "注意力机制可视化",
        outcome: "观察 token 序列与注意力权重如何影响推理。",
        concepts: &["注意力", "Token 分析", "可视化"],
    },
    Project {
        slug: "kv-cache",
        title: "KV Cache 友好的上下文设计",
        outcome: "理解上下文顺序和缓存命中对成本与延迟的影响。",
        concepts: &["KV Cache", "延迟优化", "上下文布局"],
    },
    Project {
        slug: "context-compression",
        title: "上下文压缩",
        outcome: "用摘要、抽取和结构化信息减少 token 消耗。",
        concepts: &["压缩", "摘要", "信息密度"],
    },
    Project {
        slug: "prompt-engineering",
        title: "提示工程消融",
        outcome: "量化语气、结构、工具描述对任务完成率的影响。",
        concepts: &["提示工程", "A/B 测试", "评估"],
    },
    Project {
        slug: "system-hint",
        title: "系统提示优化",
        outcome: "通过系统层指令稳定 Agent 行为。",
        concepts: &["System Prompt", "行为约束", "策略提示"],
    },
    Project {
        slug: "user-memory-evaluation",
        title: "用户记忆评估",
        outcome: "评估记忆的准确性、相关性和覆盖度。",
        concepts: &["评估框架", "测试用例", "指标"],
    },
    Project {
        slug: "user-memory",
        title: "用户长期记忆",
        outcome: "让 Agent 记录偏好、事实和跨会话上下文。",
        concepts: &["长期记忆", "个性化", "用户建模"],
    },
    Project {
        slug: "log-sanitization",
        title: "日志脱敏",
        outcome: "保留可调试信息，同时保护敏感数据。",
        concepts: &["隐私", "日志处理", "安全"],
    },
];

const WEEK3: &[Project] = &[
    Project {
        slug: "dense-embedding",
        title: "稠密向量检索",
        outcome: "构建向量检索服务并理解 ANN 索引取舍。",
        concepts: &["Embedding", "向量检索", "ANN"],
    },
    Project {
        slug: "sparse-embedding",
        title: "BM25 稀疏检索",
        outcome: "从词频、倒排索引和 BM25 解释传统搜索。",
        concepts: &["BM25", "TF-IDF", "倒排索引"],
    },
    Project {
        slug: "retrieval-pipeline",
        title: "混合检索流水线",
        outcome: "融合稠密、稀疏和重排序获得更稳的召回。",
        concepts: &["混合检索", "重排序", "融合"],
    },
    Project {
        slug: "multimodal-agent",
        title: "多模态信息提取",
        outcome: "比较原生多模态、OCR 文本化和工具化分析。",
        concepts: &["多模态", "OCR", "视觉理解"],
    },
    Project {
        slug: "structured-index",
        title: "结构化索引",
        outcome: "理解 RAPTOR 与 GraphRAG 的层次和关系索引。",
        concepts: &["RAPTOR", "GraphRAG", "知识图谱"],
    },
    Project {
        slug: "agentic-rag",
        title: "Agentic RAG",
        outcome: "让 Agent 主导多轮查询、验证和补充检索。",
        concepts: &["Agentic RAG", "ReAct", "迭代检索"],
    },
    Project {
        slug: "agentic-rag-for-user-memory",
        title: "用户记忆 Agentic RAG",
        outcome: "用 Agentic RAG 管理跨会话记忆查询。",
        concepts: &["用户记忆", "对话索引", "跨会话检索"],
    },
    Project {
        slug: "contextual-retrieval",
        title: "上下文感知检索",
        outcome: "给 chunk 增加上下文锚点，减少分块后的语义丢失。",
        concepts: &["上下文增强", "语义锚定", "检索优化"],
    },
    Project {
        slug: "contextual-retrieval-for-user-memory",
        title: "上下文感知用户记忆",
        outcome: "把结构化事实卡和上下文检索组合成双层记忆。",
        concepts: &["双层记忆", "结构化事实", "主动服务"],
    },
    Project {
        slug: "structured-knowledge-extraction",
        title: "结构化知识提取",
        outcome: "从语料中抽取因素、规则和可复用决策逻辑。",
        concepts: &["知识发现", "因子分析", "结构化抽取"],
    },
    Project {
        slug: "gaia-experience",
        title: "从成功轨迹中学习",
        outcome: "把任务轨迹总结成可检索经验。",
        concepts: &["经验学习", "轨迹总结", "自我进化"],
    },
    Project {
        slug: "browser-use-rpa",
        title: "工作流录制与回放",
        outcome: "把重复浏览器操作封装成参数化工具。",
        concepts: &["RPA", "工具生成", "外部化学习"],
    },
];

const WEEK4: &[Project] = &[
    Project {
        slug: "perception-tools",
        title: "感知工具",
        outcome: "封装搜索、文件、公共数据和多模态解析能力。",
        concepts: &["MCP", "感知", "公共数据源"],
    },
    Project {
        slug: "execution-tools",
        title: "执行工具",
        outcome: "构建带安全机制的文件、代码和终端执行工具。",
        concepts: &["执行安全", "代码解释器", "审批"],
    },
    Project {
        slug: "collaboration-tools",
        title: "协作工具",
        outcome: "接入浏览器、人类审批、通知和定时任务。",
        concepts: &["HITL", "通知", "浏览器自动化"],
    },
    Project {
        slug: "agent-with-event-trigger",
        title: "事件触发 Agent",
        outcome: "用 HTTP/Webhook/定时器驱动 Agent 工作流。",
        concepts: &["事件驱动", "异步", "工具编排"],
    },
];

const WEEK5: &[Project] = &[Project {
    slug: "coding-agent",
    title: "生产级 Coding Agent",
    outcome: "实现文件、搜索、编辑、Shell、TODO 和多模型接入工具。",
    concepts: &["代码生成", "文件编辑", "项目工具", "Lint"],
}];

const WEEK6: &[Project] = &[
    Project {
        slug: "android-world",
        title: "Android World",
        outcome: "理解移动端 UI 自动化任务如何评估 Agent。",
        concepts: &["移动自动化", "UI 交互", "任务完成率"],
    },
    Project {
        slug: "elo-leaderboard",
        title: "ELO 排行榜",
        outcome: "用成对比较构建 Agent 相对能力榜单。",
        concepts: &["ELO", "评估", "排行榜"],
    },
];

const WEEK7: &[Project] = &[
    Project {
        slug: "AdaptThink",
        title: "自适应推理深度",
        outcome: "根据问题难度选择 Thinking 或 NoThinking。",
        concepts: &["自适应推理", "成本优化", "约束优化"],
    },
    Project {
        slug: "retool",
        title: "工具增强数学推理",
        outcome: "让模型学会调用代码沙箱辅助推理。",
        concepts: &["工具使用", "数学推理", "RL"],
    },
    Project {
        slug: "AWorld-train",
        title: "具身 Agent 训练",
        outcome: "在环境交互中收集反馈并学习策略。",
        concepts: &["具身智能", "经验学习", "环境交互"],
    },
    Project {
        slug: "Intuitor",
        title: "直觉推理训练",
        outcome: "训练快速判断能力，减少冗长思维链。",
        concepts: &["直觉推理", "快速决策", "蒸馏"],
    },
    Project {
        slug: "MultilingualReasoning",
        title: "多语言推理",
        outcome: "提升跨语言任务的推理稳定性。",
        concepts: &["多语言", "泛化", "推理"],
    },
    Project {
        slug: "SpatialReasoning",
        title: "空间推理",
        outcome: "处理位置、方向和几何关系。",
        concepts: &["空间关系", "几何", "推理"],
    },
    Project {
        slug: "SimpleVLA-RL",
        title: "视觉-语言-动作 RL",
        outcome: "用视觉和语言输入训练动作策略。",
        concepts: &["VLA", "多模态 RL", "机器人"],
    },
    Project {
        slug: "continued-pretraining",
        title: "持续预训练",
        outcome: "把领域知识注入已有模型。",
        concepts: &["领域适应", "预训练", "数据配方"],
    },
    Project {
        slug: "MiniMind-pretrain",
        title: "小模型预训练",
        outcome: "理解从数据到训练循环的完整链路。",
        concepts: &["预训练", "小模型", "训练流程"],
    },
    Project {
        slug: "prompt-distillation",
        title: "提示蒸馏",
        outcome: "把长提示策略沉淀进模型行为。",
        concepts: &["蒸馏", "提示优化", "参数化知识"],
    },
    Project {
        slug: "feedback-guided-sampling",
        title: "反馈引导采样",
        outcome: "用奖励或偏好信号引导生成。",
        concepts: &["反馈学习", "采样优化", "质量控制"],
    },
    Project {
        slug: "learn-from-observation",
        title: "观察学习",
        outcome: "从示范轨迹中学习行为模式。",
        concepts: &["观察学习", "模仿学习", "行为克隆"],
    },
    Project {
        slug: "sesame",
        title: "序列建模与评估",
        outcome: "把序列数据建模、训练和评估串起来。",
        concepts: &["序列建模", "评估", "优化"],
    },
    Project {
        slug: "orpheus",
        title: "音乐生成与理解",
        outcome: "探索音频/音乐生成类模型训练入口。",
        concepts: &["音频", "音乐生成", "创意 AI"],
    },
];

const WEEK8: &[Project] = &[Project {
    slug: "live-audio",
    title: "实时语音对话",
    outcome: "设计 ASR、LLM、TTS、WebSocket 的低延迟链路。",
    concepts: &["ASR", "TTS", "WebSocket", "低延迟"],
}];

const COURSE: &[Week] = &[
    Week {
        number: 1,
        title: "Agent 基础",
        goal: "建立 Agent = 模型 + 上下文 + 工具 的基础心智模型。",
        projects: WEEK1,
    },
    Week {
        number: 2,
        title: "上下文工程与优化",
        goal: "理解上下文如何影响 Agent 的能力、成本和稳定性。",
        projects: WEEK2,
    },
    Week {
        number: 3,
        title: "知识库与学习机制",
        goal: "把检索、记忆和经验学习接入 Agent。",
        projects: WEEK3,
    },
    Week {
        number: 4,
        title: "工具生态与系统集成",
        goal: "把感知、执行、协作工具组织成安全可控的系统。",
        projects: WEEK4,
    },
    Week {
        number: 5,
        title: "Coding Agent",
        goal: "实现可编辑真实代码库的工程型 Agent。",
        projects: WEEK5,
    },
    Week {
        number: 6,
        title: "Agent 评估基准",
        goal: "用任务、对战和排行榜评估 Agent 能力。",
        projects: WEEK6,
    },
    Week {
        number: 7,
        title: "模型后训练",
        goal: "理解 SFT、RL、蒸馏和持续预训练如何改变 Agent 行为。",
        projects: WEEK7,
    },
    Week {
        number: 8,
        title: "多模态交互",
        goal: "设计低延迟语音、多模态和实时交互链路。",
        projects: WEEK8,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_covers_all_weeks() {
        let weeks = course_manifest();
        assert_eq!(weeks.len(), 8);
        assert_eq!(weeks[0].projects.len(), 4);
        assert!(weeks.iter().all(|week| !week.projects.is_empty()));
    }

    #[test]
    fn finds_week_by_friendly_name() {
        assert_eq!(find_week("week1").unwrap().number, 1);
        assert_eq!(find_week("7").unwrap().title, "模型后训练");
        assert!(find_week("week99").is_none());
    }
}
