import { writable } from 'svelte/store'

// 見出しタグ(h2, h3)リスト
export const headingStore = writable(
  [] as { id: string; text: string; level: number }[]
)
// +page.svelte から見出しを動的に取得
export function getHeadings(): Array<{
  id: string
  text: string
  level: number
}> {
  // ページ内のすべての h2, h3 タグを取得
  const headingElements = document.querySelectorAll('h2, h3')
  return Array.from(headingElements)
    .filter((elem) => {
      return !(elem.id == 'leftpanel-settings')
    })
    .map((elem, index) => {
      // 各見出しに ID を付加（必要に応じて既存のIDを保持）
      const id = elem.id || `heading-${index}`
      elem.id = id // IDを設定

      return {
        id,
        text: elem.textContent || '', // 見出しのテキスト内容
        level: elem.tagName === 'H2' ? 3 : 3, // 見出しレベル
      }
    })
}
