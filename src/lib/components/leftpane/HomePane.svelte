<script lang="ts">
  import { init, locale, register, t, waitLocale } from 'svelte-i18n'
  import { onMount } from 'svelte'
  import { getConfig } from '$lib/api'
  import { configStore } from '../../../stores'
  import type { Config } from '$lib/config' // i18nライブラリをインポート
  export let thumbnailKeys: string[]

  // パスの省略処理
  function truncatePath(path: string, maxLength: number): string {
    if (path.length <= maxLength) {
      return path // 省略不要
    }

    const keepLength = Math.floor(maxLength / 2) - 2 // 先頭/末尾に残す文字数調整
    const start = path.slice(0, keepLength) // パスの先頭部分
    const end = path.slice(-keepLength) // パスの末尾部分
    return `${start}...${end}` // 先頭+...+末尾
  }
  let config: Config | null = null
  $: configStore.subscribe((value) => {
    config = value
  })
  let isLocaleReady: boolean = false
  let isConfigReady: boolean = false
  async function initializeI18n() {
    register('en', () => import('$lib/locale/leftpane/home/en.json'))
    register('ja', () => import('$lib/locale/leftpane/home/ja.json'))
    init({
      fallbackLocale: 'en',
      initialLocale: 'ja',
    })
    await waitLocale() // 初期化が完了するまで待機
    isLocaleReady = true
  }
  onMount(async () => {
    await initializeI18n()
    config = await getConfig()
    isConfigReady = true
    locale.set(config.feature_flags.language)
  })
</script>

{#if isLocaleReady && isConfigReady}
  <h2>
    <a href="#thumbnails" style="color:inherit;text-decoration: none"
      >{$t('homePane.title')}</a
    >
  </h2>
  <h3>{$t('homePane.folderList')}</h3>

  {#if thumbnailKeys.length > 0}
    <ul class="folder-list">
      {#each thumbnailKeys as key}
        <li class="folder-item">
          <!-- ツールチップで元のパスを表示 -->
          <a class="folder-path" title={key} href="#{key}">
            {truncatePath(key, 30)}
            <!-- 最大30文字までに省略 -->
          </a>
        </li>
      {/each}
    </ul>
  {:else}
    <p>{$t('homePane.no_folders')}</p>
  {/if}
{/if}

<style>
  /* リスト全体のデザイン */
  .folder-list {
    list-style: none; /* デフォルトのリストマーカーを削除 */
    padding: 0; /* パディング無し */
    margin: 0; /* マージン無し */
  }

  .folder-item {
    padding: 0.5rem 1rem; /* アイテムにスペースを追加 */
    border-bottom: 1px solid #ddd; /* 下線 */
    cursor: auto; /* マウスオーバー時のカーソル変更 */
    transition: background-color 0.3s ease; /* 背景色にアニメーションを適用 */
  }

  .folder-item:hover {
    background-color: #f0f0f0; /* ホバー時の背景色 */
  }

  /* パスのデザイン */
  .folder-path {
    font-size: 0.9rem; /* フォントサイズ調整 */
    color: #444; /* テキストの色 */
    text-decoration: none;
    display: block;
  }

  /* ツールチップ用 */
  .folder-path[title] {
    cursor: pointer;
  }

  /* ステータス */
  p {
    color: #777;
    font-size: 0.9rem;
    overflow: hidden;
    white-space: nowrap;
  }
</style>
