# 你的創意視覺化呈現

Local-first, Markdown-driven pixel-style AI video creation studio. Bring your own API. Vue 3 + Tauri 2 + Rust.

| | |
|---|---|
| 官網 | 待建置 |
| 下載 | 待 Releases 發布 |
| 源碼 | https://github.com/Ya-MiC/ymz-creative-visualizer |

## 產品定位

本地優先、Markdown 驅動、像素風的 AI 視覺創作工作臺。用戶在自己電腦的資料夾裡寫 Markdown（故事、角色、分鏡），AI 協助生成提示詞，用戶自帶任意圖像或視頻 API（OpenRouter、MiniMax、硅基流動、自訂端點），最後在本地用 FFmpeg 合成視頻。

平臺沒有發布、分享、評論、社群、即時通訊功能。

## 商業規則

- 軟體本體免費，源碼 MIT。
- 付費只收「平臺服務」（由支付寶 ¥0.01 購買，內容待定義）。
- 用戶自帶 API，平臺不代理、不抽成。

## 隱私與資料邊界

- 軟體僅在本機掃描您指定的資料夾，不上傳任何使用者資料。
- 僅「登入、下單、確認付款、領授權」時會連網。
- 卸載乾淨，您的資料不動。

## 開發者

```bash
git clone https://github.com/Ya-MiC/ymz-creative-visualizer.git
cd ymz-creative-visualizer
pnpm install
pnpm tauri dev
pnpm tauri build
```

## License

MIT © 晏銘湛箴 / Ya-MiC
