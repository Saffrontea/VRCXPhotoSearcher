// src/stores/statusStore.ts
import { writable } from 'svelte/store'
import { scanAndRegisterImagesWithProgress } from '$lib/api'
import { t, register, init, locale, waitLocale } from 'svelte-i18n'

export const statusStore = writable({
  message: '', // ステータスメッセージ
  progress: null as number | null, // null: 進捗バー非表示, 数値: 進捗率 (例: 50%)
  type: 'info', // ステータスの種類 (info/success/error)
  isVisible: true, // ステータスバーの表示フラグ切替
})

export async function startScan(folderList: string[]) {
  try {
    // スキャン開始時の状態をセット
    statusStore.set({
      message: 'スキャンを開始します...',
      progress: 0,
      type: 'info',
      isVisible: true,
    })

    // スキャン進行中の状態を更新
    await scanAndRegisterImagesWithProgress((newProgress, newMessage) => {
      // 状態を動的に更新
      statusStore.update((state) => ({
        ...state,
        message: newMessage,
        progress: newProgress,
      }))
    }, folderList)

    // スキャン完了時
    statusStore.set({
      message: 'スキャンが完了しました！',
      progress: null, // 進捗バーを非表示
      type: 'success',
      isVisible: true,
    })
  } catch (error) {
    console.error('スキャン中のエラー:', error)

    // エラー発生時
    statusStore.set({
      message: 'スキャン中にエラーが発生しました。',
      progress: null, // 進捗バーを非表示
      type: 'error',
      isVisible: true,
    })
  }
}
