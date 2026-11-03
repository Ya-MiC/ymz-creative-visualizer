name: 功能建議
description: 提出新功能或改進建議
title: "[Feature] "
labels: ["enhancement"]
body:
  - type: markdown
    attributes:
      value: "# 功能建議\n\n請在提交前確認已閱讀 AGENTS.md 中的不可妥協項。"
  - type: input
    id: description
    attributes:
      label: 功能描述
      description: 請詳細描述您希望新增的功能
    validations:
      required: true
  - type: input
    id: usecase
    attributes:
      label: 使用場景
      description: 此功能在什麼情境下會被使用
    validations:
      required: false
