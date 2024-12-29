<script lang="ts">
  import { page } from '$app/stores' // SvelteKitの現在のページ情報
  import { derived } from 'svelte/store'
  import { init, locale, register, t, waitLocale } from 'svelte-i18n'

  // ページごとに表示する左ペインのコンポーネントを定義
  import HomePane from './leftpane/HomePane.svelte'
  import SettingsPane from './leftpane/SettingsPane.svelte'
  import { configStore } from '../../stores'
  import { onMount, tick } from 'svelte'
  import { getConfig } from '$lib/api'
  import { statusStore } from '../../statusStore'
  import type { Config } from '$lib/config'
  //import AboutPane from "./leftPaneComponents/AboutPane.svelte";

  export let activePage: string
  let config: Config | null = null
  $: configStore.subscribe((value) => {
    config = value
  })
  let isLocaleReady: boolean = false
  let isConfigReady: boolean = false
  async function initializeI18n() {
    register('en', () => import('$lib/locale/layout/en.json'))
    register('ja', () => import('$lib/locale/layout/ja.json'))
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
    configStore.set(config)
    isConfigReady = true
    locale.set(config.feature_flags.language)
  })
  // ストアから現在のステートを取得
  let status = {
    message: '',
    progress: null as number | null,
    type: 'info',
    isVisible: true,
  }
  $: statusStore.subscribe(async (value) => {
    status = { message: '', progress: null, type: '', isVisible: true }
    await tick()
    await tick()
    status = value
    setTimeout(() => {
      status.type = ''
    }, 3000)
  })

  export let thumbnailKeys: string[] // キー一覧
  export let thumbnailProcessed: boolean // 処理完了状態
  // ルートのパスに応じて左ペインのコンポーネントを動的に変更
  const currentComponent = derived(page, ($page) => {
    switch ($page.route.id) {
      case '/':
        return HomePane
      case '/settings':
        return SettingsPane
      case '/about':
        return null
      default:
        return null
    }
  })
  // 左ペインの幅管理
  let leftPaneWidth = 250
  let isResizing = false // リサイズ中フラグ

  // ドラッグ開始
  function startResize(e: MouseEvent) {
    isResizing = true
    document.addEventListener('mousemove', handleResize)
    document.addEventListener('mouseup', stopResize)
  }

  // リサイズ中
  function handleResize(e: MouseEvent) {
    if (isResizing) {
      const newWidth = e.clientX // マウスの位置で左ペインの幅を調整
      leftPaneWidth = Math.max(
        250,
        Math.min(300, Math.min(newWidth, window.innerWidth))
      )
    }
  }

  // リサイズ終了
  function stopResize() {
    isResizing = false
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)
  }

  // フォルダ選択
  // export let folderPath: string | null = null; // 選択されたフォルダパス
  export let statusMessage: string // ステータスバーのメッセージ
  //
  // function handleFolderSelection(event: Event) {
  //     const input = event.target as HTMLInputElement;
  //     if (input.files && input.files.length > 0) {
  //         folderPath = input.files[0].webkitRelativePath.split('/')[0];
  //         statusMessage = `選択されたフォルダ: ${folderPath}`;
  //     } else {
  //         statusMessage = "フォルダが選択されていません。";
  //     }
  // }
</script>

{#if isLocaleReady && isConfigReady}
  <!-- レイアウト -->
  <div class="container">
    <!-- 上部メニュー -->
    <header class="top-menu">
      <nav>
        <ul>
          <li><a href="/" class="menu-link">{$t('layout.menu.home')}</a></li>
          <li>
            <a href="/settings" class="menu-link"
              >{$t('layout.menu.settings')}</a
            >
          </li>
          <li>
            <a href="/about" class="menu-link">{$t('layout.menu.about')}</a>
          </li>
        </ul>
      </nav>
    </header>

    <div
      class="main-content"
      style={`grid-template-columns: ${leftPaneWidth}px 5px 1fr;`}
    >
      <!-- 左ペイン -->
      <aside class="left-pane">
        {#if $currentComponent}
          <svelte:component this={$currentComponent} {thumbnailKeys} />
        {:else}
          <p>{$t('layout.left_pane.no_content')}</p>
        {/if}
      </aside>

      <!-- リサイズハンドル -->
      <div class="resizer" on:mousedown={startResize}></div>

      <!-- 右ペイン（ページ固有の内容） -->
      <section class="right-pane">
        <slot />
      </section>
    </div>

    <!-- ステータスバー -->
    <footer class="status-bar {status.type}">
      <p>{status.message || $t('layout.status_bar.default_message')}</p>
    </footer>
  </div>
{/if}

<style>
  /* 全体のレイアウト */
  .container {
    display: grid;
    grid-template-rows: auto 1fr auto; /* 上部メニュー, メインコンテンツ, ステータスバー */
    height: 100vh;
    overflow: hidden;
  }

  /* 上部メニュー */
  .top-menu {
    grid-column: 1 / -1;
    background-color: #333;
    color: white;
    padding: 0.5rem 1rem;
  }

  .top-menu ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    gap: 1rem;
  }

  .menu-link {
    color: white;
    text-decoration: none;
    font-size: 16px;
  }

  .menu-link:hover {
    color: #00d4ff;
  }

  /* メインエリア */
  .main-content {
    display: grid;
    grid-template-columns: 250px 5px 1fr;
    overflow: hidden;
  }

  /* 左ペイン */
  .left-pane {
    /*background: #f8f9fa;*/
    background: #eff0f0;
    border-right: 1px solid #ddd;
    padding: 1rem;
    overflow-y: auto;
    min-width: 150px; /* 最小幅 */
    max-width: 600px; /* 最大幅 */
  }

  .left-pane h2 {
    margin-top: 0;
  }

  .left-pane p {
    margin: 0.5rem 0;
    color: #333;
    font-size: 14px;
  }

  /* リサイズハンドル */
  .resizer {
    background-color: #ddd;
    cursor: ew-resize;
    width: 5px;
    user-select: none;
  }

  .resizer:hover {
    background-color: #bbb;
  }

  /* 右ペイン */
  .right-pane {
    padding: 1rem;
    overflow-y: auto;
  }

  /* ステータスバー */
  .status-bar {
    grid-row: 3;
    grid-column: 1 / -1; /* 横幅全体に表示 */
    height: 2rem;
    background-color: #333; /* デフォルトの色 */
    color: white;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 1rem;
    font-size: 14px;
    /*transition: background-color 1s ease; !* 色変更のトランジション *!*/
  }

  .status-bar p {
    margin: 0;
  }

  /* 状態ごとの色定義 */
  .status-bar.info {
    background-color: #333; /* デフォルト色 */
  }

  .status-bar.success {
    background-color: #4caf50;
    animation: revert-color 3s ease-in-out forwards; /* アニメーションを適用 */
  }

  .status-bar.error {
    background-color: #f44336;
    animation: revert-color 3s ease-in-out forwards; /* アニメーションを適用 */
  }

  /* アニメーション定義 */
  @keyframes revert-color {
    100% {
      background-color: #333; /* 元の色に戻す */
    }
  }
</style>
