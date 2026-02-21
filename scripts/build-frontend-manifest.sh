#!/usr/bin/env bash
# 前端可验证发布：在 frontend 构建产物目录生成 manifest.json 与 manifest.sha256（08-4 第 7 章、W-Q6-FE）
# 用法：前端构建完成后在仓库根执行，默认扫描 dist/；可传目录： scripts/build-frontend-manifest.sh [dist_dir]
# 示例：cd crates/web && trunk build && cd ../.. && scripts/build-frontend-manifest.sh dist

set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${1:-dist}"
cd "$ROOT"

if [ ! -d "$DIST_DIR" ]; then
  echo "SKIP: 目录 $DIST_DIR 不存在；请先完成前端构建（如 trunk build 产出 dist/）再运行本脚本。"
  exit 0
fi

MANIFEST_JSON="${DIST_DIR}/manifest.json"
MANIFEST_SHA="${DIST_DIR}/manifest.sha256"

# 收集产物：path 与 sha256（需 jq 生成多文件列表；无 jq 时写空 artifacts）
ARTIFACTS="[]"
if command -v jq >/dev/null 2>&1; then
  if command -v sha256sum >/dev/null 2>&1; then
    while IFS= read -r -d '' f; do
      rel="${f#$DIST_DIR/}"
      [ "$rel" = "manifest.json" ] || [ "$rel" = "manifest.sha256" ] && continue
      h="$(sha256sum "$f" | cut -d' ' -f1)"
      ARTIFACTS="$(echo "$ARTIFACTS" | jq --arg p "$rel" --arg s "$h" '. + [{path:$p, sha256:$s}]')"
    done < <(find "$DIST_DIR" -type f -print0 2>/dev/null)
  else
    while IFS= read -r -d '' f; do
      rel="${f#$DIST_DIR/}"
      [ "$rel" = "manifest.json" ] || [ "$rel" = "manifest.sha256" ] && continue
      ARTIFACTS="$(echo "$ARTIFACTS" | jq --arg p "$rel" '. + [{path:$p, sha256:""}]')"
    done < <(find "$DIST_DIR" -type f -print0 2>/dev/null)
  fi
fi
echo "{\"gate\":\"Q6-frontend\",\"date\":\"$(date +%Y-%m-%d)\",\"artifacts\":$ARTIFACTS,\"sign_off\":[\"build\"]}" > "$MANIFEST_JSON"

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$MANIFEST_JSON" | cut -d' ' -f1 > "$MANIFEST_SHA"
else
  echo "（无 sha256sum，请人工生成 manifest.sha256）" > "$MANIFEST_SHA"
fi
echo "OK: 已生成 $MANIFEST_JSON 与 $MANIFEST_SHA"
