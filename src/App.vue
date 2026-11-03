<script setup lang="ts">
import { ref } from 'vue'

const view = ref<'home' | 'project' | 'editor' | 'export'>('home')
</script>

<template>
  <div class="shell">
    <header class="topbar">
      <div class="brand">
        <span class="logo">◈</span>
        <div>
          <h1>你的創意視覺化呈現</h1>
          <p class="sub">晏銘湛箴 · 本地優先 · Markdown 驅動 · 像素風 AI 視頻創作</p>
        </div>
      </div>
      <div class="actions">
        <button class="btn ghost" @click="view = 'home'">首頁</button>
        <button class="btn" @click="view = 'project'">專案</button>
      </div>
    </header>

    <main class="view" v-if="view === 'home'">
      <h2>歡迎使用</h2>
      <p>本地優先、Markdown 驅動的像素風 AI 視頻創作工作臺。</p>
      <p>您的 .md 與素材永遠儲存在本機，不上傳。</p>
    </main>

    <main class="view" v-else>
      <p>此分頁尚在建設中，敬請期待。</p>
    </main>
  </div>
</template>

<style>
.shell { font-family: var(--font); background: var(--bg); color: var(--ink); }
.topbar { display: flex; justify-content: space-between; align-items: center; padding: 16px; border-bottom: 1px solid var(--line); background: var(--surface); }
.brand { display: flex; gap: 10px; align-items: center; }
.logo { width: 36px; height: 36px; border-radius: var(--radius); display: grid; place-items: center; background: var(--raised); color: var(--accent); font-size: 24px; }
.actions { display: flex; gap: 8px; }
.btn { min-height: var(--hit); padding: 0 16px; border-radius: 999px; border: none; font-weight: 650; cursor: pointer; }
.btn.ghost { background: transparent; color: var(--ink); border: 1px solid var(--line); }
.view { padding: 24px; }
</style>
