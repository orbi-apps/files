<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import Icon from "./utils/Icon.svelte"
    import { mdiContentSave, mdiFolder, mdiFile } from "@mdi/js"
    import type { GenericObject } from "src/api/object"
    import { formatRelative, isWithinInterval, addHours, format, isToday, isYesterday, isThisYear } from 'date-fns'

    export let rename: boolean
    export let active: boolean
    export let file: GenericObject

    $: icon = file.id.mimeType.isFolder() ? mdiFolder : mdiFile

    const dispatch = createEventDispatcher()

    let newName = ""

    const resetNewName = (file: GenericObject) => {
        newName = file.name
    }
    $: resetNewName(file)

    const submit = (event) => {
        event.preventDefault()
        dispatch('rename', { oldName: file.name, newName })
    }

    const formatFileSize = (bytes: number) => {
        if(bytes == 0) return '0 Bytes'
        const k = 1000
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']
        const i = Math.floor(Math.log(bytes) / Math.log(k))
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
    }

    const formatDate = (date: Date) => {
        if (!date) return ""
        const shortInterval = {
            start: date,
            end: addHours(date, 1)
        }
        if (isWithinInterval(Date.now(), shortInterval)) return formatRelative(date, Date.now())
        if (isToday(date)) return format(date, "H:mm")
        if (isYesterday(date)) return formatRelative(date, Date.now())
        if (isThisYear(date)) return format(date, "d MMM")
        return format(date, "d MMM y")
    }
</script>

<tr
    class="hover"
    class:active={active}
    on:click
    on:dblclick
    on:contextmenu>
    <td class="file-icon"><Icon icon={icon} color="#fff" /></td>
    <td>
    {#if rename}
        <form class="form-control" on:submit={(event) => submit(event)}>
            <div class="input-group">
                <input class="input" autofocus bind:value={newName} />
                <button class="btn" type="submit"><Icon icon={mdiContentSave} /></button>
            </div>
        </form>
    {:else}
        {file.name}
    {/if}
    </td>
    <td>{formatFileSize(file.size) || ""}</td>
    <td>{formatDate(file.modified_at) || ""}</td>
</tr>