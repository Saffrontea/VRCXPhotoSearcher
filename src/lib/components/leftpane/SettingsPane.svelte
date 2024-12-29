<script lang="ts">
  import { onMount } from 'svelte'
  import { configStore } from '../../../stores'
  import type { Config } from '$lib/config'
  import { init, locale, register, waitLocale, t } from 'svelte-i18n'
  import { getConfig } from '$lib/api'
  import { getHeadings, headingStore } from '$lib/headings'

  $: configStore.subscribe((value) => {
    config = value
  })
  $: headingStore.subscribe((value) => {
    headings = value
  })
  let headings: Array<{ id: string; text: string; level: number }> = []
  let config: Config
  let isLocaleReady: boolean = false
  let isConfigReady: boolean = false
  async function initializeI18n() {
    register('en', () => import('$lib/locale/leftpane/settings/en.json'))
    register('ja', () => import('$lib/locale/leftpane/settings/ja.json'))
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
    await waitLocale() // 初期化が完了するまで待機
    headings = getHeadings()
    headingStore.set(headings)
  })
</script>

{#if isLocaleReady && isConfigReady}
  <!-- 見出しリンク -->
  <h2 id="leftpanel-settings">
    <a href="#settings" style="color:inherit; text-decoration: none;"
      >{$t('settingsPane.title')}</a
    >
  </h2>

  {#if headings.length > 0}
    <ul class="headings-list">
      {#each headings as heading}
        <li
          class="heading-item"
          class:heading-h2={heading.level === 2}
          class:heading-h3={heading.level === 3}
        >
          <a href={`#${heading.id}`} class="heading-link">{heading.text}</a>
        </li>
      {/each}
    </ul>
  {:else}
    <p></p>
  {/if}
{/if}

<style>
  .headings-list {
    list-style: none;
    padding: 0;
  }

  .heading-item {
    margin-bottom: 1rem;
  }

  .heading-h2 .heading-link {
    font-size: 1rem;
  }

  :global(.heading-h3 .heading-link) {
    font-size: clamp(0.9rem, 1%, 1rem);
    width: 100%;
    display: block;
  }

  .heading-link {
    text-decoration: none;
    color: #444;
    font-weight: 600;
  }

  .heading-link:hover {
    text-decoration: underline;
    color: #333;
  }
</style>
