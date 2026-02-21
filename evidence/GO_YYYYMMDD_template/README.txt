本目录为 Gate 证据 bundle 的模板，非真实证据。
首次过门时：
  1. 复制本目录并重命名为 GO_YYYYMMDD（如 GO_20250220）
  2. 在新区目录中编辑 manifest.json：填写 gate、date、artifacts、sign_off
  3. 生成 manifest.sha256：在 manifest.json 所在目录执行
     - Linux/macOS: sha256sum manifest.json > manifest.sha256
     - 或: 将 manifest.json 的 SHA256 值写入 manifest.sha256 文件
  4. 在 08-2 对应工单的 Evidence 列填写 evidence/GO_YYYYMMDD/ 或 manifest hash

详见 evidence/README.md。
