name: Bug 回報
description: 建立 Bug 回報以幫助我們改進
title: "[Bug] "
labels: ["bug"]
body:
  - type: markdown
    attributes:
      value: "# Bug 回報\n\n請在提交前確認已閱讀 AGENTS.md 中的不可妥協項。"
  - type: input
    id: description
    attributes:
      label: 描述
      description: 請詳細描述 Bug 的現象與預期行為
    validations:
      required: true
  - type: input
    id: steps
    attributes:
      label: 重現步驟
      description: 請列出重現 Bug 的步驟
    validations:
      required: true
  - type: input
    id: environment
    attributes:
      label: 環境
      description: 作業系統、版本、Tauri 版本等
    validations:
      required: false
