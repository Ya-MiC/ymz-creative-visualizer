# skill-digest.md · 關鍵規則壓縮

細節在 skills 目錄的哪份檔案：
- `ymz-creative-visualizer/SKILL.md`：產品定位、五條護城河、強制工作方式、參考文件地圖、絕對紅線。
- `ymz-creative-visualizer/references/00-operating-rules.md`：每個任務的固定循環（讀→說→做→驗→報）、回報格式、錯誤處理、禁止事項、語言。
- `ymz-creative-visualizer/references/01-links.md`：全部網址（A 類已查證、B 類知名官方入口）。
- `ymz-creative-visualizer/references/02-project-format.md`：本地專案資料夾與 Markdown 格式。
- `ymz-creative-visualizer/references/03-provider-system.md`：API / 模型接入（核心）。
- `ymz-fullstack-playbook/SKILL.md`：全棧實戰手冊，給能力有限的代理使用。
- `ymz-fullstack-playbook/references/00-how-to-work.md`：工作方式（給 Nemotron）。
- `ymz-fullstack-playbook/references/01-links.md`：支付寶、打包發布、管理面板模板網址。
- `ymz-fullstack-playbook/references/02-stack-decisions.md`：技術選型已定，不要重新討論。

## 關鍵規則（平常只讀這份）
- 軟體本體免費、MIT；收費只收平台服務，用戶自帶 API 不抽成。
- 平臺沒有發布、分享、評論、社群、即時通訊功能。
- 顏色、字體、動效、圖示只取自 `src/styles/tokens.css` 與品牌視覺參考。
- 私鑰、AppID、API Key 只放 Cloudflare/GitHub Secrets。
- 所有改動在獨立分支，不推 main、不發 Release、不改倉庫 Description。
- 依賴只收寬鬆授權（MIT、Apache-2.0、BSD、ISC）。
- 外部 API 參數必須來自官方文檔，查不到寫 `TODO(verify)`。
- 每一步做完，把指令與輸出原文貼給用戶。
- 不在電腦前時，只做可逆、不對外的事，並把狀態寫進 `STATE.md`。
