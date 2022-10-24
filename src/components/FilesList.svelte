<script lang="ts">
    import { clickOutside } from './utils/clickOutside'
    import FileRow from './FileRow.svelte'
    import BatchRenameDialog from './dialogs/BatchRenameDialog.svelte'
    import type { GenericObject } from 'src/api/object'
    import { createEventDispatcher } from 'svelte'
  
    export let files: GenericObject[]
    export let selectedFiles: number[]
    export let rename = false

    const dispatch = createEventDispatcher();

    let openContextMenu = false

    $: inlineRename = rename && selectedFiles.length <= 1
    $: openRenameDialog = rename && selectedFiles.length > 1

    let lastSelectedIndex = 0

    export let newFile: GenericObject | undefined

    const handleSelection = (index: number, event: MouseEvent) => {
        let selectedIndices = [index]
        let replaceSelection = !event.ctrlKey
        
        if (event.shiftKey) {
            selectedIndices = Array.from(
                {length: Math.max(lastSelectedIndex, index) - Math.min(lastSelectedIndex, index) + 1},
                (_, i) => Math.min(lastSelectedIndex, index) + i)
        }

        lastSelectedIndex = index
        dispatch("select", {selectedIndices, replaceSelection})
    }

    const handleContextMenu = (event: MouseEvent, index: number | undefined) => {
        event.preventDefault()
        dispatch("contextMenu", index)
    }

    const onConfirmInlineRename = (event: CustomEvent<any>) => {
        dispatch("rename", {from: event.detail.oldName, to: event.detail.newName})
    }

    const onConfirmBatchRename = (event: CustomEvent<any>) => {
        dispatch("rename", {from: event.detail.from, to: event.detail.to})
    }

    const onClickOutside = () => {
        dispatch("clickOutside", {selectedIndices: [], replaceSelection: true})
    }

    const onConfirmCreateFile = (event: CustomEvent<any>) => {
        newFile.name = event.detail.newName
        dispatch("create", newFile)
        newFile = undefined
    }

    const onOpen = (file: GenericObject) => {
        dispatch("open", file)
    }
</script>

<BatchRenameDialog bind:open={openRenameDialog} on:confirm={onConfirmBatchRename} />
  
<div class="overflow-x-auto table-auto disable-select h-full">
    <table class="table w-full overflow-y-scroll" use:clickOutside={{enabled: !openContextMenu, cb: onClickOutside}}>
        <thead>
            <tr>
                <th class="file-icon"></th>
                <th>Name</th>
            </tr>
        </thead>
        <tbody>
            {#each files as file, index}
            <FileRow
                {file}
                active={selectedFiles.includes(index)}
                rename={inlineRename && selectedFiles[0] === index}
                on:rename={onConfirmInlineRename}
                on:click={(event) => handleSelection(index, event)}
                on:dblclick={() => onOpen(file)}
                on:contextmenu={(event) => handleContextMenu(event, index)} />
            {/each}
            {#if newFile}
            <FileRow
                file={newFile}
                active={false}
                rename={true}
                on:rename={onConfirmCreateFile}
                on:click={() => {}}
                on:dblclick={() => {}}
                on:contextmenu={() => {}} />
            {/if}
        </tbody>
    </table>
</div>

<style>
    .file-icon {
        width:1px;
    }

    tbody {
        overflow-y: scroll;
    }

    th {
        position: sticky;
        top: 0;
        border-top-left-radius: 0;
        border-top-right-radius: 0;
    }
</style>