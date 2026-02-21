# TravelTrust

> **去中心化协议**：基于区块链的全球导游信誉与托管层 — 稳定币支付、质押准入、链上可验证评分与仲裁。资金与状态链上可验证，规则透明、可审计，治理可演进。  
> **Disclaimer**：本仓库为开源技术项目，不构成投资或法律建议，不招揽投资、不承诺任何收益。

## 愿景

- **支付**：稳定币（如 USDC）订单托管；若采用平台代币则仅用于手续费折扣、质押与治理，不涉及募资或收益承诺。
- **信誉**：仅链上/系统内「真实成交」可评价，权重与订单金额与历史挂钩，防刷单。
- **履约**：导游质押准入，游客可选押金；争议走证据 + 仲裁，违约扣罚质押。

## 技术栈（全 Rust）

| 层级     | 技术选型              | 说明 |
|----------|-----------------------|------|
| 后端 API | Axum + Tower          | 异步 HTTP、中间件、与前端/链解耦 |
| 数据     | SQLx + 关系型/分布式 DB | PostgreSQL 或 CockroachDB/TiDB 等，见 [总库总览](docs/01-总库总览.md) |
| 前端     | Yew (Rust → WASM)     | 游客端 / 导游端 / 仲裁后台 |
| 共享逻辑 | `traveltrust-core`   | 领域类型、校验、与链交互抽象 |
| 区块链   | EVM（以太坊 + L2）    | 智能合约 Solidity，链下 Rust；稳定币 USDC/USDT/DAI，L2 如 Arbitrum/Base 降低 gas，见 [总库总览](docs/01-总库总览.md) |

## 数据形态与 DApp

- **协议定位**：**去中心化协议**；资金与关键状态链上可验证，治理多签+TimeLock，运营层可演进（详见 [总库总览](docs/01-总库总览.md)）。
- **技术形态**：链上做托管(Escrow)、质押(Staking)、信誉存证；数据库做用户/导游/订单/争议等业务与检索。
- **智能合约**：Escrow、Staking 必须上链；Reputation 存证建议上链；Dispute 执行可选上链。
- **UI**：全 Rust（Yew/WASM）；同一前端支持 **Web 版** 与 **DApp 版**（连接钱包后签名支付与质押）。
- **钱包**：不自研钱包，通过标准接口（如 WalletConnect、注入 provider）兼容常见钱包。

详见 [总库总览](docs/01-总库总览.md)。

## 仓库结构

以下为**目标/规划结构**；当前实现状态以本页「实现状态」为准，详见 [08-0 §十二](docs/08-0-系列审计-命名排序与合并瘦身.md)（如 chain-client 为可选、未入 workspace；api 子目录随实现补齐）。

```
Wbe3-TravelTrust/
├── crates/
│   ├── core/          # 领域模型、Escrow/Staking/Reputation 抽象
│   ├── api/           # Axum 后端、REST、DB、链客户端封装
│   ├── web/           # Yew 前端 (游客/导游/仲裁) + DApp 钱包连接
│   └── chain-client/  # 可选：与链交互的共用封装（当前未入 workspace）
├── contracts/         # 智能合约：Escrow、Staking、Registry（01/02 方案 B）；Reputation 存证可选；设计见 contracts/README.md
├── docs/              # 技术文档（00～08 按序号，含 08 合规门禁）
├── ops/               # Runbook、演练模板、值班/批准链
├── evidence/          # Gate 通过证据 bundle（GO_YYYYMMDD/）、模板见 GO_YYYYMMDD_template/
├── scripts/           # 08-3/08-4 一致性、部署前 SSOT、evidence 校验、前端 manifest、17 条清单（见 08-5、00 目录结构）
├── .github/           # PR 模板、CI workflow（08 门禁校验 + 构建）
└── README.md
```

## 实现状态（与 docs/04、08-0 §十二 对齐）

- **api**：04 §三 全量路由已挂（含可选 GET /api/v1/me/stats），x-request-id（traceId）、请求超时（30s）、请求体限制（1MB）、鉴权占位中间件、STRICT_SSOT 可选校验已就位；幂等键（Idempotency-Key/X-Idempotency-Key）透传与响应回写已就位，实现时在此做 key 去重与结果复用（01 §10 #14）。**生产环境必须设置 CORS_ORIGINS**（未设则开发态允许任意 origin）。
- **web**：yew-router 路由（/、/auth、/me、/orders、/disputes）、api 模块（get_health、get_me）、首页 loading/error 示例已就位；Phase 4 按 05/06 落地业务与 dapp 钱包。
- **contracts**：README 与设计锚点、实现时技术约束说明已建（01/02/08-4）；Solidity 实现待落。

## 核心模块（对应你之前讨论的清单）

| 模块        | 职责 |
|-------------|------|
| **Registry** | 导游/用户注册、质押门槛、城市·语言·服务类型 |
| **Escrow**   | 下单锁定金额、完成放款、争议时冻结 |
| **Staking**  | 导游质押准入、违约扣罚、仲裁员质押 |
| **Reputation** | 仅完成订单可评、权重评分、可选 Web2 信誉导入 |
| **DisputeCourt** | 证据提交、仲裁员裁决、自动执行退款/扣罚 |

## 快速开始

```bash
# 安装 Rust 工具链
# https://rustup.rs

# 后端（可选：复制 .env.example 为 .env 并设置 SSOT_VERSION、PORT、CORS_ORIGINS 等，见 Runbook §10；生产须设置 CORS_ORIGINS）
cd crates/api && cargo run

# 前端
cd crates/web && trunk serve
```

## 合规与风控（设计原则）

- 本仓库为**开源技术文档与代码**，仅供学习与参考，**不构成任何投资、法律或合规建议**，不招揽投资、不承诺回报。
- 导游资质：按国家/地区预留「持证/合规」字段，不默许无证接单。
- 代币设计：若存在平台代币则仅用于手续费折扣、质押与治理，**不涉及募资、不承诺任何收益**；支付以稳定币为主。
- KYC/AML：跨境托管与导游侧预留 KYC 接口，具体合规要求以当地法规为准。

## 文档

- **[00-文档索引](docs/00-文档索引.md)**：文档列表与阅读顺序（按企业级顺序排列）
- **[01-总库总览](docs/01-总库总览.md)**：业务流程、混合架构、链选型（EVM+L2）、钱包策略、仓库骨架
- **[02-架构设计](docs/02-架构设计.md)**：分层、子域、状态机、钱与链抽象、合约/模块对应
- **[03-业务流程与风控](docs/03-业务流程与风控.md)**：流程规则、公平评分与防恶意、争议人工裁决
- **[04-后端与API](docs/04-后端与API.md)**：数据表、API 路由 v1、风控、落地顺序
- **[05-前端总览](docs/05-前端总览.md)**：Yew 结构、页面/组件、api 与 dapp、代币支付落点
- **[06-DApp架构总览](docs/06-DApp架构总览.md)**：DApp 定位、架构简图、钱包/签名/合约交互
- **[07-开发流程与顺序](docs/07-开发流程与顺序.md)**：先确认的架构、企业级开发阶段与顺序（Phase 0～5）
- **08 合规与门禁**：[08-1 战略与合规风险检查清单](docs/08-1-战略与合规风险检查清单.md)、[08-2 闭合工单表](docs/08-2-附录-闭合工单表.md)、[08-3 参数与门禁表](docs/08-3-参数与门禁表.md)、[08-4 对外口径包](docs/08-4-对外口径包.md)；发版前须过 08-4 定稿勾选、[Runbook P0 最小必填项](ops/RUNBOOK.md)、08-2 [发版前审查一（语义一致性）](docs/08-2-附录-闭合工单表.md#发版前审查一关键语义一致性审查表) 与 [发版前审查二（Gate 冲突矩阵）](docs/08-2-附录-闭合工单表.md#发版前审查二gate-冲突矩阵与优先级规则)；P0 门禁、Gate-1～5、evidence 与 CI 见 [00-文档索引 §08 系列](docs/00-文档索引.md)、[08-5-CI与一致性落地说明](docs/08-5-CI与一致性落地说明.md)、[evidence/README](evidence/README.md)。**企业级审计结论**（文档缺口/技术缺口/风险与落点）见 [08-0 §十一～§十四](docs/08-0-系列审计-命名排序与合并瘦身.md)。**全方位文档与缺口检查**见 [多维度文档与技术检查报告](docs/多维度文档与技术检查报告.md)。

## License

MIT
