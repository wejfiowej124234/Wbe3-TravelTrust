# 推送到 GitHub 仓库

将本地代码与文档推送到：<https://github.com/wejfiowej124234/Wbe3-TravelTrust>

## 1. 在项目根目录执行

```bash
cd C:\Users\plant\Desktop\Wbe3-TravelTrust

# 若无远程或要改用上述仓库，添加/替换远程
git remote add origin https://github.com/wejfiowej124234/Wbe3-TravelTrust.git
# 若已有 origin，可改为：
# git remote set-url origin https://github.com/wejfiowej124234/Wbe3-TravelTrust.git

# 暂存所有变更（含 README 与 docs）
git add .

# 提交（若已有未提交变更）
git commit -m "docs and README: disclaimer and compliance wording"

# 推送到 main（若远端是 master 则改为 git push -u origin master）
git push -u origin main
```

## 2. 若推送被拒（例如远端已有历史）

- 先拉再推：`git pull origin main --rebase` 然后 `git push -u origin main`
- 或强制覆盖（慎用，会覆盖远端）：`git push -u origin main --force`

## 3. 确认未提交敏感内容

- `.env` 已在 `.gitignore`，不会被提交
- 勿提交密钥、密码、真实姓名/邮箱等；README 已加 Disclaimer，合规表述已收紧
