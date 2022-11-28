<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'
    import { mdiWindowClose, mdiWindowMinimize, mdiWindowMaximize, mdiChevronRight, mdiChevronLeft, mdiMagnify } from '@mdi/js'
    import Icon from './utils/Icon.svelte'
    import Breadcrumbs from './Breadcrumbs.svelte'
    import { settings, vfs } from 'src/store'
    import { emit } from '@tauri-apps/api/event'
    import { serializeFile } from 'src/api/object'
    import { WindowType } from 'src/settings'

    let selectedFiles: number[] = $vfs.selectedFilesIndices
    let updateFilesAndFolders = () => {
        selectedFiles = $vfs.selectedFilesIndices
    }

    $vfs.onSelectionChange(updateFilesAndFolders)

    let showSearch = false

    const onClose = () => {
        console.log($vfs.selectedFiles)
        emit('closeDialog', JSON.stringify({
            selectedFiles: $vfs.selectedFiles.map(file => serializeFile(file)),
            selectedProvider: $vfs.activeProvider
        }))
        appWindow.close()
    }
</script>
<div data-tauri-drag-region class="w-full navbar bg-base-300 flex justify-between">
    <div class="btn-group">
        <button class="btn btn-square" on:click={() => {$vfs.goBack()}}>
            <Icon icon={mdiChevronLeft} size={16} color="hsla(var(--bc) / var(--tw-text-opacity, 1))" />
        </button>
        <button class="btn btn-square" on:click={() => {$vfs.goForward()}}>
            <Icon icon={mdiChevronRight} size={16} color="hsla(var(--bc) / var(--tw-text-opacity, 1))" />
        </button>
    </div>
    <div class="card bg-base-100 shadow-x breadcrumbs-container">
        <div class="input-group">
            {#if showSearch}
            <input type="text" placeholder="Search…" class="input" autofocus />
            {:else}
            <Breadcrumbs />
            {/if}
            <!-- <button class="btn btn-square" on:click={() => { showSearch = !showSearch }}>
                <Icon icon={mdiMagnify} size={20} color="hsla(var(--bc) / var(--tw-text-opacity, 1))" />
            </button> -->
        </div>
    </div>
    {#if $settings.windowType == WindowType.main }
        <div class="flex-none">
            <button class="btn btn-circle btn-sm mx-1" on:click={() => appWindow.minimize()}>
                <Icon icon={mdiWindowMinimize} size={16} color="hsla(var(--bc) / var(--tw-text-opacity, 1))" />
            </button>
            <button class="btn btn-circle btn-sm mx-1" on:click={() => appWindow.toggleMaximize()}>
                <Icon icon={mdiWindowMaximize} size={16} color="hsla(var(--bc) / var(--tw-text-opacity, 1))" />
            </button>
            <button class="btn btn-circle btn-sm mx-1" on:click={() => {emit('closeDialog', {selectedFiles: $vfs.selectedFiles.map(file => serializeFile(file))}); appWindow.close()}}>
                <Icon icon={mdiWindowClose} size={16} color="hsla(var(--bc) / var(--tw-text-opacity, 1))" />
            </button>
        </div>
    {:else }
        <div class="flex-none">
            <button class="btn mx-1" on:click={() => {emit('closeDialog', {selectedFiles: []}); appWindow.close()}}>
                Cancel
            </button>
            <button class="btn mx-1 btn-primary" disabled={selectedFiles.length == 0} on:click={() => onClose()}>
                Confirm
            </button>
        </div>
    {/if}
</div>

<style>
    .breadcrumbs-container {
        width: 50%;
    }

    .breadcrumbs-container .input-group input {
        flex-grow: 1;
        text-align: center;
    }
</style>