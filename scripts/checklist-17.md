# 17 条验收清单（01 §10 审计验收表）

实现/发版前逐条勾选：每条须有单测、E2E 或 runbook 可产出对应 Evidence artifact。详见 [01-总库总览 §10 审计验收表](../docs/01-总库总览.md)。

| # | Invariant / Rule | Evidence artifact | □ 已覆盖 |
|---|------------------|-------------------|----------|
| 1 | 价值守恒：终态 payout+refund+slash+platformFee(+arbitrationFee)==grossAmount | invariant_test_results.json；contract_bytecode_sha256 | |
| 2 | Paid 仅来自 deposit() | 链上事件日志；e2e_payment_via_deposit.png | |
| 3 | topUp/重复 Paid 规则、DB 合并规则写死 | projection_rebuild_test.log；paid_merge_rule | |
| 4 | orderId↔escrow 一对一 | reconciliation_order_escrow.csv | |
| 5 | participants 不可变、无零地址 | Slither 报告；部署参数记录 | |
| 6 | Paid 事件可重建结算 | event_schema_assertion.json；rebuild_from_events_test.log | |
| 7 | PartiallyRefunded/Slashed 合法组合集 | resolution_amount_rules.json；执行器单测 | |
| 8 | token 异常后承诺（黑名单/冻结） | runbook_token_frozen.md；allowed_tokens_hash | |
| 9 | Dispute 与自动放款互斥 | DisputeOpened 事件；config 校验 disputeDeadline≥autoCompleteAt | |
| 10 | executeResolution 绑定实例+守恒 | resolution_approval_*.json；合约测试 | |
| 11 | 裁决签名含 RBAC 快照、双人审批 | evidence_pack_manifest_sha256；审计表 | |
| 12 | DB 状态仅链事件驱动 | correction_log；reconciliation_report_*.json | |
| 13 | reorg 撤销与投影回退 | reorg_handling_test.log；checkpoint | |
| 14 | 幂等键覆盖四类来源（API/队列/执行器/连点） | API 日志 idempotency_key；executed[resolutionId] | |
| 15 | 对账三段式触发条件 | reconciliation_rules.json；对账运行日志 | |
| 16 | 状态机与副作用联动 | state_machine_side_effects.json；通知/档期审计日志 | |
| 17 | finalityN 与配置单一来源 | config.toml+env hash；replay_after_finality_change.log | |

**用法**：实现时每行「□ 已覆盖」打勾并填写 artifact 路径或编号；发版前 17 行均须已勾选。
