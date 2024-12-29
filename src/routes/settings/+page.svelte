<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog'
  import {
    addFolder,
    getAllFolders,
    deleteFolder,
    deleteIgnoreFolder,
    getAllIgnoreFolders,
    addIgnoreFolder,
    getConfig,
    setConfig,
  } from '$lib/api'
  import Layout from '../../lib/components/Layout.svelte'
  import { onMount } from 'svelte'
  import { startScan, statusStore } from '../../statusStore'
  import { configStore } from '../../stores' // 共通レイアウトをインポート
  import type { Config } from '$lib/config'
  import { t, register, init, locale, waitLocale } from 'svelte-i18n'
  import { getHeadings, headingStore } from '$lib/headings'

  // settings 用の翻訳データを登録

  // init({
  //   fallbackLocale: 'en',
  //   initialLocale: 'ja' // 必要に応じて、ブラウザから判定する部分も導入可能
  // });
  let isLocaleReady = false
  function updateLanguage(lang: string) {
    locale.set(lang) // ロケールを更新
    saveConfig()
  }
  async function initializeI18n() {
    register('en', () => import('$lib/locale/settings/en.json'))
    register('ja', () => import('$lib/locale/settings/ja.json'))
    init({
      fallbackLocale: 'en',
      initialLocale: 'ja',
    })
    await waitLocale() // 初期化が完了するまで待機
    isLocaleReady = true
  }

  $: headingStore.subscribe((value) => {
    headings = value
  })
  let headings: Array<{ id: string; text: string; level: number }> = []

  let selectedFolder: string = ''
  let folders: { id: number; path: string }[] = []

  let status = {
    message: '',
    progress: null as null | number,
    type: 'info',
    isVisible: true,
  }
  $: statusStore.subscribe((value) => (status = value))

  let config: Config
  $: configStore.subscribe((value) => {
    config = value
  })

  async function loadFolders() {
    try {
      folders = await getAllFolders() // フォルダを取得
    } catch (error) {
      statusStore.set({
        message: `${$t('errorLoadingFolders')} ${(error as Error).message}`,
        progress: null,
        type: 'error',
        isVisible: true,
      })
      console.error($t('errorLoadingFolders'), error)
    }
  }

  async function selectFolder() {
    selectedFolder = (await open({
      directory: true,
      multiple: false,
      title: $t('selectThumbnailFolder'), // 翻訳を適用
    })) as string
  }

  async function saveAndImportFolder() {
    if (selectedFolder) {
      try {
        await addFolder(selectedFolder)
        await startScan([selectedFolder]) // スキャンとインポートの開始
        await loadFolders()
        selectedFolder = ''
      } catch (error) {
        statusStore.set({
          message: `${$t('errorAddingFolder')} ${(error as Error).message}`,
          progress: null,
          type: 'error',
          isVisible: true,
        })
        console.error($t('errorAddingFolder'), error)
      }
    }
  }
  let progress = 0
  export let statusMessage: string

  async function deleteSelectedFolder(id: number) {
    try {
      await deleteFolder(id)
      await loadFolders()
    } catch (error) {
      statusStore.set({
        message: `${$t('errorDeletingFolder')} ${(error as Error).message}`,
        progress: null,
        type: 'error',
        isVisible: true,
      })
      console.error($t('errorDeletingFolder'), error)
    }
  }

  async function loadIgnoreFolders() {
    try {
      ignoreFolders = await getAllIgnoreFolders() // 無視対象フォルダを取得
    } catch (error) {
      statusStore.set({
        message: `${$t('errorLoadingIgnoreFolders')} ${(error as Error).message}`,
        progress: null,
        type: 'error',
        isVisible: true,
      })
      console.error($t('errorLoadingIgnoreFolders'), error)
    }
  }

  async function deleteSelectedIgnoreFolder(id: number) {
    try {
      await deleteIgnoreFolder(id)
      await loadIgnoreFolders()
    } catch (error) {
      statusStore.set({
        message: `${$t('errorDeletingIgnoreFolder')} ${(error as Error).message}`,
        progress: null,
        type: 'error',
        isVisible: true,
      })
      console.error($t('errorDeletingIgnoreFolder'), error)
    }
  }
  async function selectIgnoreFolder() {
    selectedIgnoreFolder = (await open({
      directory: true,
      multiple: false,
      title: $t('selectIgnoreFolder'), // 翻訳を適用
    })) as string
  }

  async function saveAndImportIgnoreFolder() {
    if (selectedIgnoreFolder) {
      try {
        await addIgnoreFolder(selectedIgnoreFolder)
        await loadIgnoreFolders()
        selectedIgnoreFolder = ''
      } catch (error) {
        statusStore.set({
          message: `${$t('errorAddingIgnoreFolder')} ${(error as Error).message}`,
          progress: null,
          type: 'error',
          isVisible: true,
        })
        console.error($t('errorAddingIgnoreFolder'), error)
      }
    }
  }
  let isConfigReady: boolean = false

  onMount(async () => {
    await initializeI18n() // 初期化
    config = await getConfig()
    isConfigReady = true
    await loadFolders()
    await loadIgnoreFolders()
  })

  async function saveConfig() {
    if (config) {
      configStore.set(config) // ストアを更新
      statusStore.set({
        message: $t('configSaved'), // 設定が保存された旨を表示
        progress: null,
        type: 'info',
        isVisible: true,
      })
      await waitLocale() // 初期化が完了するまで待機
      headings = getHeadings()
      headingStore.set(headings)
      await setConfig(config)
    }
  }

  let selectedIgnoreFolder: string = ''
  let ignoreFolders: { id: number; path: string }[] = []
</script>

<Layout
  activePage="settings"
  {statusMessage}
  thumbnailKeys={[]}
  thumbnailProcessed={false}
>
  <!-- 固有のコンテンツ部分 -->
  {#if isLocaleReady && isConfigReady}
    <h1 id="settings">{$t('settings')}</h1>

    <!-- フォルダ一覧 -->
    <h2>{$t('registeredFolders')}</h2>
    {#if folders.length > 0}
      <ul>
        {#each folders as folder (folder.id)}
          <li>
            <span class="path">{folder.path}</span>
            <button
              class="update-button"
              on:click={() => startScan([folder.path])}>{$t('update')}</button
            >
            <button
              class="delete-button"
              on:click={() => deleteSelectedFolder(folder.id)}
              >{$t('delete')}</button
            >
          </li>
        {/each}
      </ul>
    {:else}
      <p>{$t('noFolders')}</p>
    {/if}
    <div class="select-folder">
      <p>{$t('selectThumbnailFolder')}</p>
      <button on:click={selectFolder}>{$t('selectFolder')}</button>
    </div>

    {#if selectedFolder}
      <p>{$t('selectedFolder')}: {selectedFolder}</p>
      <button on:click={saveAndImportFolder}>{$t('saveAndImport')}</button>
    {/if}

    <hr />

    <h2>{$t('ignoredFolders')}</h2>
    {#if ignoreFolders.length > 0}
      <ul>
        {#each ignoreFolders as folder (folder.id)}
          <li>
            <span class="path">{folder.path}</span>
            <button
              class="delete-button"
              on:click={() => deleteSelectedIgnoreFolder(folder.id)}
              >{$t('delete')}</button
            >
          </li>
        {/each}
      </ul>
    {:else}
      <p>{$t('noFolders')}</p>
    {/if}
    <div class="select-ignore-folder">
      <p>{$t('selectIgnoreFolder')}</p>
      <button on:click={selectIgnoreFolder}>{$t('selectFolder')}</button>
    </div>

    {#if selectedIgnoreFolder}
      <p>{$t('selectedFolder')}: {selectedIgnoreFolder}</p>
      <button on:click={saveAndImportIgnoreFolder}
        >{$t('saveIgnoreFolder')}</button
      >
    {/if}
    <hr />
    <div class="updatedb-settings">
      <h3>{$t('updateDatabaseOnStartup')}</h3>
      <label>
        <input
          type="checkbox"
          bind:checked={config.feature_flags.update_db_when_startup}
          on:change={saveConfig}
        />
        {$t('updateDatabaseOnStartupLabel')}
      </label>
    </div>
    <div class="language-setting">
      <!-- 使用言語設定 -->
      <h3>{$t('language')}</h3>
      <select
        bind:value={config.feature_flags.language}
        on:change={() => updateLanguage(config.feature_flags.language)}
      >
        <option value="ja">{$t('languageOptionJa')}</option>
        <option value="en">{$t('languageOptionEn')}</option>
      </select>
    </div>
  {/if}
</Layout>

<style>
  ul {
    list-style: none;
    padding: 0;
    width: 80%;
    margin: auto;
    border: 1px solid #aaa;
  }

  ul li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
  }

  ul li:not(:last-child) {
    border-bottom: 1px solid #aaa;
  }

  ul li span {
    flex: 1;
    word-wrap: break-word;
  }

  ul li button {
    margin-left: 1rem;
    flex-shrink: 0;
  }

  button {
    cursor: pointer;
    padding: 0.5rem 1rem;
    background-color: #0070f3;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 0.8rem;
    margin: 1rem;
  }
  .delete-button {
    background-color: #ff3d3d !important;
    font-size: 0.8rem;
  }
  .delete-button:hover {
    background-color: #ff2300 !important;
  }
  .update-button {
    background-color: #0070f3 !important;
  }
  .update-button:hover {
    background-color: #0056b3 !important;
  }

  button:hover {
    background-color: #0056b3;
  }

  .select-folder,
  .select-ignore-folder,
  .language-setting {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 1rem auto;
  }
  .select-folder,
  .select-ignore-folder {
    width: 80%;
  }
  .language-setting select {
    padding: 0.25rem;
  }
  .language-setting {
    width: 90%;
    margin-left: 0;
  }
  .language-setting h3 {
    margin-block-start: 0;
  }
  .updatedb-settings {
    margin: 2rem auto;
  }
  .path {
    word-break: break-all;
    word-wrap: break-word;
    white-space: pre-wrap;
    font-size: 0.9rem;
    margin-left: 1rem;
  }
</style>
