# TravelTrust 智能合约（待实现）

本目录为 **Escrow、Staking、Registry（方案 B）** 及可选 **Reputation 存证** 的合约落点，与 [01-总库总览](../docs/01-总库总览.md) §4/§5、[02-架构设计](../docs/02-架构设计.md) §十、[08-4 对外口径包](../docs/08-4-对外口径包.md)、[Runbook §7](../ops/RUNBOOK.md) 一致。

## 设计承诺（定稿须与实现一致）

- **Escrow**：Factory + 每单 clone/槽位；锁代币、确认放款、争议按裁决退/扣；**无 admin 后门、无 emergency withdraw**；订单↔escrow 以链上事件为准。
- **Staking**：导游/仲裁员质押、扣罚、解押；档位与接单上限见 08-3（stakeTierThresholds / stakeToOrderCapMap）。
- **Registry（方案 B）**：链上质押 + 链下审核 + 链上发资格；可接单条件见 01 §4、§10。
- **Reputation**：可选存证；与 01 可验证信誉一致。

## 实现状态

| 模块       | 状态     | 说明 |
|------------|----------|------|
| Escrow     | 待实现   | Solidity，与 01 §5、02 §十、08-4 无 admin/无 emergency 一致 |
| Staking    | 待实现   | Solidity，08-3 stakeTierThresholds |
| Registry   | 待实现   | 方案 B：链上发资格 |
| Reputation | 可选待实现 | 存证 |

合约实现后可置于本目录（如 `contracts/src/`）或独立 repo；若独立 repo，须在本 README 或 02 §十 注明路径，并产出不可逆结构图与 08-4 承诺证明入 evidence。

## 实现时技术约束

定稿与实现时须写明，便于复现与审计：

- **Solidity 版本**：编译器版本（如 0.8.19+）与 EVM 版本
- **目标网络**：如 Polygon PoS chainId 137（见 01 §七 MVP 链级最终决策块）
- **依赖**：链下交互用 alloy 或 ethers 的版本；测试/部署脚本环境
- **构建与验证**：solc 或 Foundry/Hardhat 版本；可复现构建步骤

## 参考

- 01 §4 哪些需要智能合约、§5 链与代币选型
- 02 §十 功能与参考实例对照、合约与模块对应
- 08-4 协议终极边界声明与终局设计、Runbook §7 Immutable Core / 多签权限矩阵
