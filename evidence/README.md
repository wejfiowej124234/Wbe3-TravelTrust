# Evidence 目录（Gate 通过证据 bundle）

本目录存放 **Gate 通过** 与 **演练** 的取证级证据 bundle，与 [08-1-战略与合规风险检查清单](../docs/08-1-战略与合规风险检查清单.md)、[08-2-附录-闭合工单表](../docs/08-2-附录-闭合工单表.md) 配套。08 定稿与闭合标准见 [08-4](../docs/08-4-对外口径包.md)、[08-3](../docs/08-3-参数与门禁表.md) 开篇「审计闭合标准」及 [08-2 定稿前检查](../docs/08-2-附录-闭合工单表.md)。

## 目录约定

- **evidence/GO_YYYYMMDD/** — 某次 Gate 通过（或发版前五门全过）的 evidence bundle。
  - 内含：`manifest.json`（产物清单）、`manifest.sha256`（校验）、截图/日志索引等。
  - 工单表 **Evidence** 列可贴：`evidence/GO_20250220/` 或 manifest 的 hash。
- **evidence/GO_YYYYMMDD_template/** — **可复制模板**：首次过门时复制为本目录并重命名为 GO_YYYYMMDD，再填写 manifest。勿在此目录内放真实证据。
- **evidence/GO_placeholder/** — **仅占位说明**（非 bundle、非模板）：说明真实 bundle 用 GO_YYYYMMDD 目录。
- **evidence/DR-YYYYQX-0N/** — 单次演练（Runbook 演练）产物，可选按演练编号建子目录。

**首次过门时**：① 复制 **evidence/GO_YYYYMMDD_template/** 为 `evidence/GO_YYYYMMDD/`（如 GO_20250220）② 在新区目录内编辑 `manifest.json`（填 gate、date、artifacts、sign_off）③ 生成 `manifest.sha256`（见模板内 README.txt）④ 在 08-2 对应工单 Evidence 列填写该路径或 manifest hash。无 bundle 时 Gate 不视为闭合。

## manifest 格式与必填字段（SSOT）

**必填字段**（缺一则该 bundle 不得作为门禁证据）：

| 字段 | 类型 | 说明 |
|------|------|------|
| gate | string | 如 Gate-1～Gate-5；须与 08-2 Gate 汇总一致 |
| date | string | YYYY-MM-DD，过门或发版日期 |
| artifacts | array | 至少 1 项；每项含 path、sha256（小写 hex） |
| sign_off | array | 至少 1 人；角色或代号 |

**示例**：

```json
{
  "gate": "Gate-1",
  "date": "2025-02-20",
  "artifacts": [
    { "path": "SSOT-PARAMS-v1.pdf", "sha256": "..." },
    { "path": "PDP-ch1-8-signed.pdf", "sha256": "..." }
  ],
  "sign_off": [ "法务", "运维" ]
}
```

生成后计算 `sha256 manifest.json`（或 `sha256sum manifest.json`）写入 `manifest.sha256`，便于验证未被篡改。

**校验**：定稿或过门时建议对 manifest 做一次校验（必填字段存在、date 格式、artifacts[].sha256 为 64 位 hex）。可执行 **scripts/validate-evidence-manifest.sh [path/to/manifest.json]**（依赖 jq）；无 jq 时人工按上表核对，落 08-2 定稿前检查。

**Gate 通过检查**（满足后该 bundle 方可作为门禁证据）：□ manifest 含 `gate`、`date`、`artifacts`、`sign_off` □ 08-4 已定稿时，manifest 内引用版本号与 08-4 文末版本一致 □ 工单 Evidence 列已贴本目录路径或 manifest hash。

*勿提交敏感内容（密钥、未脱敏 PII）；仅路径与 hash、脱敏清单可入仓。*

**缺口说明**：真实 `evidence/GO_YYYYMMDD/` bundle 须在过门时按上文「首次过门时」四步产出并入仓；仓内不代造真实证据。除该人工步骤外，无其他可于仓内补齐的 evidence 缺口。
