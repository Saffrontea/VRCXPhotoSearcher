import { writable } from 'svelte/store'
import type { Config } from '$lib/config'

// 初期化が完了したかどうかを示すフラグ
export const updateDBWhenInit = writable(false)
export const configStore = writable(Object() as Config)

export const thumbnailStore = writable([] as any[])
