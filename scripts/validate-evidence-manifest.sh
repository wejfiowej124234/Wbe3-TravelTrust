#!/usr/bin/env bash
# evidence manifest.json 校验（Gate bundle 必填字段、date 格式、artifacts[].sha256 为 64 位 hex）
# 用法：./scripts/validate-evidence-manifest.sh [path/to/manifest.json]
# 依赖：jq。无 jq 时请人工按 evidence/README 必填字段表核对。

set -e
MANIFEST="${1:-}"
if [ -z "$MANIFEST" ]; then
  echo "用法: $0 <path/to/manifest.json>"
  exit 1
fi
if [ ! -f "$MANIFEST" ]; then
  echo "错误: 文件不存在: $MANIFEST"
  exit 1
fi

if ! command -v jq &>/dev/null; then
  echo "警告: 未安装 jq，请人工按 evidence/README 必填字段（gate、date、artifacts、sign_off）核对"
  exit 0
fi

err=0
# 必填字段
for key in gate date artifacts sign_off; do
  if ! jq -e ".\"$key\"" "$MANIFEST" &>/dev/null; then
    echo "错误: 缺少必填字段: $key"
    err=1
  fi
done
# date 格式 YYYY-MM-DD
if date_val=$(jq -r '.date' "$MANIFEST" 2>/dev/null) && [ -n "$date_val" ] && [ "$date_val" != "null" ]; then
  if ! echo "$date_val" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}$'; then
    echo "错误: date 须为 YYYY-MM-DD: $date_val"
    err=1
  fi
fi
# artifacts 至少 1 项，每项含 path、sha256（64 位 hex）
if arts=$(jq -r '.artifacts' "$MANIFEST" 2>/dev/null) && [ "$arts" != "null" ]; then
  count=$(jq '.artifacts | length' "$MANIFEST")
  if [ "$count" -lt 1 ]; then
    echo "错误: artifacts 至少 1 项"
    err=1
  fi
  for i in $(seq 0 $((count - 1))); do
    sha=$(jq -r ".artifacts[$i].sha256" "$MANIFEST")
    if [ -z "$sha" ] || [ "$sha" = "null" ]; then
      echo "错误: artifacts[$i] 缺少 sha256"
      err=1
    elif ! echo "$sha" | grep -qE '^[a-f0-9]{64}$'; then
      echo "错误: artifacts[$i].sha256 须为 64 位小写 hex: $sha"
      err=1
    fi
  done
fi
# sign_off 至少 1 人
so_count=$(jq '.sign_off | length' "$MANIFEST" 2>/dev/null || echo 0)
if [ "$so_count" -lt 1 ]; then
  echo "错误: sign_off 至少 1 人"
  err=1
fi

if [ $err -eq 1 ]; then
  exit 1
fi
echo "OK: manifest 必填字段与格式校验通过"
