# 貢獻指南

歡迎貢獻！請先閱讀 AGENTS.md 中的不可妥協項。

## 如何貢獻

1. Fork 本倉庫
2. 建立獨立分支（`git checkout -b feature/xxx`）
3. 提交變更（`git commit -m "feat: xxx"`）
4. 推送分支（`git push origin feature/xxx`）
5. 開啟 Pull Request

## 代碼規範

- 顏色、字體、動效、圖示只取自 `src/styles/tokens.css` 與品牌視覺參考。
- 依賴只收寬鬆授權（MIT、Apache-2.0、BSD、ISC）。
- 每個 PR 必須通過 CI（pnpm build、vue-tsc、cargo check）。
