<script lang="ts">
    import { vfs } from '../store'
    import ContextMenu from './utils/ContextMenu.svelte'
    import { MimeType, ObjectId, type GenericObject } from 'src/api/object'
    import type { FileTemplate } from './utils/files.templates'
    import FilesList from './FilesList.svelte'
    import { UserAttentionType, WebviewWindow } from '@tauri-apps/api/window'
  
    let files = $vfs.files
    let selectedFiles: number[] = $vfs.selectedFilesIndices
    let updateFilesAndFolders = () => {
        files = $vfs.files
        selectedFiles = $vfs.selectedFilesIndices
    }

    $vfs.register(updateFilesAndFolders)
    $vfs.onSelectionChange(updateFilesAndFolders)

    let rename = false

    let waitingForUser = false

    let openContextMenu = false
    let position = {x: 0, y: 0}

    let newFile: GenericObject | undefined

    const handleSelection = (event: { replaceSelection: boolean; selectedIndices: number[]; }) => {
        if (event.replaceSelection) $vfs.clearSelection()
        for (const index of event.selectedIndices) {
            $vfs.selectFile(index)
        }
    }

    const onClickOutside = () => {
        rename = false
        $vfs.unlockSelection()
        $vfs.clearSelection()
    }

    const handleContextMenuOnFile = (event: CustomEvent) => {
        const index = event.detail
        if (index !== undefined && !$vfs.selectedFilesIndices.find((x) => x == index)) {
            handleSelection({selectedIndices: [index], replaceSelection: true})
        }
    }

    const handleContextMenu = (event: MouseEvent) => {
        event.preventDefault()
        openContextMenu = true
        position = {x: event.x - 10, y: event.y - 10}
    }

    const onStartRename = () => {
        $vfs.lockSelection()
        openContextMenu = false
        rename = true
    }

    const onConfirmRename = (event: CustomEvent<any>) => {
        rename = false
        $vfs.rename(event.detail.from, event.detail.to)
        $vfs.unlockSelection()
    }

    const onConfirmCreateFile = (event: CustomEvent<any>) => {
        $vfs.create(event.detail)
        newFile = undefined
    }

    const createFile = (event: CustomEvent<FileTemplate>) => {
        $vfs.unlockSelection()
        openContextMenu = false
        newFile = {
            id: new ObjectId("", new MimeType(event.detail.mimeType)),
            name: (event.detail.prefix || "") + (event.detail.suffix || "")
        }
    }

    const onOpen = (event: CustomEvent<any>) => {
        const file = event.detail
        if (file.id.mimeType.isFolder()) {
            $vfs.push(file.id, file.name)
        } else {
            $vfs.open()
        }
    }

    let moveWindow: WebviewWindow;

    const onMoveFile = async () => {
        $vfs.lockSelection()
        waitingForUser = true
        moveWindow = new WebviewWindow("moveWindow", {
            alwaysOnTop: true,
            decorations: false,
            fullscreen: false,
            height: 600,
            resizable: true,
            title: "Move File",
            width: 800,
            transparent: true,
            url: "index.html"
        })

        moveWindow.once("ready", () => {
            moveWindow.emit("windowSettings", {
                windowType: "move"
            })
        })

        moveWindow.once("tauri://error", function (e) {
            console.log("error creating window")
        })

        await moveWindow.once("closeDialog", (event) => {
            waitingForUser = false
            const payload = JSON.parse(event.payload as string)
            if (payload.selectedFiles.length > 0) {
                console.log(payload)
                $vfs.move(payload.selectedFiles[0], payload.selectedProvider)
            }
            $vfs.unlockSelection()
        })
    }

    const requestAttentionOnMoveWindow = () => {
        moveWindow?.setFocus()
        moveWindow?.requestUserAttention(UserAttentionType.Informational)
    }
</script>

{#if waitingForUser}
<div class="wait-for-user" on:click={requestAttentionOnMoveWindow}></div>
{/if}

<ContextMenu {position} bind:open={openContextMenu} on:rename={() => onStartRename()} on:create={createFile} on:move={onMoveFile} />
<div class="h-full" on:contextmenu={handleContextMenu}>
    <FilesList
        {files}
        {selectedFiles}
        {rename}
        {newFile}
        on:clickOutside={onClickOutside}
        on:open={onOpen}
        on:create={onConfirmCreateFile}
        on:rename={onConfirmRename}
        on:select={(event) => handleSelection(event.detail)}
        on:contextMenu={handleContextMenuOnFile} />
</div>

<style>
    .wait-for-user {
        background-color: rgba(0,0,0,0.5);
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        z-index: 9999;
    }
</style>