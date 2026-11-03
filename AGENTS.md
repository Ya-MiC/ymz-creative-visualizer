# AGENTS.md · 你的創意視覺化呈現

品牌：晏銘湛箴（Ya-MiC）。

## 不可妥協

1. 本地優先：不收集遙測、不上傳檔案內容。除了「登入、下單、確認付款、領授權」，軟體不連網傳用戶資料。
2. 授權乾淨：MIT；依賴只收寬鬆授權（MIT、Apache-2.0、BSD、ISC）。GPL/AGPL/專有授權只能參考設計，不可引入程式碼。
3. 色票原色不得改（`src/styles/tokens.css`）。需要新色時停下來問使用者。
4. 觸控熱區 ≥ 44px；尊重 `prefers-reduced-motion`。
5. 介面支援繁中、簡中、英文，全程 UTF-8。
6. 私鑰、AppID、API Key、郵件服務金鑰、簽名憑證只放 Cloudflare Secrets 或 GitHub Secrets。不進倉庫、不進前端、不進日誌、不貼在對話裡。
7. 所有改動在獨立分支。不推 main（除非使用者明確授權）、不按 Release 的 Publish、不改倉庫 Description。
8. 外部 API 端點、參數、狀態值必須來自官方文檔或真實回應；查不到就寫 `TODO(verify)`，然後問使用者。
9. 不使用逆向、破解或違反服務條款的第三方介面。
10. 每一步做完，把指令與輸出原文貼給使用者，不可以只說「完成了」。

## 技術棧

- 前端：Vue 3 + Vite + TypeScript
- 殼：Tauri 2 + Rust
- 像素引擎：Canvas 2D（`src/composables/usePixelEngine.ts`）
- 分鏡結構：`src/composables/useMvTimeline.ts`
- 本地檔案庫：`src/composables/useLocalVault.ts`
- 色票：`src/styles/tokens.css`

## 對代理的要求

- 用繁體中文回覆使用者。
- 不把 API key / 密碼寫進倉庫。
- 對外發布、改品牌承諾、改收款資訊前先問使用者。
