<script lang="ts">
  import type { Config } from '$lib/config'
  import { init, locale, register, waitLocale,t } from 'svelte-i18n'
  import { configStore } from '../../stores'
  import { onMount } from 'svelte'
  import { getConfig } from '$lib/api'
  import { resolveResource } from '@tauri-apps/api/path';
  import { readTextFile } from '@tauri-apps/plugin-fs';
  import Layout from '$lib/components/Layout.svelte'
  import { statusStore } from '../../statusStore'
  import { openUrl } from '@tauri-apps/plugin-opener'


  let config: Config
  let isLocaleReady = false
  let isConfigReady = false
  let statusMessage: string = '';
  let isVisible = false;
  let status = {
    message: '',
    progress: null as number | null,
    type: 'info',
    isVisible: true,
  }
  $: statusStore.subscribe((value) => (status = value))
  $: configStore.subscribe((value) => {
    config = value
  })

  let fossaContent: string;
  let icon;

  // i18n設定
  async function initializeI18n() {
    register('en', () => import('$lib/locale/about/en.json'))
    register('ja', () => import('$lib/locale/about/ja.json'))
    init({
      fallbackLocale: 'en',
      initialLocale: 'ja',
    })
    await waitLocale()
    isLocaleReady = true
  }

  onMount(async () => {
    await initializeI18n()
    config = await getConfig()
    isConfigReady = true
    locale.set(config.feature_flags.language)
    await waitLocale()
    fossaContent = await readTextFile(await resolveResource('resources/FOSSA_REPORT.txt'));
  })


  function openGithub() {
    openUrl("https://github.com/Saffrontea");
  }
</script>

<Layout
  activePage="about"
  {statusMessage}
  thumbnailKeys={[]}
  thumbnailProcessed={false}
>
  {#if isLocaleReady && isConfigReady}
    <!-- アプリ概要 -->
     <div class="about-container">

    <h1 id="about">{$t('about')}</h1>

    <!-- アイコン表示 -->
    <div class="icon-container">
      <img src="/icon.png" alt="App Icon" width="100" height="100" />
    </div>
    <p>{$t('app_description')}</p>

    <hr />

    <!-- アプリ機能 -->
    <h2>{$t('features')}</h2>
    <ul>
      <li>{$t('feature_metadata_analysis')}</li>
      <li>{$t('feature_search')}</li>
      <li>{$t('feature_ui')}</li>
      <li>{$t('feature_folder_analysis')}</li>
      <li>{$t('feature_fast_safe')}</li>
    </ul>

    <hr />

    <!-- Changelog セクション（非表示中） -->
    <div class="changelog">
      <h2>{$t('changelog')}</h2>
      <p>{$t('changelog_placeholder')}</p>
    </div>




    <!-- アプリ情報 -->
    <h2>{$t('app_info')}</h2>
    <p><strong>{$t('version')}:</strong> 1.0.0</p>
    <p><strong>{$t('author')}:</strong> Saffrontea</p>
    <p><strong>{$t('contact')}:</strong> <a href="#" on:click={openGithub}>Github</a></p>
  </div>



    <!-- FOSSA ライセンス情報 -->
     <div class="license-container">
      <h2>Licenses</h2>
      <span class="toggle-button" on:click={() => { isVisible = !isVisible; }} class:open={isVisible}>
      </span>
     </div>
    
    {#if isVisible}

    <div class="fossa-container">
      <pre>{fossaContent}</pre> <!-- FOSSAファイル内容を埋め込み -->
    </div>
    {/if}
  {/if}
</Layout>

<style>
    .icon-container {
        display: flex;
        justify-content: center;
        margin-bottom: 20px;
    }

    .icon-container img {
        border-radius: 12px; /* アイコンに丸みを追加 */
    }

    h1, h2 {
        text-align: center;
    }

    ul {
        list-style-position: inside;
        padding-left: 0;
    }

    .changelog {
        display: none; /* Changelog セクションを非表示に設定 */
    }

    .fossa-container{
      height: 40vh;
      overflow: scroll;
      background-color: white;
      border: 1px solid #333;
      border-radius: 0.1rem;
      padding: 1rem;
    }
    .fossa-container pre {
      font-size: 0.7rem;
      white-space: pre-wrap;
    }

    .toggle-button {
    display: block;
    font-size: 0.8rem;
    text-align: center;
    line-height: 28px;
    cursor: pointer;
    transition: transform 0.2s ease, background-color 0.3s ease;
    user-select: none;
    margin: 5px;
    color:#222;
    padding-top: 0.34rem;
  }


  /* 展開時の矢印を変更 */
  .toggle-button::before {
    content: '◀';  /* 初期状態では右向き矢印 */
    transition: transform 0.2s ease;
  }

  /* 展開された場合 */
  .toggle-button.open::before {
    content: '▼';  /* 展開後は下向き矢印 */
    transform: rotate(90deg);  /* 回転させる */
  }

  .license-container{
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
  }

  .about-container{
    padding: 0 20%;
  }

</style>