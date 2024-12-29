import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Config } from '$lib/config'

// 初期設定済みかどうかを確認
export async function getInitialSetupState(): Promise<boolean> {
  const folders = await invoke<string[]>('get_all_folders')
  console.log(folders)
  return folders.length > 0
}

// フォルダを追加
export async function addFolder(path: string): Promise<void> {
  await invoke('add_folder', { path })
}

// // 画像をスキャンしてデータベースに登録
// export async function scanAndRegisterImages(): Promise<void> {
//   await invoke('scan_and_register_images');
// }

export async function scanAndRegisterImagesWithProgress(
  eventCallback: (progress: number, message: string) => void,
  folderList: Array<string>
): Promise<void> {
  return new Promise((resolve, reject) => {
    const event_id = Date.now().toString() // 一意のイベントIDを生成

    // Rust 側からのイベントをリッスン
    const unlisten = listen('scan_progress', (event) => {
      // イベントIDが一致する場合のみ処理
      const payload = event.payload as {
        event_id: string
        progress: number
        message: string
      }
      if (payload.event_id === event_id) {
        eventCallback(payload.progress, payload.message)
      }
    })

    // Rust 側でスキャンを開始
    invoke('scan_and_register_images_with_progress', {
      folderList: folderList,
      eventId: event_id,
    })
      .then(() => {
        unlisten.then((unimpl) => unimpl()) // イベントのリスナー解除
        resolve() // 処理完了
      })
      .catch((error) => {
        unlisten.then((unimpl) => unimpl()) // イベントのリスナー解除
        reject(error) // エラー処理
      })
  })
}

// // サムネイルリストを取得
// export async function getAllThumbnails(): Promise<any> {
//   const path = await invoke<string[]>('search_files_in_folders');
//   const thumbnails = await invoke<any>('generate_and_get_thumbnails',{filePaths: path});
//   return thumbnails;
// }

export async function getAllFolders(): Promise<{ id: number; path: string }[]> {
  return await invoke<{ id: number; path: string }[]>('get_all_folders')
}

export async function getAllIgnoreFolders(): Promise<
  { id: number; path: string }[]
> {
  return await invoke<{ id: number; path: string }[]>('get_all_ignore_folders')
}

export async function deleteFolder(id: number): Promise<void> {
  await invoke('delete_folder', { id })
}

export async function deleteIgnoreFolder(id: number): Promise<void> {
  await invoke('delete_ignore_folder', { id })
}

export async function addIgnoreFolder(path: string): Promise<void> {
  await invoke('add_ignore_folder', { path })
}

export async function getThumbnailsChunk(
  offset: number,
  limit: number
): Promise<any[]> {
  const paths = await invoke<[{ path: string; uuid: string }]>(
    'search_files_in_folders'
  ) // 全ファイル取得
  const chunk = paths.slice(offset, offset + limit) // 指定範囲のファイルパスを計算
  // サムネイル生成と取得
  return await invoke<any>('generate_and_get_thumbnails', { filePaths: chunk }) // 結果を返す
}

export async function getMetadata(
  dbid: string,
  filePath: string
): Promise<any> {
  const response = await invoke<{
    metadata: any
    data_url: string
    file_created_at: string
  }>('get_image_metadata', { uuid: dbid, filePath: filePath })

  return {
    metadata: response.metadata,
    data_url: response.data_url,
    file_created_at: response.file_created_at,
  }
}

export async function getConfig(): Promise<Config> {
  return await invoke<Config>('get_config')
}
export async function setConfig(config: Object): Promise<void> {
  await invoke('set_config', { config })
}

export async function searchImage(
  conditions: Array<any>
): Promise<Array<Object>> {
  return await invoke('search_images', { conditions })
}
