import { writable } from 'svelte/store'
import { VirtualFS } from './api/virtualfs'
import { WindowType, type Settings } from './settings'

export const vfs = writable<VirtualFS>(new VirtualFS())

export const settings = writable<Settings>({
    windowType: WindowType.main
})