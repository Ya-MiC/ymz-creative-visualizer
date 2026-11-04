# AGENT-BOOT.md · 你的創意視覺化呈現

## 產品一句話
本地優先、Markdown 驅動、像素風的 AI 視覺創作工作臺。用戶自帶 API，不代理、不抽成。

## 技術棧
- 桌面：Tauri 2 + Rust（Cargo workspace：crates/ymz-core + src-tauri 薄命令層）
- 前端：Vue 3 + Pinia + Vue Router + Vite + TypeScript
- 官網：純靜態頁，GitHub Pages / Cloudflare Pages
- 後端（階段 C）：Cloudflare Workers + D1

## 鐵律摘要
1. 顏色、字體、動效、圖示只取自 `src/styles/tokens.css` 與品牌視覺參考。
2. 私鑰、AppID、API Key 僅放 Cloudflare/GitHub Secrets，不進倉庫、前端、日誌、對話。
3. 所有改動在獨立分支，不推 main、不發 Release、不改倉庫 Description、不替用戶付款、不替用戶登入或簽約支付寶、不替用戶提交比賽作品、不發公開貼文。
4. 絕對不要買星、互刷、建立假帳號或任何刷量行為。
5. 依賴只收寬鬆授權（MIT、Apache-2.0、BSD、ISC）。GPL/AGPL/專有只能參考設計，不可引入程式碼。
6. 外部 API 端點、參數、狀態值必須來自官方文檔或真實回應；查不到寫 `TODO(verify)`，然後問用戶。
7. 不使用逆向、破解或違反服務條款的第三方介面。
8. 每一步做完，把指令與輸出原文貼給用戶；輸出超過 40 行寫到 `logs/` 檔案，回報只貼路徑。
9. 觸控或點擊熱區至少 44px；尊重 `prefers-reduced-motion`；錯誤訊息清楚；「已複製」類提示約 2.5 秒後自動隱藏；介面支援繁中、簡中、英文，全程 UTF-8。
10. 不在電腦前時，只做可逆、不對外的事，並把狀態寫進 `STATE.md`。
11. 不編造數字、星數、價格、測試結果。

## 目錄地圖
- `crates/ymz-core/`：純 Rust 函式庫（project、frontmatter、generated_block、atomic_write、snapshot、diff、wikilinks、search、providers、jobs、media、uninstall）
- `src-tauri/`：薄薄的 Tauri 命令，呼叫 ymz-core，不寫業務邏輯
- `src/features/{workspace,editor,providers,jobs,compose,settings}`：前端功能模組
- `src/components/`：通用元件
- `src/types/`：TypeScript 型別集中
- `src/styles/tokens.css`：設計系統原色（不可更動）
- `formats/PROJECT-SCHEMA.md`：專案格式規格
- `docs/agent/`：代理記憶檔案（本檔、STATE.md、DECISIONS.md、DOGFOOD.md、skill-digest.md）

## 如何跑建置與測試
```bash
# 前端
pnpm install --frozen-lockfile
pnpm exec vue-tsc --noEmit
pnpm exec vite build

# Rust（完整 workspace，非 --lib）
cd src-tauri  # 或 crates/ymz-core
cargo test
cargo check
```

## 如何回報
格式：已讀 / 已改 / 執行與輸出（最後 40 行 + 錯誤） / 結果 / 沒做到 / 需要我決定或操作。
每個任務開始前說「我這次只做：…」，結束時說「我沒有做：…」。
