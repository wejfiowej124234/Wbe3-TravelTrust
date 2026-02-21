#!/usr/bin/env bash
# 08-3 与 08-4 一致性校验（W-PDP-SSOT-CONSISTENCY）
# 用法：在仓库根目录执行 scripts/check-08-consistency.sh [base_ref]
# 若未传 base_ref 则与 HEAD 比较（单次提交）；CI 中可传 main 或 $BASE_REF。
# 规则：若 docs/08-3 的「关键 key 与 08-4 章节映射」表中任一 key 被改动，则 docs/08-4 中「文档版本（CI 校验用）」行必须在本 diff 中有变更，否则 exit 1。

set -e
BASE="${1:-HEAD^}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# 无父提交时（如首次提交）跳过检查
if [ "$BASE" = "HEAD^" ] && ! git rev-parse HEAD^ 2>/dev/null; then
  echo "OK: 无父提交，跳过 08-3/08-4 一致性检查"
  exit 0
fi

# 08-3 映射表中的 param_key（与 08-3 文档「关键 key 与 08-4 章节映射」表一致；增删 key 须同步此处）
KEYS="ofacHitPolicy|pauseCooldown|pauseAllowlist|evidenceRetentionDays|evidenceMaxSize|evidenceTypeAllowlist|arbFeeBase|arbFeeCap|fxDisplayPolicy|paramChangeMaxPer30d|paramFreezeDays|freezeDisputePolicy|serviceStartTimeSource|minArbitratorCount|chargebackPolicy"

# 08-4 版本行标识（兼容全角/半角括号与空格，避免格式微调导致误判）
VERSION_MARKER="文档版本"

# 精确匹配 08-3/08-4 文件名，避免 pathspec 匹配不到实际文件
diff_08_3="$(git diff "$BASE" -- "docs/08-3-参数与门禁表.md" 2>/dev/null || true)"
diff_08_4="$(git diff "$BASE" -- "docs/08-4-对外口径包.md" 2>/dev/null || true)"

# 若未改 08-3，直接通过
if [ -z "$diff_08_3" ]; then
  echo "OK: docs/08-3 无变更，跳过 08-4 版本号检查"
  exit 0
fi

# 检查 08-3 的 diff 是否触及映射 key（在 26 key 表或映射表段落中出现的 key）
if ! echo "$diff_08_3" | grep -qE "$KEYS"; then
  echo "OK: docs/08-3 有变更但未触及映射表中的 param_key"
  exit 0
fi

# 触及映射 key：08-4 的「文档版本（CI 校验用）」行必须在本 PR 中有变更
if [ -z "$diff_08_4" ]; then
  echo "FAIL: 本次变更触及 08-3 映射表中的 key，但 docs/08-4 无变更。请在 08-4 文末更新「文档版本（CI 校验用）」行，或在本 PR 中同步修改 08-4 对应章节。"
  exit 1
fi
# 08-4 变更中须包含版本行（含「文档版本」及 CI 校验用标识，兼容全角/半角括号）
if ! echo "$diff_08_4" | grep -q "$VERSION_MARKER"; then
  echo "FAIL: 本次变更触及 08-3 映射表中的 key，但 docs/08-4 的变更中未包含「文档版本（CI 校验用）」行。请更新 08-4 文末该行（如 vYYYYMMDD）。"
  exit 1
fi
if ! echo "$diff_08_4" | grep -qE "CI.?校验用|v[0-9]"; then
  echo "FAIL: 08-4 变更中须包含文档版本（CI 校验用）行且含版本号（如 vYYYYMMDD）。"
  exit 1
fi

echo "OK: 08-3 映射 key 有变更且 08-4 版本行已同步"
exit 0
